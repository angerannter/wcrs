mod args;
mod result;

use std::error::Error;
use std::fs;
use args::Args;
use clap::Parser;
use result::OutputData;

fn main() {
    let args = Args::parse();

    match run(&args) {
        Ok(_) => {
            if args.lines { print!("lines: {} ", args.lines) };
            if args.bytes { print!("bytes: {} ", args.bytes) };
            if args.words { print!("words: {} ", args.words) };
            if args.chars { print!("chars: {} ", args.chars) };
        }
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }

fn run(args: &Args) -> Result<OutputData, Box<dyn Error>> {
        if args.files.is_file() {
            if args.unbuffered {
                let ayri = count_single_file_unbuffered(args)?;
                println!("lines: {}, word: {}, byte: {}", ayri.lines, ayri.words, ayri.bytes);
            }
        }
        Err(Box::from("L"))
    }

    fn count_single_file_unbuffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
        let contents = fs::read_to_string(&args.files)?;

        let output_data = assign_to_output_data(
            args,
            contents.len(),
            contents.chars().count(),
            contents.lines().count() -1,
            contents.split_whitespace().count()
        );

        Ok(output_data)
    }
    fn assign_to_output_data(args: &Args, bytes: usize, chars: usize, lines: usize, words: usize) -> OutputData {
        OutputData {
            bytes: if args.bytes { bytes } else { 0 },
            chars: if args.chars { chars } else { 0 },
            lines: if args.lines { lines } else { 0 },
            words: if args.words { words } else { 0 },
        }
    }
}
