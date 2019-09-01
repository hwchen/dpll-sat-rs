type Literal = i32;
type Clause = Vec<Literal>;

pub fn dpll(clauses: Vec<Clause>) -> Option<Clause> {
    if clauses.is_empty() {
        return Some(vec![]);
    }

    // deal with atomic clauses first
    let atomic_clauses = get_atomic_clauses(&clauses);

    let mut clauses = clauses;

    for atom in &atomic_clauses {
        clauses = reduce_clauses(*atom, &clauses);
    }

    // now the rest
    inner_dpll(atomic_clauses, clauses)
}

fn inner_dpll(solution: Clause, clauses: Vec<Clause>) -> Option<Clause> {
    if clauses.is_empty() {
        return Some(solution);
    }

    if clauses.contains(&vec![]) {
        return None;
    }

    let lit = clauses.iter().nth(0).unwrap()[0]; // unwrap, no empty clauses
    let mut lit_solution = solution.clone();
    lit_solution.push(lit);

    let red_pos = inner_dpll(lit_solution, reduce_clauses(lit, &clauses));

    red_pos.or_else(|| {
        // red_neg
        let not_lit = negate_literal(lit);
        let mut not_lit_solution = solution;
        not_lit_solution.push(not_lit);

        inner_dpll(not_lit_solution, reduce_clauses(not_lit, &clauses))
    })
    .or(None)
}

fn negate_literal(literal: Literal) -> Literal {
    literal * -1
}

fn remove_literal(lit: Literal, clause: &[Literal]) -> Clause {
    clause.iter()
        .filter(|l| **l != lit)
        .cloned()
        .collect()
}

fn get_atomic_clauses(clauses: &[Clause]) -> Vec<Literal> {
    clauses.iter()
        .filter(|clause| clause.len() == 1)
        .flat_map(|clause| clause.iter())
        .cloned()
        .collect()
}

fn reduce_clauses(lit: Literal, clauses: &[Clause]) -> Vec<Clause> {
    clauses.iter()
        // reduce_sat
        .filter(|clause| !clause.contains(&lit))
        // reduce_un_sat TODO mutate?
        .map(|clause| {
            let not_lit = negate_literal(lit);
            remove_literal(not_lit, &clause)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        // order matters? ah, because the SAT solver will output _one_ viable solution,
        // not all solutions

        let clauses = vec![
            vec![-1, -2],
            vec![-3, 2, 1],
        ];
        let solution = dpll(clauses);
        assert_eq!(solution, Some(vec![-1, -3]));

        let clauses = vec![
            vec![1, -5, 4],
            vec![-1, 5, 3, 4],
            vec![-3, -4],
        ];
        let solution = dpll(clauses);
        assert_eq!(solution, Some(vec![1, 5, -3]));

        let clauses = vec![
            vec![1],
            vec![-2],
            vec![3],
        ];
        let solution = dpll(clauses);
        assert_eq!(solution, Some(vec![1, -2, 3]));
    }

    #[test]
    fn test_remove_literal() {
        assert_eq!(remove_literal(3, &[3,1,2,3]), vec![1,2]);
    }

    #[test]
    fn test_get_atomic_clauses() {
        assert_eq!(get_atomic_clauses(&[vec![1], vec![1,2]]), vec![1]);
    }

    #[test]
    fn test_reduce_clauses() {
        let empty_vec: &[Vec<_>] = &[];
        assert_eq!(reduce_clauses(1, &[vec![1], vec![1,2,3]]), empty_vec);
        assert_eq!(reduce_clauses(1, &[vec![1], vec![2,3]]), &[vec![2,3]]);
        assert_eq!(reduce_clauses(2, &[vec![1], vec![1,2]]), &[vec![1]]);
        assert_eq!(reduce_clauses(2, &[vec![-2], vec![1,2,3]]), &[vec![]]);
        assert_eq!(reduce_clauses(2, &[vec![-2,-1], vec![1,2,-3]]), &[vec![-1]]);
        assert_eq!(reduce_clauses(1, &[vec![-2,-1], vec![1,2,-3]]), &[vec![-2]]);
        assert_eq!(reduce_clauses(-3, &[vec![-2,-1], vec![1,2,-3]]), &[vec![-2,-1]]);
    }
}
