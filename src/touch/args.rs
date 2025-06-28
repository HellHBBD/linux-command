use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::{ArgAction, Parser};
use std::fs::File;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Parser)]
#[command(
    name = "touch",
    version = "1.0.0",
    disable_help_flag = true,
    about = "將所指定的每個檔案的存取時間和修改時間變更為目前時間",
    long_about = "將所指定的每個檔案的存取時間和修改時間變更為目前時間.\n\
除非指定 -c 或 -h 選項，否則指定不存在的檔案將會被建立為空檔案.\n\
如果所指定檔名為 - 則特殊處理，程式將變更與標準輸出相關聯的檔案的存取時間。"
)]
pub struct Args {
    #[arg(long = "help", action = ArgAction::Help, help = "顯示說明文字")]
    pub help: Option<bool>,

    /// 要處理的檔案列表
    #[arg(value_name = "檔案", help = "要變更時間戳的檔案", required = true)]
    pub files: Vec<PathBuf>,

    /// 只變更存取時間（簡短版本）
    #[arg(short = 'a', help = "只變更存取時間")]
    pub access_only: bool,

    /// 不建立任何檔案
    #[arg(short = 'c', long = "no-create", help = "不建立任何檔案")]
    pub no_create: bool,

    /// 使用指定字串表示時間而非目前時間
    #[arg(
        short = 'd',
        long = "date",
        value_name = "字串",
        help = "使用指定字串表示時間而非目前時間",
        conflicts_with_all = ["reference", "time_format"]
    )]
    pub date: Option<String>,

    /// 忽略選項（為了兼容性）
    #[arg(short = 'f', help = "(忽略)", hide = true)]
    pub force: bool,

    /// 會影響符號連結本身，而非符號連結所指示的目的地
    #[arg(
        short = 'h',
        long = "no-dereference",
        help = "會影響符號連結本身，而非符號連結所指示的目的地\n(當系統支援變更符號連結的所有者時，此選項才有用)"
    )]
    pub no_dereference: bool,

    /// 只變更修改時間
    #[arg(short = 'm', help = "只變更修改時間")]
    pub modify_only: bool,

    /// 使用此檔案的時間而非目前時間
    #[arg(
        short = 'r',
        long = "reference",
        value_name = "FILE",
        help = "使用此檔案的時間而非目前時間",
        conflicts_with_all = ["date", "time_format"]
    )]
    pub reference: Option<PathBuf>,

    /// 使用指定時間而非目前時間
    #[arg(
        short = 't',
        value_name = "[[CC]YY]MMDDhhmm[.ss]",
        help = "使用指定時間而非目前時間，\n時間格式與 -d 不同: [[CC]YY]MMDDhhmm[.ss]",
        conflicts_with_all = ["date", "reference"]
    )]
    pub time_format: Option<String>,

    #[arg(
        long = "time",
        value_name = "WORD",
        help = "指定要變更的時間：access、atime、use 或 modify、mtime"
    )]
    pub time: Option<String>,
}

impl Args {
    pub fn touch_files(&self) -> Result<()> {
        let (mut access_only, mut modify_only) = (self.access_only, self.modify_only);

        if let Some(time_word) = &self.time {
            match time_word.as_str() {
                "access" | "atime" | "use" => access_only = true,
                "modify" | "mtime" => modify_only = true,
                _ => anyhow::bail!("無效的時間值: {}", time_word),
            }
        }

        for path in &self.files {
            if !path.exists() && self.no_create {
                continue;
            }

            let (atime, mtime) = self.get_times(path)?;

            let atime = if modify_only {
                filetime::FileTime::from_last_access_time(&path.metadata()?)
            } else {
                atime
            };

            let mtime = if access_only {
                filetime::FileTime::from_last_modification_time(&path.metadata()?)
            } else {
                mtime
            };

            if self.no_dereference {
                filetime::set_symlink_file_times(path, atime, mtime)
                    .with_context(|| format!("failed to set times for symlink {:?}", path))?;
            } else {
                if !path.exists() {
                    File::create(path)
                        .with_context(|| format!("failed to create file {:?}", path))?;
                }
                filetime::set_file_times(path, atime, mtime)
                    .with_context(|| format!("failed to set times for {:?}", path))?;
            }
        }
        Ok(())
    }

    fn get_times(&self, _path: &PathBuf) -> Result<(filetime::FileTime, filetime::FileTime)> {
        if let Some(reference_path) = &self.reference {
            let metadata = std::fs::metadata(reference_path)
                .with_context(|| format!("failed to get metadata of {:?}", reference_path))?;
            Ok((
                filetime::FileTime::from_last_access_time(&metadata),
                filetime::FileTime::from_last_modification_time(&metadata),
            ))
        } else if let Some(date_str) = &self.date {
            let dt = parse_date_string(date_str)?;
            let ft = filetime::FileTime::from_system_time(dt.into());
            Ok((ft, ft))
        } else if let Some(time_str) = &self.time_format {
            let dt = parse_time_format(time_str)?;
            let ft = filetime::FileTime::from_system_time(dt.into());
            Ok((ft, ft))
        } else {
            let now = SystemTime::now();
            Ok((
                filetime::FileTime::from_system_time(now),
                filetime::FileTime::from_system_time(now),
            ))
        }
    }
}

fn parse_date_string(date_str: &str) -> Result<DateTime<Local>> {
    // Chrono's `parse_from_str` is quite powerful and can handle many formats.
    // We might need to add more specific format strings if needed.
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(&Local))
        .or_else(|_| {
            NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| Local.from_local_datetime(&ndt).single().unwrap())
        })
        .with_context(|| format!("無法解析日期字串: {}", date_str))
}

fn parse_time_format(time_str: &str) -> Result<DateTime<Local>> {
    let format = if time_str.contains('.') {
        "%Y%m%d%H%M.%S"
    } else {
        "%Y%m%d%H%M"
    };

    let year_str;
    let (year_part, time_part) = if time_str.len() > 12 {
        time_str.split_at(4)
    } else {
        let now = Local::now();
        year_str = now.format("%Y").to_string();
        (year_str.as_str(), time_str)
    };

    let full_time_str = format!("{}{}", year_part, time_part);

    NaiveDateTime::parse_from_str(&full_time_str, format)
        .map(|ndt| Local.from_local_datetime(&ndt).single().unwrap())
        .with_context(|| format!("無法解析時間格式: {}", time_str))
}
