use dpll_sat::dpll;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let filepath = std::env::args().nth(1).expect("please enter a filepath");

    let data = std::fs::read_to_string(filepath).expect("error reading file");

    let cnf = data.lines()
        .skip_while(|line| {
            if let Some(c) = line.chars().nth(0) {
                c == 'c' || c == 'p'
            } else {
                true // this skipe empty leading lines
            }
        })
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("could not parse integer"))
                .take_while(|x| *x != 0) // 0 is line ending in file format
                .collect()
        })
        .collect();

    let solution = dpll(cnf);
    let end = start.elapsed();

    println!("{:?}, {:?}, {}.{:06}", solution, solution.as_ref().map(|xs| xs.len()), end.as_secs(), end.subsec_micros());
}

