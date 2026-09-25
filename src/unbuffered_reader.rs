use std::error::Error;
use std::fs;
use std::path::Path;
use crate::args::Args;
use crate::result::{assign_to_output_data, OutputData};

pub fn count_single_file_unbuffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
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

pub fn count_files_unbuffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
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
    output_data.chars += content.chars()
        .count();
    output_data.lines += content.bytes()
        .filter(|&b| b == b'\n')
        .count();
    output_data.words += content.split_whitespace()
        .count();
}