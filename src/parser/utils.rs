use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::parse_statement::parse_statement;
use super::PrologStatement;

fn line_comment(line: &str) -> String {
    if let Some(index) = line.find('%') {
        line[..index].trim().to_string()
    } else {
        line.trim().to_string()
    }
}

fn multiline_comment(line: &str) -> String {
    let mut cleaned = String::new();
    let mut in_block_comment = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                in_block_comment = false;
                chars.next();
            }

            continue
        }

        if c == '/' && chars.peek() == Some(&'*') {
            in_block_comment = true;
            chars.next();
            continue
        }

        cleaned.push(c);
    }

    cleaned.trim().to_string()
}

fn clean_line(line: &str) -> String {
    let line_after_multiline_cleanup = multiline_comment(line);
    line_comment(&line_after_multiline_cleanup)
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
                            PrologStatement::PrologFact(_fact) => {
                            }
                            PrologStatement::PrologRule(_rule) => {
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
