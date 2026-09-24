mod args;

use std::fs;
use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("{:?}!", args);
    println!("Files: {:?}", args.files);

    let contents = fs::read_to_string(args.files)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
