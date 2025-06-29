use chrono::{DateTime, Local};
use clap::Parser;
use std::cmp::Ordering;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

mod args;

fn main() -> anyhow::Result<()> {
    let args = args::LsArgs::parse();
    let path = Path::new(&args.path);

    list_directory(path, &args, 0)?; // Start recursion with depth 0

    Ok(())
}

fn list_directory(path: &Path, args: &args::LsArgs, depth: usize) -> anyhow::Result<()> {
    let mut all_entries: Vec<PathBuf> = Vec::new();

    // Collect all entries from the directory
    for entry_result in fs::read_dir(path)? {
        let entry_path = entry_result?.path();
        all_entries.push(entry_path);
    }

    let mut final_entries: Vec<PathBuf> = Vec::new();

    if args.all {
        final_entries.push(path.join("."));
        final_entries.push(path.join(".."));
        final_entries.extend(all_entries);
    } else {
        for entry_path in all_entries {
            let file_name = entry_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if !file_name.starts_with('.') {
                final_entries.push(entry_path);
            }
        }
    }

    // Custom sorting logic for . and .. and then by name, or by time/size
    final_entries.sort_by(|a, b| {
        let a_name = a
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let b_name = b
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        // Prioritize "."
        if a_name == "." && b_name != "." {
            return Ordering::Less;
        }
        if a_name != "." && b_name == "." {
            return Ordering::Greater;
        }

        // Prioritize ".." after "."
        if a_name == ".." && b_name != ".." && b_name != "." {
            return Ordering::Less;
        }
        if a_name != ".." && b_name == ".." && a_name != "." {
            return Ordering::Greater;
        }

        // Apply other sorting criteria
        if args.sort_time {
            let meta_a = fs::metadata(a).ok();
            let meta_b = fs::metadata(b).ok();
            match (meta_a, meta_b) {
                (Some(ma), Some(mb)) => mb.modified().unwrap().cmp(&ma.modified().unwrap()), // Newest first
                _ => a_name.cmp(b_name),
            }
        } else if args.sort_size {
            let meta_a = fs::metadata(a).ok();
            let meta_b = fs::metadata(b).ok();
            match (meta_a, meta_b) {
                (Some(ma), Some(mb)) => mb.len().cmp(&ma.len()), // Largest first
                _ => a_name.cmp(b_name),
            }
        } else {
            a_name.cmp(b_name) // Default alphabetical sort
        }
    });

    if args.reverse {
        final_entries.reverse();
    }

    let mut total_blocks = 0;
    let mut outputs = Vec::new();

    let mut max_nlink_len = 0;
    let mut max_user_len = 0;
    let mut max_group_len = 0;
    let mut max_size_len = 0;

    if args.long {
        for entry_path in &final_entries {
            let metadata = match fs::metadata(entry_path) {
                Ok(meta) => meta,
                Err(_) => continue,
            };
            total_blocks += metadata.blocks();

            max_nlink_len = max_nlink_len.max(metadata.nlink().to_string().len());
            max_user_len = max_user_len.max(
                get_user_by_uid(metadata.uid())
                    .map(|u| u.name().to_string_lossy().len())
                    .unwrap_or_else(|| metadata.uid().to_string().len()),
            );
            max_group_len = max_group_len.max(
                get_group_by_gid(metadata.gid())
                    .map(|g| g.name().to_string_lossy().len())
                    .unwrap_or_else(|| metadata.gid().to_string().len()),
            );
            max_size_len = max_size_len.max(if args.human_readable {
                bytes_to_human_readable(metadata.len()).len()
            } else {
                metadata.len().to_string().len()
            });
        }
    }

    // Print directory name for recursive listing
    if args.recursive {
        if depth == 0 {
            println!("{}:", path.display());
        } else {
            println!("\n{}:", path.display());
        }
    }

    if args.long {
        if depth == 0 || args.recursive {
            println!("total {}", total_blocks / 2); // Each block is 1024, but counted as 512, so divide by 2
        }
    }

    for entry_path in &final_entries {
        let metadata = match fs::metadata(entry_path) {
            Ok(meta) => meta,
            Err(_) => continue, // Skip files we can't get metadata for
        };

        let file_name = entry_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if args.long {
            let file_type = if metadata.is_dir() {
                'd'
            } else if metadata.is_symlink() {
                'l'
            } else {
                '-'
            };

            let perms = metadata.permissions();
            let mode = perms.mode();
            let rwx = |mode, r_mask, w_mask, x_mask| {
                (
                    if (mode & r_mask) != 0 { 'r' } else { '-' },
                    if (mode & w_mask) != 0 { 'w' } else { '-' },
                    if (mode & x_mask) != 0 { 'x' } else { '-' },
                )
            };

            let (ur, uw, ux) = rwx(mode, 0o400, 0o200, 0o100);
            let (gr, gw, gx) = rwx(mode, 0o040, 0o020, 0o010);
            let (or, ow, ox) = rwx(mode, 0o004, 0o002, 0o001);

            let nlink = metadata.nlink().to_string();

            let user = get_user_by_uid(metadata.uid())
                .map(|u| u.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| metadata.uid().to_string());

            let group = get_group_by_gid(metadata.gid())
                .map(|g| g.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| metadata.gid().to_string());

            let size = if args.human_readable {
                bytes_to_human_readable(metadata.len())
            } else {
                metadata.len().to_string()
            };

            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let time_format =
                if modified.format("%Y").to_string() == Local::now().format("%Y").to_string() {
                    modified.format("%b %e %H:%M").to_string()
                } else {
                    modified.format("%b %e  %Y").to_string()
                };

            let perms_str = format!("{}{}{}{}{}{}{}{}{}", ur, uw, ux, gr, gw, gx, or, ow, ox);

            outputs.push(format!(
                "{}{} {:>width_nlink$} {:<width_user$} {:<width_group$} {:>width_size$} {} {}",
                file_type,
                perms_str,
                nlink,
                user,
                group,
                size,
                time_format,
                file_name,
                width_nlink = max_nlink_len,
                width_user = max_user_len,
                width_group = max_group_len,
                width_size = max_size_len,
            ));
        } else {
            outputs.push(file_name.to_string());
        }
    }

    for output in outputs {
        println!("{}", output);
    }

    // Recursive listing
    if args.recursive {
        for entry_path in &final_entries {
            if entry_path.is_dir()
                && !entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .starts_with('.')
            {
                list_directory(entry_path, args, depth + 1)?;
            }
        }
    }

    Ok(())
}

fn bytes_to_human_readable(bytes: u64) -> String {
    if bytes < 1024 {
        return bytes.to_string();
    }
    let units = ["K", "M", "G", "T", "P", "E", "Z", "Y"];
    let mut display_bytes = bytes as f64;
    let mut unit_index = 0;
    while display_bytes >= 1024.0 && unit_index < units.len() - 1 {
        display_bytes /= 1024.0;
        unit_index += 1;
    }
    format!("{:.1}{}", display_bytes, units[unit_index])
}
