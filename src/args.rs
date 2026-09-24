use std::path::PathBuf;
use clap::Parser;

/// Rust implementation of wc
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

    /// Read contents of files all at once instead of buffering (RAM intensive)
    #[arg(short, long, default_value_t = false)]
    pub unbuffered: bool,

    /// Files to be counted
    #[arg(short, long, default_value = ".")]
    pub files: PathBuf
}