use super::PrologPredicate;

pub fn parse_predicates(line: &str) -> Option<Vec<PrologPredicate>> {
    println!("line to be treated: {}", line);
    let mut predicates: Vec<PrologPredicate> = Vec::new();

    let list_predicate = line.split(',').map(|s| s.trim());

    for predicate in list_predicate {
        println!("rule predicate: {}", predicate);
        if let Some(converted_predicate) = parse_predicate(predicate.trim()){
            predicates.push(converted_predicate);
        }
    }

    return Some(predicates)
}

pub fn parse_predicate(part: &str) -> Option<PrologPredicate> {
    let part = part.trim();

      if let Some(open_param_index) = part.find('(') {
        if let Some(close_param_index) = part.find(')') {
            let args_str = &part[open_param_index + 1..close_param_index];

            let head = &part[..open_param_index].trim();

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
            None
        }
    } else {
        Some(PrologPredicate { head: part.to_string(), args: vec![] })
    }
}
