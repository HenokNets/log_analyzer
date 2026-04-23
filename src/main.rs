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

        if let Some(ref level) = args.level {
            if line.contains(level) {
                println!("{}", line);
                count += 1;
            }
        } else {
            println!("{}", line);
        }
    }

    println!("Total matches: {}", count);

    Ok(())
}