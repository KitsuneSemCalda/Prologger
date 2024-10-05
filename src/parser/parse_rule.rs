use super::{parse_predicate::parse_predicate, parse_predicate::parse_predicates, PrologRule};

pub fn parse_rule(line: &str) -> Option<PrologRule> {
    let parts: Vec<&str> = line.split(":-").collect();

    if parts.len() != 2 {
        return None;
    }

    let head = parse_predicate(parts[0].trim())?;
    let body = parse_predicates(parts[1].trim())?;

    Some(PrologRule { head, body })

}
