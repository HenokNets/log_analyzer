use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    path: String,

    #[arg(long)]
    level: Option<String>,

    #[arg(long)]
    keyword: Option<String>,
}

struct LogEntry {
    level: String,
    message: String,
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

        let entry = LogEntry {
            level: level.to_string(),
            message,
        };

        let mut matches = true;

        //level filter
        if let Some (ref level) = args.level {
            if entry.level != *level {
                matches=false;
            }
        }

        //keyword filter
        if let Some (ref keyword) = args.keyword {
            if !entry.message.contains (keyword) {
                matches=false;
            }
        }

        //final decision
        if matches {
            println!("{}", entry.message);
            count += 1;
        }

    }

    println!("Total matches: {}", count);

    Ok(())
}