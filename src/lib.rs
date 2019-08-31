type Literal = i32;
type Clause = Vec<Literal>;

pub fn dll(clauses: Vec<Clause>) -> (bool, Clause) {
    if clauses.is_empty() {
        return (true, vec![]);
    }

    inner_dpll(vec![], clauses)
}

fn inner_dpll(solution: Clause, clauses: Vec<Clause>) -> (bool, Clause) {
    if clauses.is_empty() {
        return (true, solution);
    }

    if clauses.contains(&vec![]) {
        return (false, solution);
    }

    if contains_atomic(clauses.clone()) {
        let atom = get_atomic_clauses(clauses.clone()).pop().unwrap();
        let mut atom_solution = vec![atom];
        atom_solution.extend_from_slice(&solution);

        return inner_dpll(atom_solution, reduce_clauses(atom, clauses));
    }

    // otherwise
    let lit = clauses.iter().nth(0).unwrap()[0]; // unwrap, no empty clauses
    let not_lit = negate_literal(lit);
    let mut lit_solution = vec![lit];
    lit_solution.extend_from_slice(&solution);
    let mut not_lit_solution = vec![not_lit];
    not_lit_solution.extend_from_slice(&solution);

    let red_pos = inner_dpll(lit_solution, reduce_clauses(lit, clauses.clone()));
    // TODO do this one only if not red_pos.0
    let red_neg = inner_dpll(not_lit_solution, reduce_clauses(not_lit, clauses.clone()));

    if red_pos.0 { red_pos }
    else if red_neg.0 { red_neg }
    else { (false, solution) }
}

fn negate_literal(literal: Literal) -> Literal {
    literal * -1
}

fn remove_literal(lit: Literal, clause: Vec<Literal>) -> Vec<Literal>{
    let mut clause = clause.clone();
    let idx = clause.iter().position(|l| *l == lit);
    if let Some(idx) = idx {
        clause.remove(idx as usize);
    }
    clause
}

fn get_atomic_clauses(clauses: Vec<Clause>) -> Clause {
    clauses.into_iter()
        .filter(|clause| clause.len() == 1)
        .flat_map(|clause| clause.into_iter())
        .collect()
}

fn contains_atomic(clauses: Vec<Clause>) -> bool {
    get_atomic_clauses(clauses).len() != 0
}

fn reduce_clauses(lit: Literal, clauses: Vec<Clause>) -> Vec<Clause> {
    clauses.iter()
        // reduce_sat
        .filter(|clause| !clause.contains(&lit))
        // reduce_un_sat TODO mutate
        .map(|clause| {
            let not_lit = negate_literal(lit);
            remove_literal(not_lit, clause.clone())
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
