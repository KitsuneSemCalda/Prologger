use super::{parse_fact::parse_fact, parse_rule::parse_rule, PrologStatement};

pub fn parse_statement(line: &str) -> Option<PrologStatement> {
    if !line.contains(":-") {
        parse_fact(line).map(PrologStatement::PrologFact)
    } else {
        parse_rule(line).map(PrologStatement::PrologRule)
    }
}
