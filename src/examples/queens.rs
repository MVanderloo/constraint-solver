use crate::csp::Assignment;
use crate::csp::constraint::Constraint;
use crate::csp::csp::Csp;
use crate::csp::domain::VecDomain;
use crate::csp::variable::Variable;

pub fn create_queens_csp(size: usize) -> Csp<usize, VecDomain<usize>> {
    let mut csp = Csp::<usize, VecDomain<usize>>::new();

    // Create one variable for each column, representing the row where the queen is placed
    for col in 0..size {
        let var = Variable::new(&format!("Q{}", col));
        let domain = VecDomain::new(0..size);
        csp.add_variable(var, domain).unwrap();
    }

    // Add constraints to prevent queens from attacking each other
    for i in 0..size {
        for j in i + 1..size {
            let var_i = Variable::new(&format!("Q{}", i));
            let var_j = Variable::new(&format!("Q{}", j));

            // Queens cannot be in the same row
            let name = format!("NotSameRow-{}-{}", i, j);
            let vars = vec![var_i.clone(), var_j.clone()];
            let constraint = Constraint::new(&name, vars, move |assignment| {
                if let (Some(row_i), Some(row_j)) = (assignment.get(&var_i), assignment.get(&var_j))
                {
                    // Check for same row
                    if row_i == row_j {
                        return false;
                    }

                    // Check for diagonal attack
                    let col_diff = (j as isize) - (i as isize);
                    let row_diff = (*row_j as isize) - (*row_i as isize);
                    if col_diff.abs() == row_diff.abs() {
                        return false;
                    }

                    true
                } else {
                    true
                }
            });

            csp.add_constraint(constraint).unwrap();
        }
    }

    csp
}

pub fn print_queens_board(size: usize, assignment: Option<&Assignment<usize>>) {
    println!("{}x{} Queens Problem:", size, size);

    // Print column indices
    print!("  ");
    for col in 0..size {
        print!(" {} ", col);
    }
    println!();

    // Print top border
    print!("  +");
    for _ in 0..size {
        print!("---+");
    }
    println!();

    // Print board with queens
    for row in 0..size {
        print!("{} |", row);
        for col in 0..size {
            let var = Variable::<usize>::new(&format!("Q{}", col));
            let has_queen = if let Some(assignment) = assignment {
                assignment.get(&var).map_or(false, |r| *r == row)
            } else {
                false
            };

            if has_queen {
                print!(" Q |");
            } else {
                print!("   |");
            }
        }
        println!();

        // Print row separator
        print!("  +");
        for _ in 0..size {
            print!("---+");
        }
        println!();
    }
}
