pub mod parse_fact;
pub mod parse_predicate;
pub mod parse_rule;
pub mod parse_statement;
pub mod utils;

use core::fmt;

/// Structure representing a Prolog predicate.
///
/// A predicate consists of a name (head) and a list of arguments.
///
/// # Fields
/// - `head`: The name of the predicate.
/// - `args`: A list of arguments for the predicate.
#[derive(Debug)]
pub struct PrologPredicate {
    pub head: String,
    pub args: Vec<String>,
}

impl fmt::Display for PrologPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.head)
        } else {
            write!(f, "{}({})", self.head, self.args.join(", "))
        }
    }
}

/// Structure representing a Prolog rule.
///
/// A rule consists of a head and a body.
///
/// # Fields
/// - `head`: The predicate representing the rule.
/// - `body`: A list of predicates that form the body of the rule.
#[derive(Debug)]
pub struct PrologRule {
    pub head: PrologPredicate,
    pub body: Vec<PrologPredicate>,
}

impl fmt::Display for PrologRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let body_str: Vec<String> = self.body.iter().map(|b| b.to_string()).collect();
        write!(f, "{} :- {}", self.head, body_str.join(", "))
    }
}

/// Structure representing a Prolog fact.
///
/// A fact is a statement that is considered true.
///
/// # Fields
/// - `head`: The predicate representing the fact.
#[derive(Debug)]
pub struct PrologFact {
    pub predicate: PrologPredicate,
}

impl fmt::Display for PrologFact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fact: {}", self.predicate)
    }
}

/// Enum representing a Prolog statement.
///
/// Variants include:
/// - `PrologFact`: represents a fact, which is an assertion assumed to be true.
/// - `PrologRule`: represents a rule, consisting of a head and a body that need to be inferred.
#[derive(Debug)]
pub enum PrologStatement {
    PrologFact(PrologFact),
    PrologRule(PrologRule),
}
