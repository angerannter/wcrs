use clap::Parser;

#[derive(Parser, Debug)]
pub struct OutputData {
    pub bytes: usize,
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
}

