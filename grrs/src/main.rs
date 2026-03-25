use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
// search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let arg = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
