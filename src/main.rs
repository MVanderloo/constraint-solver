use csp_solver::solver::heuristics::mrv_degree;
use csp_solver::solver::utils::domain_order;
use csp_solver::solver::{ArcConsistencySolver, ForwardCheckingSolver};
use csp_solver::{BacktrackingSolver, examples};
use std::time::Instant;

fn main() {
    println!("=== CSP Solver Performance Comparison ===\n");

    // test australia map coloring
    println!("--- Australia Map Coloring ---");
    test_australia_problem();
    println!();

    // test sudoku
    println!("--- 4x4 Sudoku ---");
    test_sudoku_problem();
    println!();

    // test 8-queens
    println!("--- 8-Queens ---");
    test_queens_problem(8);
    println!();

    // test 12-queens for performance comparison
    println!("--- 12-Queens ---");
    test_queens_problem(12);
    println!();

    // demonstrate finding multiple solutions
    println!("=== Multiple Solutions Demo ===");
    demonstrate_multiple_solutions();

    // performance stress test
    println!("\n=== Stress Test ===");
    stress_test();
}

fn test_australia_problem() {
    let csp = examples::australia::create_australia_csp();
    println!(
        "Variables: {}, Constraints: {}",
        csp.num_variables(),
        csp.num_constraints()
    );

    type AustraliaSolver = fn(
        &csp_solver::csp::csp::Csp<String, csp_solver::csp::HashSetDomain<String>>,
    ) -> Option<csp_solver::csp::Assignment<String>>;

    let algorithms: Vec<(&str, AustraliaSolver)> = vec![
        ("Basic Backtrack", |csp| {
            BacktrackingSolver::backtrack_search(csp)
        }),
        ("MRV", |csp| BacktrackingSolver::mrv_search(csp)),
        ("LCV", |csp| BacktrackingSolver::lcv_search(csp)),
        ("MRV+LCV", |csp| BacktrackingSolver::mrv_lcv_search(csp)),
        ("MRV+Degree", |csp| {
            BacktrackingSolver::find_solution(csp, mrv_degree, domain_order)
        }),
        ("Forward Checking", |csp| ForwardCheckingSolver::solve(csp)),
        ("Arc Consistency", |csp| ArcConsistencySolver::solve(csp)),
    ];

    let mut results = Vec::new();

    for (name, solver_fn) in algorithms {
        let start = Instant::now();
        let solution = solver_fn(&csp);
        let duration = start.elapsed();

        let status = if solution.is_some() {
            "SOLVED"
        } else {
            "NO SOLUTION"
        };
        println!("{:18} | {:>10} | {:>12.2?}", name, status, duration);

        results.push((name, duration, solution.is_some()));
    }

    if let Some((fastest_name, fastest_time, _)) = results
        .iter()
        .filter(|(_, _, solved)| *solved)
        .min_by_key(|(_, time, _)| *time)
    {
        println!("Fastest: {} ({:.2?})", fastest_name, fastest_time);

        // show the solution
        if let Some(solution) = BacktrackingSolver::backtrack_search(&csp) {
            examples::australia::print_australia_map(Some(&solution));
        }
    }
}

