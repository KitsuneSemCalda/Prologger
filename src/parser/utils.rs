use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_file(filepath: &str) -> io::Result<()> {
    let path = Path::new(&filepath);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    
    for line in reader.lines(){
        match line {
            Ok(content) => println!("{}", content),
            Err(error) => eprintln!("Error in read line: {}", error)
        }
    }

    Ok(())
}
