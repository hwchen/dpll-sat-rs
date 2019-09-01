type Literal = i32;
type Clause = Vec<Literal>;

pub fn dpll(clauses: Vec<Clause>) -> Option<Clause> {
    if clauses.is_empty() {
        return Some(vec![]);
    }

    // deal with atomic clauses first
    let mut solution = get_atomic_clauses(&clauses);

    let mut clauses = clauses;

    for atom in &solution {
        reduce_clauses(*atom, &mut clauses);
    }

    // now the rest
    let sat = inner_dpll(&mut solution, &mut clauses);
    if sat {
        Some(solution)
    } else {
        None
    }
}

fn inner_dpll(solution: &mut Clause, clauses: &mut Vec<Clause>) -> bool {
    if clauses.is_empty() {
        return true;
    }

    if clauses.contains(&vec![]) {
        return false;
    }

    let lit = clauses[0][0]; // clauses should not be empty. This is checked above.
    solution.push(lit);

    let mut try_clauses = clauses.clone();
    reduce_clauses(lit, &mut try_clauses);
    let red_pos = inner_dpll(solution, &mut try_clauses);

    red_pos ||
    {
        // red_neg
        let not_lit = negate_literal(lit);
        solution.pop();
        solution.push(not_lit);

        reduce_clauses(not_lit, clauses);
        inner_dpll(solution, clauses)
    } ||
    false
}

fn negate_literal(literal: Literal) -> Literal {
    literal * -1
}

fn remove_literal(lit: Literal, clause: &mut Clause) {
    let idxs = clause.iter().enumerate()
        .filter(|(_i, l)| **l ==lit)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    for idx in idxs.iter().rev() {
        clause.remove(*idx);
    }
}

fn get_atomic_clauses(clauses: &[Clause]) -> Vec<Literal> {
    clauses.iter()
        .filter(|clause| clause.len() == 1)
        .flat_map(|clause| clause.iter())
        .cloned()
        .collect()
}

fn reduce_clauses(lit: Literal, clauses: &mut Vec<Clause>) {
    let red_sat_idxs = clauses.iter().enumerate()
        .filter(|(_i, clause)| clause.contains(&lit))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    for idx in red_sat_idxs.iter().rev() {
        clauses.remove(*idx);
    }

    for clause in clauses.iter_mut() {
            let not_lit = negate_literal(lit);
            remove_literal(not_lit, clause)
    }
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
        let mut clause = vec![3,1,2,3];
        remove_literal(3, &mut clause);
        assert_eq!(clause, vec![1,2]);
    }

    #[test]
    fn test_get_atomic_clauses() {
        assert_eq!(get_atomic_clauses(&[vec![1], vec![1,2]]), vec![1]);
    }

    #[test]
    fn test_reduce_clauses() {
        let empty_vec: &[Vec<_>] = &[];

        let mut clauses_0 = vec![vec![1], vec![1,2,3]];
        let mut clauses_1 = vec![vec![1], vec![2,3]];
        let mut clauses_2 = vec![vec![1], vec![1,2]];
        let mut clauses_3 = vec![vec![-2], vec![1,2,3]];
        let mut clauses_4 = vec![vec![-2,-1], vec![1,2,-3]];
        let mut clauses_5 = vec![vec![-2,-1], vec![1,2,-3]];
        let mut clauses_6 = vec![vec![-2,-1], vec![1,2,-3]];

        reduce_clauses(1, &mut clauses_0);
        reduce_clauses(1, &mut clauses_1);
        reduce_clauses(2, &mut clauses_2);
        reduce_clauses(2, &mut clauses_3);
        reduce_clauses(2, &mut clauses_4);
        reduce_clauses(1, &mut clauses_5);
        reduce_clauses(-3, &mut clauses_6);

        assert_eq!(clauses_0, empty_vec);
        assert_eq!(clauses_1, &[vec![2,3]]);
        assert_eq!(clauses_2, &[vec![1]]);
        assert_eq!(clauses_3, &[vec![]]);
        assert_eq!(clauses_4, &[vec![-1]]);
        assert_eq!(clauses_5, &[vec![-2]]);
        assert_eq!(clauses_6, &[vec![-2,-1]]);
    }
}
