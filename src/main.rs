mod assignment;
mod constraint;
mod csp;
mod domain;
mod variable;

use constraint::common;
use csp::Csp;
use domain::HashSetDomain;
use solver::Solver;
use variable::Variable;

fn main() {
    let _ = variable::Variable::<i32>::new("x");

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
    let is_different = |a: &String, b: &String| a != b;

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

    // Tasmania is an island, no adjacency constraints needed for it

    // Print the CSP
    println!("{}", australia);

    // Solve the problem using backtracking search
    let solution = Solver::backtrack_search(&australia);

    // Print the solution
    match solution {
        Some(assignment) => println!("Solution found: {}", assignment),
        None => println!("No solution found"),
    }
}
