use clap::Parser;

mod args;
use args::Args;

fn main() {
    let mut args = Args::parse();

    // 處理複合選項
    args.process_combined_flags();

    args.print_files();
}
