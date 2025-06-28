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
    long_about = "將所指定的每個檔案的存取時間和修改時間變更為目前時間。\n\
除非指定 -c 或 -h 選項，否則指定不存在的檔案將會被建立為空檔案。\n\
如果所指定檔名為 - 則特殊處理，程式將變更與標準輸出相關聯的檔案的存取時間。"
)]
pub struct Args {
    #[arg(long = "help", action = ArgAction::Help, help = "顯示說明文字")]
    pub help: Option<bool>,

    /// 要處理的檔案列表
    #[arg(value_name = "檔案", help = "要變更時間戳的檔案", required = true)]
    pub files: Vec<PathBuf>,

    /// 只變更存取時間（簡短版本）
    #[arg(short = 'a', help = "只變更存取時間", conflicts_with = "modify_only")]
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
    #[arg(short = 'm', help = "只變更修改時間", conflicts_with = "access_only")]
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
}

impl Args {
    pub fn touch_files(&self) {
        let paths = &self.files;
        for path in paths {
            if path.exists() {
                // 檔案或資料夾存在，更新時間
                let current_time = SystemTime::now();

                // 設定 access time 和 modification time 為當前時間
                filetime::set_file_times(
                    path,
                    filetime::FileTime::from_system_time(current_time),
                    filetime::FileTime::from_system_time(current_time),
                )
                .unwrap();

                println!("Updated timestamps for: {}", path.display());
            } else {
                // 檔案不存在，建立空白檔案
                File::create(path).unwrap();
                println!("Created empty file: {}", path.display());
            }
        }
    }
}
