use super::{parse_predicate::parse_predicate, PrologFact};

pub fn parse_fact(line: &str) -> Option<PrologFact> {
    let predicate = parse_predicate(line)?;

    Some(PrologFact { predicate })
}
