use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    path: String,

    #[arg(long)]
    level: Option<String>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        let level = parts[0];
        let _timestamp = parts[1];
        let message = parts[2..].join(" ");

    }

    println!("Total matches: {}", count);

    Ok(())
}