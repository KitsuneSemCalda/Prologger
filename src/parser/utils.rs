use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::parse_statement::parse_statement;
use super::PrologStatement;

fn clean_line(line: &str) -> String {
    if let Some(index) = line.find('%') {
        line[..index].trim().to_string()
    } else {
        line.trim().to_string()
    }
}

pub fn parse_file(filepath: &str) -> io::Result<()> {
    let path = Path::new(&filepath);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let cleaned_line = clean_line(&content);

                if !cleaned_line.is_empty() {
                    if let Some(parsed_line) = parse_statement(&cleaned_line) {
                        match parsed_line {
                            PrologStatement::PrologFact(fact) => {
                                println!("Parsed fact: {}", fact.predicate);
                            }
                            PrologStatement::PrologRule(rule) => {
                                println!("Parsed rule: {}", rule);
                                for predicate in &rule.body {
                                    println!("  Body predicate: {}", predicate);
                                }
                            }
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("Occurs an unknoledge error: {}", e);
            }
        }
    }

    Ok(())
}
