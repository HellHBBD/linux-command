use anyhow::Result;
use clap::Parser;

mod args;
use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    args.touch_files()
}