use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::args::Args;
use crate::result::OutputData;

pub fn count_single_file_buffered(args: &Args) -> Result<OutputData, Box<dyn Error>> {
    let file = File::open(&args.files)?;
    let mut reader = BufReader::new(file);
    let mut output_data = OutputData {
        bytes: 0,
        lines: 0,
        words: 0,
        chars: 0
    };

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

    Ok(output_data)
}