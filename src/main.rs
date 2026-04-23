use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Parser)]
struct Args {
    #[arg(long)]
    path: String,
    level: Option<String>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let file: File = File::open (args.path)?;

    let reader = BufReader::new (file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        
        if line.contains ("ERROR") {
            println!("{}", line);
             count+=1;
            
        }
        
    }

    println!("Total errors: {}", count);
    

    Ok(())
    
}
