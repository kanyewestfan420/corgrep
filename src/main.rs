use std::path::PathBuf;
use std::{fs, io};
use std::fs::File;
use std::io::{ BufReader, BufRead };
use colored::Colorize;
use clap::Parser;
use std::fs::metadata;

/// grep but written in rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    
    /// Pattern to find
    #[arg(short, long)]
    pattern: String,

    /// Path of directory or file to target
    #[arg(long, default_value = ".")]
    path: String,

}

fn get_files(path: String) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
}

fn main() -> io::Result<()> {

    let args = Args::parse();
    
    let path = args.path;

    let mut entries = get_files(path)?;

    let pattern = args.pattern;

    entries.sort();
	
    println!("{}{}\n", format!("cor").truecolor(200, 137, 0),format!("grep").truecolor(247, 47, 0));

    for entry in entries {
        if metadata(&entry).unwrap().is_dir() {
            continue;
        }
        let input = File::open(&entry)?;
        let buffered = BufReader::new(&input);
        let mut matches: i32 = 0;
        println!("File: {}", &entry.display());
        for (n , line) in buffered.lines().enumerate() {
            match line {
                Ok(_) => if line.as_ref().unwrap().to_string().contains(&pattern) {
                    matches += 1;
                    println!("{n} {}", line.unwrap().to_string().
                    trim_start().replace(&pattern, &format!("{}", &pattern.red())));
                },
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::InvalidData {
                        println!("Invalid data/encoding!")
                    } else {
                        println!("Unknown error!")
                    }
                    matches = -1;
                    break
                }
            }
            
        }
        if matches != -1 {
            println!("Matches found: {matches}")
        }
        println!("");
    }
    println!("{} on GitHub", "kanyewestfan420".bold());
    Ok(())
}
