use std::path::PathBuf;
use clap::Parser;

/// Rust implementation of wc, pass either a file or directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Byte count of target files(s)
    #[arg(short, long, default_value_t = true)]
    pub bytes: bool,

    /// Character count of target files(s)
    #[arg(short, long, default_value_t = false)]
    pub chars: bool,

    /// Newline count of target file(s)
    #[arg(short, long, default_value_t = true)]
    pub lines: bool,

    /// Word count of target file(s)
    #[arg(short, long, default_value_t = true)]
    pub words: bool,

    /// Buffered read of target file(s); may be slower but less RAM intensive
    #[arg(short = 'B', long, default_value_t = false)]
    pub buffered_read: bool,

    /// File(s) to be counted
    #[arg(short, long, default_value = ".")]
    pub files: PathBuf
}