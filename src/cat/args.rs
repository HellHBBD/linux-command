use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

fn show_nonprinting_chars(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        match ch {
            // 控制字符 (0-31, 除了 TAB 和 LF)
            c if c.is_control() && c as u8 != 0x09 && c as u8 != 0x0A => {
                let code = c as u8;
                if code < 0x20 {
                    result.push('^');
                    result.push((code + 0x40) as char);
                }
                // DEL character
                else if code == 0x7F {
                    result.push_str("^?");
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
#[command(about = "Concatenate files and print on the standard output.")]
#[command(
    long_about = "Concatenate files and print on the standard output. With no FILE, or when FILE is -, read standard input."
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
        let mut line_number = 1;
        let mut prev_line_empty = false;

        for file_name in files {
            let reader: Box<dyn BufRead> = if file_name == "-" {
                Box::new(BufReader::new(io::stdin()))
            } else {
                Box::new(BufReader::new(File::open(&file_name).unwrap()))
            };

            for line_result in reader.lines() {
                let mut line = line_result.unwrap();

                if self.squeeze_blank && line.is_empty() && prev_line_empty {
                    continue;
                }
                prev_line_empty = line.is_empty();

                // Numbering logic
                let mut prefix = String::new();
                if self.number {
                    if self.number_nonblank && line.is_empty() {
                        // Do not number blank lines if -b is used
                    } else {
                        prefix = format!("{:6}  ", line_number);
                        line_number += 1;
                    }
                }

                // Formatting logic
                if self.show_nonprinting {
                    line = show_nonprinting_chars(&line);
                }
                if self.show_tabs {
                    line = line.replace('\t', "^I");
                }
                if self.show_ends {
                    line.push('$');
                }

                println!("{}{}", prefix, line);
            }
        }
    }
}


