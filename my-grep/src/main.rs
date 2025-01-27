use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    file_name: String,
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let file = File::open(&cli.file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(&cli.pattern) {
            println!("{line}");
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = start(cli) {
        println!("Err: {e}");
    }
}
