// examples/australia.rs
use crate::csp::Assignment;
use crate::csp::constraint::common;
use crate::csp::csp::Csp;
use crate::csp::domain::HashSetDomain;
use crate::csp::variable::Variable;
use std::collections::HashMap;

pub fn create_australia_csp() -> Csp<String, HashSetDomain<String>> {
    // Create a CSP for the Australian map coloring problem
    let mut australia = Csp::<String, HashSetDomain<String>>::new();

    // Define the regions as variables
    let wa = Variable::new("WA");
    let nt = Variable::new("NT");
    let sa = Variable::new("SA");
    let q = Variable::new("Q");
    let nsw = Variable::new("NSW");
    let v = Variable::new("V");
    let t = Variable::new("T");

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

    australia
}

pub fn print_australia_map(assignment: Option<&Assignment<String>>) {
    // ASCII art representation of Australia
    let mut map = vec![
        "                      +------+    ".to_string(),
        "                      |  Q   |    ".to_string(),
        "    +------+------+---+      |    ".to_string(),
        "    |      |      |   |      |    ".to_string(),
        "    |  WA  |  NT  | SA+------+    ".to_string(),
        "    |      |      |   |      |    ".to_string(),
        "    |      |      |   | NSW  |    ".to_string(),
        "    +------+------+---+      |    ".to_string(),
        "                  |   +------+    ".to_string(),
        "                  | V |           ".to_string(),
        "                  +---+           ".to_string(),
        "                                  ".to_string(),
        "                      +---+       ".to_string(),
        "                      | T |       ".to_string(),
        "                      +---+       ".to_string(),
    ];

    // If assignment is provided, fill in colors
    if let Some(assignment) = assignment {
        // Define ANSI color codes for the terminal
        let color_codes = HashMap::from([
            ("red", "\x1b[41m"),   // Red background
            ("green", "\x1b[42m"), // Green background
            ("blue", "\x1b[44m"),  // Blue background
        ]);
        let reset = "\x1b[0m"; // Reset formatting

        // Replace the map with colored versions
        let regions = [
            ("WA", (4, 5)),
            ("NT", (12, 5)),
            ("SA", (20, 5)),
            ("Q", (24, 1)),
            ("NSW", (24, 6)),
            ("V", (22, 9)),
            ("T", (24, 13)),
        ];

        for (name, (x, y)) in regions {
            let var = Variable::<String>::new(name);
            if let Some(color) = assignment.get(&var) {
                if let Some(code) = color_codes.get(color.as_str()) {
                    let row = &mut map[y];
                    let colored_char = format!("{}{}{}", code, " ", reset);
                    let new_row = row[0..x].to_string() + &colored_char + &row[x + 1..].to_string();
                    map[y] = new_row;
                }
            }
        }
    }

    // Print the map
    println!("Australia Map Coloring:");
    for line in map {
        println!("{}", line);
    }

    // Print legend if assignment is provided
    if assignment.is_some() {
        println!("\nColor Legend:");
        println!("\x1b[41m  \x1b[0m Red");
        println!("\x1b[42m  \x1b[0m Green");
        println!("\x1b[44m  \x1b[0m Blue");
    }
}
