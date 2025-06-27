use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

fn show_nonprinting_chars(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        match ch {
            // 控制字符 (0-31, 除了 TAB 和 LF)
            c if c.is_control() && c != '\t' && c != '\n' => {
                let code = c as u8;
                if code < 32 {
                    result.push('^');
                    result.push((code + 64) as char);
                } else if code == 127 {
                    result.push_str("^?");
                } else {
                    // 處理高位字符 (128-255)
                    result.push_str("M-");
                    let adjusted = code - 128;
                    if adjusted < 32 {
                        result.push('^');
                        result.push((adjusted + 64) as char);
                    } else if adjusted == 127 {
                        result.push_str("^?");
                    } else {
                        result.push(adjusted as char);
                    }
                }
            }

            // 普通字符
            _ => result.push(ch),
        }
    }

    result
}

#[derive(Parser)]
#[command(name = "cat")]
#[command(version = "0.1.0")]
#[command(about = "連結所有指定檔案並將結果寫到標準輸出")]
#[command(
    long_about = "連結所有指定檔案並將結果寫到標準輸出。\n如果沒有指定檔案，或者檔案為「-」，則從標準輸入讀取。"
)]
pub struct Args {
    /// 檔案列表，如果沒有指定則從標準輸入讀取
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,

    /// 等效於 -vET
    #[arg(short = 'A', long = "show-all")]
    pub show_all: bool,

    /// 對非空輸出列編號，同時取消 -n 選項效果
    #[arg(short = 'b', long = "number-nonblank")]
    pub number_nonblank: bool,

    /// 等效於 -vE
    #[arg(short = 'e', hide = true)]
    pub e_flag: bool,

    /// 在每列結束處顯示 "$"
    #[arg(short = 'E', long = "show-ends")]
    pub show_ends: bool,

    /// 對輸出的所有列加上編號
    #[arg(short = 'n', long = "number")]
    pub number: bool,

    /// 不輸出多列空列
    #[arg(short = 's', long = "squeeze-blank")]
    pub squeeze_blank: bool,

    /// 與 -vT 等效
    #[arg(short = 't', hide = true)]
    pub t_flag: bool,

    /// 將跳格字元顯示為 ^I
    #[arg(short = 'T', long = "show-tabs")]
    pub show_tabs: bool,

    /// (忽略)
    #[arg(short = 'u', hide = true)]
    pub u_flag: bool,

    /// 使用 ^ 和 M- 引用，除了 LFD 和 TAB 之外
    #[arg(short = 'v', long = "show-nonprinting")]
    pub show_nonprinting: bool,
}

impl Args {
    /// 處理複合選項邏輯
    pub fn process_combined_flags(&mut self) {
        // -A 等效於 -vET
        if self.show_all {
            self.show_nonprinting = true;
            self.show_ends = true;
            self.show_tabs = true;
        }

        // -e 等效於 -vE
        if self.e_flag {
            self.show_nonprinting = true;
            self.show_ends = true;
        }

        // -t 等效於 -vT
        if self.t_flag {
            self.show_nonprinting = true;
            self.show_tabs = true;
        }

        // -b 會覆蓋 -n
        if self.number_nonblank {
            self.number = true;
        }
    }

    /// 獲取要處理的檔案列表，如果為空則返回 stdin 指示符
    pub fn get_files(&self) -> Vec<String> {
        if self.files.is_empty() {
            vec!["-".to_string()]
        } else {
            self.files
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect()
        }
    }

    pub fn print_files(&self) {
        let files = self.get_files();
        for file_name in files {
            let mut line_number = 1;

            if file_name == "-" {
                // read from stdin
                for line in io::stdin().lock().lines() {
                    let line = line.unwrap();
                    self.print_line(line, &mut line_number);
                }
            } else {
                // read from file
                let file = File::open(file_name).unwrap();
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.unwrap();
                    self.print_line(line, &mut line_number);
                }
            }
        }
    }

    pub fn print_line(&self, line: String, line_number: &mut i32) {
        if self.number_nonblank && line.is_empty() {
            // skip empty line
            println!("");
            return;
        }
        if self.number {
            // line number
            print!("{line_number:6}  ");
        }
        let mut string = if self.show_nonprinting {
            show_nonprinting_chars(&line)
        } else {
            line.to_string()
        };
        if self.show_ends {
            string.push('$');
        }
        if self.show_tabs {
            string = string.replace('\t', "\t^I");
        }
        println!("{string}");
        *line_number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_no_args() {
        let args = Args::try_parse_from(["cat"]).unwrap();
        assert_eq!(args.get_files(), vec!["-"]);
    }

    #[test]
    fn test_with_files() {
        let args = Args::try_parse_from(["cat", "file1.txt", "file2.txt"]).unwrap();
        assert_eq!(args.get_files(), vec!["file1.txt", "file2.txt"]);
    }

    #[test]
    fn test_show_all_flag() {
        let mut args = Args::try_parse_from(["cat", "-A", "file.txt"]).unwrap();
        args.process_combined_flags();

        assert!(args.show_all);
        assert!(args.show_nonprinting);
        assert!(args.show_ends);
        assert!(args.show_tabs);
    }

    #[test]
    fn test_e_flag() {
        let mut args = Args::try_parse_from(["cat", "-e", "file.txt"]).unwrap();
        args.process_combined_flags();

        assert!(args.e_flag);
        assert!(args.show_nonprinting);
        assert!(args.show_ends);
    }

    #[test]
    fn test_t_flag() {
        let mut args = Args::try_parse_from(["cat", "-t", "file.txt"]).unwrap();
        args.process_combined_flags();

        assert!(args.t_flag);
        assert!(args.show_nonprinting);
        assert!(args.show_tabs);
    }
}
