// examples/sudoku.rs
use crate::csp::Assignment;
use crate::csp::constraint::common;
use crate::csp::csp::Csp;
use crate::csp::domain::VecDomain;
use crate::csp::variable::Variable;

// 4x4 Sudoku has values 1-4 and 2x2 boxes
pub fn create_sudoku_csp(initial_values: &[(usize, usize, usize)]) -> Csp<usize, VecDomain<usize>> {
    let mut csp = Csp::<usize, VecDomain<usize>>::new();

    // Create variables for each cell
    for row in 0..4 {
        for col in 0..4 {
            let var = Variable::new(&format!("C{}{}", row, col));

            // Check if there's an initial value for this cell
            let initial = initial_values
                .iter()
                .find(|(r, c, _)| *r == row && *c == col);

            if let Some((_, _, value)) = initial {
                // Set domain to just the initial value
                let domain = VecDomain::new(vec![*value]);
                csp.add_variable(var, domain).unwrap();
            } else {
                // Set domain to all possible values
                let domain = VecDomain::new(vec![1, 2, 3, 4]);
                csp.add_variable(var, domain).unwrap();
            }
        }
    }

    // Add row constraints (all different in each row)
    for row in 0..4 {
        let mut row_vars = Vec::new();
        for col in 0..4 {
            row_vars.push(Variable::new(&format!("C{}{}", row, col)));
        }
        let constraint = common::all_different(&format!("Row{}", row), row_vars);
        csp.add_constraint(constraint).unwrap();
    }

    // Add column constraints (all different in each column)
    for col in 0..4 {
        let mut col_vars = Vec::new();
        for row in 0..4 {
            col_vars.push(Variable::new(&format!("C{}{}", row, col)));
        }
        let constraint = common::all_different(&format!("Col{}", col), col_vars);
        csp.add_constraint(constraint).unwrap();
    }

    // Add box constraints (all different in each 2x2 box)
    for box_row in 0..2 {
        for box_col in 0..2 {
            let mut box_vars = Vec::new();
            for row in 0..2 {
                for col in 0..2 {
                    box_vars.push(Variable::new(&format!(
                        "C{}{}",
                        box_row * 2 + row,
                        box_col * 2 + col
                    )));
                }
            }
            let constraint = common::all_different(&format!("Box{}{}", box_row, box_col), box_vars);
            csp.add_constraint(constraint).unwrap();
        }
    }

    csp
}

pub fn print_sudoku_board(assignment: Option<&Assignment<usize>>) {
    println!("4x4 Sudoku:");

    // Print top border
    println!("+-----------+");

    for row in 0..4 {
        print!("|");

        for col in 0..4 {
            let var = Variable::<usize>::new(&format!("C{}{}", row, col));

            let value = if let Some(assignment) = assignment {
                assignment
                    .get(&var)
                    .map_or(" ".to_string(), |v| v.to_string())
            } else {
                " ".to_string()
            };

            // Add dividers between boxes
            print!(" {}", value);

            if col % 2 == 1 {
                print!(" |");
            }
        }
        println!();

        // Add horizontal dividers between boxes
        if row % 2 == 1 {
            println!("+-----------+");
        }
    }

    // Make sure we have a bottom border if the last row wasn't a box boundary
    if 4 % 2 != 0 {
        println!("+-----------+");
    }
}

pub fn create_sample_sudoku() -> Csp<usize, VecDomain<usize>> {
    // Create a sample 4x4 Sudoku with some initial values
    // Format: (row, column, value)
    let initial_values = vec![
        (0, 0, 1),
        (0, 3, 4),
        (1, 2, 4),
        (2, 1, 1),
        (3, 0, 4),
        (3, 3, 2),
    ];

    create_sudoku_csp(&initial_values)
}
