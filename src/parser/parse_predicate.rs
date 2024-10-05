use super::PrologPredicate;

pub fn parse_predicate(part: &str) -> Option<PrologPredicate> {
    let npart = part.trim();

    if let Some(open_param_index) = npart.find('(') {
        if let Some(close_param_index) = npart.find(')') {
            let args_str = &npart[open_param_index + 1..close_param_index];
            let head = &npart[..open_param_index].trim();

            let args: Vec<String> = args_str
                .split(',')
                .map(|arg| arg.trim().to_string())
                .filter(|arg| !arg.is_empty())
                .collect();

            Some(PrologPredicate {
                head: head.to_string(),
                args,
            })
        } else {
            eprintln!("Missing closing parenthesis");
            None
        }
    } else {
        Some(PrologPredicate {
            head: part.to_string(),
            args: vec![],
        })
    }
}

pub fn parse_predicates(line: &str) -> Option<Vec<PrologPredicate>> {
    let mut predicates = Vec::new();
    let mut current_predicate = String::new();
    let mut open_parens = 0;

    for c in line.chars() {
        match c {
            '(' => {
                open_parens += 1;
                current_predicate.push(c);
            }
            ')' => {
                open_parens -= 1;
                current_predicate.push(c);
            }
            ',' if open_parens == 0 => {
                if !current_predicate.is_empty() {
                    predicates.push(parse_predicate(&current_predicate)?);
                    current_predicate.clear();
                }
            }
            _ => current_predicate.push(c),
        }
    }

    if !current_predicate.is_empty() {
        predicates.push(parse_predicate(&current_predicate)?);
        current_predicate.clear();
    }

    Some(predicates)
}
