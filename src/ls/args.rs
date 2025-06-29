use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ls", version = "0.1.0", about = "List directory contents")]
pub struct LsArgs {
    #[arg(short, long, help = "Do not ignore entries starting with .")]
    pub all: bool,

    #[arg(short, long, help = "Use a long listing format")]
    pub long: bool,

    #[arg(short = 'H', long, help = "With -l, print human readable sizes (e.g., 1K 234M 2G)")]
    pub human_readable: bool,

    #[arg(short = 'r', long, help = "Reverse order while sorting")]
    pub reverse: bool,

    #[arg(short = 't', long, help = "Sort by modification time, newest first")]
    pub sort_time: bool,

    #[arg(short = 'S', help = "Sort by file size, largest first")]
    pub sort_size: bool,

    #[arg(short = 'R', long, help = "List subdirectories recursively")]
    pub recursive: bool,

    #[arg(default_value = ".", help = "Path to list")]
    pub path: String,
}