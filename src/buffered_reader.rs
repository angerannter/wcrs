use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::args::Args;
use crate::result::OutputData;

pub fn count_single_file_buffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
    let mut output_data = OutputData {
        bytes: 0,
        lines: 0,
        words: 0,
        chars: 0
    };

    count_file(&args.files, &mut output_data)?;

    Ok(output_data)
}

pub fn count_files_buffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
    let mut output_data = OutputData {
        bytes: 0,
        lines: 0,
        words: 0,
        chars: 0
    };

    count_dir(&args.files, &mut output_data)?;

    Ok(output_data)
}

pub fn count_file(path: &Path, output_data: &mut OutputData) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        output_data.bytes += bytes_read;

        if buffer.last() == Some(&b'\n') {
            output_data.lines += 1;
        }

        let line = std::str::from_utf8(&buffer)?;

        output_data.chars += line.chars().count();
        output_data.words += line.split_whitespace().count();
    }

    Ok(())
}

pub fn count_dir(path: &Path, output_data: &mut OutputData) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            count_dir(&path, output_data)?;
        } else if path.is_file() {
            count_file(&path, output_data)?;
        }
    }

    Ok(())
}