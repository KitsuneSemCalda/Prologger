pub mod utils;

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
