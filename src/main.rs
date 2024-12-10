use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let p = Path::new(&args[1]);

    let file = File::open(&p);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
