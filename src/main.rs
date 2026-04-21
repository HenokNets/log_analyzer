use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Parser)]
struct Args {
    #[arg(long)]
    path: String,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let file = File::open (args.path)?;

    let reader = BufReader::new (file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    

    Ok(())
    
}
