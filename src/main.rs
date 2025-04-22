use csp_solver::BacktrackingSolver;
use csp_solver::examples;
use std::time::Instant;

fn main() {
    let australia = examples::australia::create_australia_csp();
    println!("{}", australia);
    let start = Instant::now();
    let solution1 = BacktrackingSolver::backtrack_search(&australia);
    let duration = start.elapsed();
    match &solution1 {
        Some(assignment) => println!("{}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);

    let sudoku = examples::sudoku::create_sample_sudoku();
    println!("{}", sudoku);
    let start = Instant::now();
    let solution1 = BacktrackingSolver::backtrack_search(&sudoku);
    let duration = start.elapsed();
    match &solution1 {
        Some(assignment) => println!("{}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);

    let queens = examples::queens::create_queens_csp(8);
    println!("{}", queens);
    let start = Instant::now();
    let solution1 = BacktrackingSolver::backtrack_search(&queens);
    let duration = start.elapsed();
    match &solution1 {
        Some(assignment) => println!("{}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);
}
