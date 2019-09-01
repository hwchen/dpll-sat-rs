type Literal = i32;
type Clause = Vec<Literal>;

pub fn dpll(clauses: Vec<Clause>) -> Option<Clause> {
    if clauses.is_empty() {
        return Some(vec![]);
    }

    inner_dpll(vec![], clauses)
}

fn inner_dpll(solution: Clause, clauses: Vec<Clause>) -> Option<Clause> {
    if clauses.is_empty() {
        return Some(solution);
    }

    if clauses.contains(&vec![]) {
        return None;
    }

    if contains_atomic(&clauses) {
        let atom = get_atomic_clauses(&clauses)[0];
        let mut atom_solution = vec![atom];
        atom_solution.extend_from_slice(&solution);

        return inner_dpll(atom_solution, reduce_clauses(atom, &clauses));
    }

    // otherwise
    let lit = clauses.iter().nth(0).unwrap()[0]; // unwrap, no empty clauses
    let not_lit = negate_literal(lit);

    let mut lit_solution = vec![lit];
    lit_solution.extend_from_slice(&solution);
    let mut not_lit_solution = vec![not_lit];
    not_lit_solution.extend_from_slice(&solution);

    let red_pos = inner_dpll(lit_solution, reduce_clauses(lit, &clauses));
    // TODO do this one only if not red_pos.0
    let red_neg = inner_dpll(not_lit_solution, reduce_clauses(not_lit, &clauses));

    if red_pos.is_some() { red_pos }
    else if red_neg.is_some() { red_neg }
    else { None }
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

fn get_atomic_clauses(clauses: &[Clause]) -> Clause {
    clauses.iter()
        .filter(|clause| clause.len() == 1)
        .flat_map(|clause| clause.iter())
        .cloned()
        .collect()
}

fn contains_atomic(clauses: &[Clause]) -> bool {
    get_atomic_clauses(&clauses).len() != 0
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
        assert_eq!(solution, Some(vec![-3, -1]));

        let clauses = vec![
            vec![1, -5, 4],
            vec![-1, 5, 3, 4],
            vec![-3, -4],
        ];
        let solution = dpll(clauses);
        assert_eq!(solution, Some(vec![-3, 5, 1]));

        let clauses = vec![
            vec![1],
            vec![-2],
            vec![3],
        ];
        let solution = dpll(clauses);
        assert_eq!(solution, Some(vec![3, -2, 1]));
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
    fn test_contains_atomic() {
        assert_eq!(contains_atomic(&[vec![1], vec![1,2]]), true);
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