fn test_sudoku_problem() {
    let csp = examples::sudoku::create_sample_sudoku();
    println!(
        "Variables: {}, Constraints: {}",
        csp.num_variables(),
        csp.num_constraints()
    );

    type SudokuSolver = fn(
        &csp_solver::csp::csp::Csp<usize, csp_solver::csp::VecDomain<usize>>,
    ) -> Option<csp_solver::csp::Assignment<usize>>;

    let algorithms: Vec<(&str, SudokuSolver)> = vec![
        ("Basic Backtrack", |csp| {
            BacktrackingSolver::backtrack_search(csp)
        }),
        ("MRV", |csp| BacktrackingSolver::mrv_search(csp)),
        ("LCV", |csp| BacktrackingSolver::lcv_search(csp)),
        ("MRV+LCV", |csp| BacktrackingSolver::mrv_lcv_search(csp)),
        ("MRV+Degree", |csp| {
            BacktrackingSolver::find_solution(csp, mrv_degree, domain_order)
        }),
        ("Forward Checking", |csp| ForwardCheckingSolver::solve(csp)),
        ("Arc Consistency", |csp| ArcConsistencySolver::solve(csp)),
    ];

    let mut results = Vec::new();

    for (name, solver_fn) in algorithms {
        let start = Instant::now();
        let solution = solver_fn(&csp);
        let duration = start.elapsed();

        let status = if solution.is_some() {
            "SOLVED"
        } else {
            "NO SOLUTION"
        };
        println!("{:18} | {:>10} | {:>12.2?}", name, status, duration);

        results.push((name, duration, solution.is_some()));
    }

    if let Some((fastest_name, fastest_time, _)) = results
        .iter()
        .filter(|(_, _, solved)| *solved)
        .min_by_key(|(_, time, _)| *time)
    {
        println!("Fastest: {} ({:.2?})", fastest_name, fastest_time);

        // show the solution
        if let Some(solution) = BacktrackingSolver::backtrack_search(&csp) {
            examples::sudoku::print_sudoku_board(Some(&solution));
        }
    }
}

fn test_queens_problem(n: usize) {
    let csp = examples::queens::create_queens_csp(n);
    println!(
        "Variables: {}, Constraints: {}",
        csp.num_variables(),
        csp.num_constraints()
    );

    type QueensSolver = fn(
        &csp_solver::csp::csp::Csp<usize, csp_solver::csp::VecDomain<usize>>,
    ) -> Option<csp_solver::csp::Assignment<usize>>;

    let algorithms: Vec<(&str, QueensSolver)> = vec![
        ("Basic Backtrack", |csp| {
            BacktrackingSolver::backtrack_search(csp)
        }),
        ("MRV", |csp| BacktrackingSolver::mrv_search(csp)),
        ("LCV", |csp| BacktrackingSolver::lcv_search(csp)),
        ("MRV+LCV", |csp| BacktrackingSolver::mrv_lcv_search(csp)),
        ("MRV+Degree", |csp| {
            BacktrackingSolver::find_solution(csp, mrv_degree, domain_order)
        }),
        ("Forward Checking", |csp| ForwardCheckingSolver::solve(csp)),
        ("Arc Consistency", |csp| ArcConsistencySolver::solve(csp)),
    ];

    let mut results = Vec::new();

    for (name, solver_fn) in algorithms {
        let start = Instant::now();
        let solution = solver_fn(&csp);
        let duration = start.elapsed();

        let status = if solution.is_some() {
            "SOLVED"
        } else {
            "NO SOLUTION"
        };
        println!("{:18} | {:>10} | {:>12.2?}", name, status, duration);

        results.push((name, duration, solution.is_some()));

        // bail early if taking too long
        if duration.as_secs() > 30 {
            println!("Stopping - algorithm taking too long");
            break;
        }
    }

    if let Some((fastest_name, fastest_time, _)) = results
        .iter()
        .filter(|(_, _, solved)| *solved)
        .min_by_key(|(_, time, _)| *time)
    {
        println!("Fastest: {} ({:.2?})", fastest_name, fastest_time);

        // show solution for smaller boards
        if n <= 8 {
            if let Some(solution) = BacktrackingSolver::backtrack_search(&csp) {
                examples::queens::print_queens_board(n, Some(&solution));
            }
        }
    }
}

fn demonstrate_multiple_solutions() {
    let queens_4 = examples::queens::create_queens_csp(4);
    let all_solutions = BacktrackingSolver::find_all_backtracking(&queens_4);
    println!("4-Queens has {} solutions", all_solutions.len());

    // show first few solutions
    for (i, solution) in all_solutions.iter().take(2).enumerate() {
        println!("Solution {}:", i + 1);
        examples::queens::print_queens_board(4, Some(solution));
    }
}

