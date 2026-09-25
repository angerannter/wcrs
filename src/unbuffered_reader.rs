use std::error::Error;
use std::fs;
use std::path::Path;
use crate::args::Args;
use crate::result::OutputData;

pub fn count_single_file_unbuffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
    let mut output_data = OutputData {
        bytes: 0,
        words: 0,
        chars: 0,
        lines: 0,
    };

    read_file(&args.files, &mut output_data)?;

    Ok(output_data)
}

pub fn count_files_unbuffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
    let mut output_data = OutputData {
        bytes: 0,
        words: 0,
        chars: 0,
        lines: 0,
    };

    count_dir(&args.files, &mut output_data)?;

    Ok(output_data)
}

fn count_dir(path: &Path, output_data: &mut OutputData) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            count_dir(&path, output_data)?;
        } else if path.is_file() {
            read_file(&path, output_data)?;
        }
    }

    Ok(())
}

fn read_file(path: &Path, output_data: &mut OutputData) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(path)?;

    output_data.bytes += content.len();
    output_data.chars += content.chars()
        .count();
    output_data.lines += content.bytes()
        .filter(|&b| b == b'\n')
        .count();
    output_data.words += content.split_whitespace()
        .count();

    Ok(())
}