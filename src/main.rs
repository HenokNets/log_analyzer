use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Args {
    #[arg(long, help = "Path to the log file")]
    path: String,

    #[arg(long, help = "Filter by log level (e.g., ERROR, INFO)")]
    level: Option<String>,

    #[arg(long, help = "Filter logs containing this keyword")]
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

        fn parse_line(line: &str) -> Option<LogEntry> {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 3 {
                return None;
            }

            let level = parts[0];
            let message = parts[2..].join(" ");

            Some(LogEntry {
                level: level.to_string(),
                message,
            })
        }

        fn matches_filters(entry: &LogEntry, args: &Args) -> bool {
            let mut matches = true;

            //level filter
            if let Some(ref level) = args.level {
                if entry.level != *level {
                    matches = false;
                }
            }

            //keyword filter
            if let Some(ref keyword) = args.keyword {
                if !entry.message.contains(keyword) {
                    matches = false;
                }
            }

            matches
}
    }

    println!("Total matches: {}", count);

    Ok(())
}
