use clap::Parser;
use crate::args::Args;

#[derive(Parser, Debug)]
pub struct OutputData {
    pub bytes: usize,
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
}

pub fn assign_to_output_data(args: &Args, bytes: usize, chars: usize, lines: usize, words: usize) -> OutputData {
    OutputData {
        bytes: if args.bytes { bytes } else { 0 },
        chars: if args.chars { chars } else { 0 },
        lines: if args.lines { lines } else { 0 },
        words: if args.words { words } else { 0 },
    }
}