fn stress_test() {
    // test scalability with different queen sizes
    let sizes = vec![4, 6, 8, 10];

    println!("N-Queens scaling comparison (MRV vs Forward Checking):");
    println!(
        "{:>4} | {:>12} | {:>12} | {:>8}",
        "N", "MRV", "Forward Check", "Speedup"
    );
    println!("-----|--------------|--------------|--------");

    for n in sizes {
        let queens = examples::queens::create_queens_csp(n);

        // mrv baseline
        let start = Instant::now();
        let mrv_solution = BacktrackingSolver::mrv_search(&queens);
        let mrv_time = start.elapsed();

        // forward checking
        let start = Instant::now();
        let fc_solution = ForwardCheckingSolver::solve(&queens);
        let fc_time = start.elapsed();

        let speedup = if fc_time.as_nanos() > 0 {
            mrv_time.as_secs_f64() / fc_time.as_secs_f64()
        } else {
            f64::INFINITY
        };

        let mrv_status = if mrv_solution.is_some() {
            format!("{:.2?}", mrv_time)
        } else {
            "TIMEOUT".to_string()
        };

        let fc_status = if fc_solution.is_some() {
            format!("{:.2?}", fc_time)
        } else {
            "TIMEOUT".to_string()
        };

        println!(
            "{:>4} | {:>12} | {:>12} | {:>7.1}x",
            n, mrv_status, fc_status, speedup
        );

        // bail out if taking too long
        if mrv_time.as_secs() > 5 {
            println!("Stopping stress test - problems getting too large");
            break;
        }
    }

    // additional benchmark: compare all algorithms on 8-queens
    println!("\n8-Queens algorithm comparison:");
    let queens_8 = examples::queens::create_queens_csp(8);

    let mut times = Vec::new();

    // basic
    let start = Instant::now();
    let _ = BacktrackingSolver::backtrack_search(&queens_8);
    times.push(("Basic", start.elapsed()));

    // mrv
    let start = Instant::now();
    let _ = BacktrackingSolver::mrv_search(&queens_8);
    times.push(("MRV", start.elapsed()));

    // forward checking
    let start = Instant::now();
    let _ = ForwardCheckingSolver::solve(&queens_8);
    times.push(("FC", start.elapsed()));

    times.sort_by_key(|(_, time)| *time);

    println!("Ranking (fastest to slowest):");
    for (i, (name, time)) in times.iter().enumerate() {
        println!("{}. {} - {:?}", i + 1, name, time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_algorithms_find_solutions() {
        let australia = examples::australia::create_australia_csp();

        assert!(BacktrackingSolver::backtrack_search(&australia).is_some());
        assert!(BacktrackingSolver::mrv_search(&australia).is_some());
        assert!(ForwardCheckingSolver::solve(&australia).is_some());
        assert!(ArcConsistencySolver::solve(&australia).is_some());
    }

    #[test]
    fn test_performance_consistency() {
        let sudoku = examples::sudoku::create_sample_sudoku();

        // all algorithms should find solutions
        let basic_solution = BacktrackingSolver::backtrack_search(&sudoku);
        let mrv_solution = BacktrackingSolver::mrv_search(&sudoku);
        let fc_solution = ForwardCheckingSolver::solve(&sudoku);

        assert_eq!(basic_solution.is_some(), mrv_solution.is_some());
        assert_eq!(basic_solution.is_some(), fc_solution.is_some());

        if let (Some(basic), Some(mrv)) = (basic_solution, mrv_solution) {
            // should be valid solutions even if assignments differ
            assert!(sudoku.is_solution(&basic));
            assert!(sudoku.is_solution(&mrv));
        }
    }

    #[test]
    fn test_multiple_solutions() {
        let queens_4 = examples::queens::create_queens_csp(4);
        let solutions = BacktrackingSolver::find_all_backtracking(&queens_4);

        // 4-queens should have exactly 2 solutions
        assert_eq!(solutions.len(), 2);

        // all solutions should be valid
        for solution in &solutions {
            assert!(queens_4.is_solution(solution));
        }
    }
}
