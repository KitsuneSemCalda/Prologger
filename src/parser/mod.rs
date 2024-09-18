pub mod utils;

/* Struct representing a Prolog Rule.
 
A Rule consists of a Head and a Body.
 
# Head: The Head appears before the `:-` and represents the query being searched for.
 
# Body: The Body appears after the `:-` and represents a list of facts or sub-queries that are chained together.
*/

#[derive(Debug)]
pub struct PrologRule{
    pub head: String, 
    pub body: Vec<String>,
}

/* Struct representing a Prolog Fact. 

A Fact consists in a Head with a assertion to be considered True.

# Head: The head represents a statement that is assumed to be true. 

Dont have a separated body is simply considered true.

*/

#[derive(Debug)]
pub struct PrologFact {
    pub head: String,
}

/* Enum representing either a Prolog Fact or Prolog Rule.

# Variants:
    - Fact: represents a fact, which is a statement assumed to be true.
    - Rule: represents a rule, which with a Head and a Body and need be infered
*/

#[derive(Debug)]
pub enum PrologStatement {
    PrologFact(PrologFact),
    PrologRule(PrologRule),
}
