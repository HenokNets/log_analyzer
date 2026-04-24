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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.path)
                .map_err(|e| anyhow::anyhow!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        total+=1;

        if let Some(entry) = parse_line(&line) {
            if matches_filters(&entry, &args) {
                println!("[{}] {}", entry.level, entry.message);
                count += 1;
            }
        }
}

    if count == 0 {
        println!("No matching log entries found");
    }

    println!("Total lines: {}", total);
    println!("Matched lines: {}", count);

    Ok(())
}
