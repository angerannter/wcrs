mod args;
mod result;
mod unbuffered_reader;
mod buffered_reader;

use std::error::Error;
use args::Args;
use clap::{Parser};
use result::OutputData;
use crate::buffered_reader::{count_files_buffered, count_single_file_buffered};
use crate::unbuffered_reader::{count_files_unbuffered, count_single_file_unbuffered};

fn main() {
    let args = Args::parse();
    match run(&args) {
        Ok(result) => {
            if args.lines { print!("lines: {} ", result.lines) };
            if args.chars { print!("chars: {} ", result.chars) };
            if args.words { print!("words: {} ", result.words) };
            if args.bytes { print!("bytes: {} ", result.bytes) };
        }
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

fn run(args: &Args) -> Result<OutputData, Box<dyn Error>> {
    if args.files.is_file() {
        if !args.buffered_read {
            count_single_file_unbuffered(args)
        } else {
            count_single_file_buffered(args)
        }
    } else {
        if !args.buffered_read {
            count_files_unbuffered(args)
        } else {
            count_files_buffered(args)
        }
    }
}
