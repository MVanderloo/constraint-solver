use constraint::common;
use csp_solver::csp::csp::Csp;
use csp_solver::csp::{assignment, constraint, domain, variable::Variable};
use csp_solver::solver::Solver;
use domain::HashSetDomain;
use std::time::Instant;

fn main() {
    // Create a CSP for the Australian map coloring problem
    let mut australia = Csp::<String, HashSetDomain<String>>::new();

    // Define the regions as variables
    let wa = Variable::new("Western Australia");
    let nt = Variable::new("Northern Territory");
    let sa = Variable::new("South Australia");
    let q = Variable::new("Queensland");
    let nsw = Variable::new("New South Wales");
    let v = Variable::new("Victoria");
    let t = Variable::new("Tasmania");

    // Define the colors as domain values
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let domain = HashSetDomain::new(colors);

    // Add variables to the CSP
    australia.add_variable(wa.clone(), domain.clone()).unwrap();
    australia.add_variable(nt.clone(), domain.clone()).unwrap();
    australia.add_variable(sa.clone(), domain.clone()).unwrap();
    australia.add_variable(q.clone(), domain.clone()).unwrap();
    australia.add_variable(nsw.clone(), domain.clone()).unwrap();
    australia.add_variable(v.clone(), domain.clone()).unwrap();
    australia.add_variable(t.clone(), domain.clone()).unwrap();

    // Define the adjacency constraints (regions that share a border)
    australia
        .add_constraint(common::diff("WA-NT", wa.clone(), nt.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("WA-SA", wa.clone(), sa.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("NT-SA", nt.clone(), sa.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("NT-Q", nt.clone(), q.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("SA-Q", sa.clone(), q.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("SA-NSW", sa.clone(), nsw.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("SA-V", sa.clone(), v.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("Q-NSW", q.clone(), nsw.clone()))
        .unwrap();
    australia
        .add_constraint(common::diff("NSW-V", nsw.clone(), v.clone()))
        .unwrap();

    // Print the CSP
    println!("{}", australia);

    // Solve using different strategies
    println!("\nSolving with simple backtracking:");
    let start = Instant::now();
    let solution1 = Solver::backtrack_search(&australia);
    let duration = start.elapsed();
    match &solution1 {
        Some(assignment) => println!("Solution found: {}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);

    println!("\nSolving with MRV heuristic:");
    let start = Instant::now();
    let solution2 = Solver::mrv_search(&australia);
    let duration = start.elapsed();
    match &solution2 {
        Some(assignment) => println!("Solution found: {}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);

    println!("\nSolving with LCV heuristic:");
    let start = Instant::now();
    let solution3 = Solver::lcv_search(&australia);
    let duration = start.elapsed();
    match &solution3 {
        Some(assignment) => println!("Solution found: {}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);

    println!("\nSolving with combined MRV and LCV heuristics:");
    let start = Instant::now();
    let solution4 = Solver::mrv_lcv_search(&australia);
    let duration = start.elapsed();
    match &solution4 {
        Some(assignment) => println!("Solution found: {}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);

    // Example of using a custom variable selection strategy
    println!("\nSolving with custom selection strategy (selecting most connected variable):");
    let start = Instant::now();
    let custom_selection = |assignment: &assignment::Assignment<String>,
                            csp: &Csp<String, HashSetDomain<String>>| {
        csp.get_variables()
            .into_iter()
            .filter(|var| !assignment.is_assigned(var))
            .max_by_key(|var| csp.get_constraints_for_variable(var).len())
    };

    let solution5 = Solver::solve(&australia, custom_selection, Solver::domain_order);
    let duration = start.elapsed();
    match &solution5 {
        Some(assignment) => println!("Solution found: {}", assignment),
        None => println!("No solution found"),
    }
    println!("Time taken: {:?}", duration);
}
