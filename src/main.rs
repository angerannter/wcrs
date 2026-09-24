mod args;
mod result;

use std::error::Error;
use std::fs;
use std::path::{Path};
use args::Args;
use clap::{Parser};
use result::OutputData;
use result::assign_to_output_data;

fn main() {
    let args = Args::parse();
    match run(&args) {
        Ok(result) => {
            if args.lines { print!("lines: {} ", result.lines) };
            if args.bytes { print!("bytes: {} ", result.bytes) };
            if args.words { print!("words: {} ", result.words) };
            if args.chars { print!("chars: {} ", result.chars) };
        }
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }

fn run(args: &Args) -> Result<OutputData, Box<dyn Error>> {
        if args.files.is_file() {
            if args.unbuffered {
                return count_single_file_unbuffered(args);
            }
        } else {
            if args.unbuffered {
                return count_files_unbuffered(args);
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

    fn count_files_unbuffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
        let mut output = OutputData {
            bytes: 0,
            words: 0,
            chars: 0,
            lines: 0,
        };

        args.files.read_dir()?.for_each(|entry| {
            let path = entry.unwrap().path();

            if path.is_dir() {
                read_path(&path, &mut output);
            }

            if path.is_file() {
                read_file(&path, &mut output);
            }
        });

        Ok(output)
    }

    fn read_path(path: &Path, output_data: &mut OutputData) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();

            if path.is_dir() {
                read_path(&path, output_data);
            } else if path.is_file() {
                read_file(&path, output_data);
            }
        }
    }

    fn read_file(path: &Path, output_data: &mut OutputData) {
        let content = fs::read_to_string(path).unwrap();

        output_data.bytes += content.len();
        output_data.chars += content.chars().count();
        output_data.lines += content.bytes()
            .filter(|&b| b == b'\n')
            .count();
        output_data.words += content.split_whitespace().count();
    }
}
