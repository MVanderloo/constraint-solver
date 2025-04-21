use crate::assignment::Assignment;
use crate::csp::Csp;
use crate::domain::Domain;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Solver module for constraint satisfaction problems
pub struct Solver;

impl Solver {
    /// Simple backtracking search to solve a CSP
    pub fn backtrack_search<T: Clone + Eq + Hash + Debug + Display, D: Domain<T>>(
        csp: &Csp<T, D>,
    ) -> Option<Assignment<T>> {
        Self::backtrack(&mut Assignment::new(), csp)
    }

    /// Recursive backtracking search
    fn backtrack<T: Clone + Eq + Hash + Debug + Display, D: Domain<T>>(
        assignment: &mut Assignment<T>,
        csp: &Csp<T, D>,
    ) -> Option<Assignment<T>> {
        // If assignment is complete, return it
        if assignment.is_complete(csp.num_variables()) {
            return Some(assignment.clone());
        }

        // Select an unassigned variable
        let unassigned_vars: Vec<_> = csp
            .get_variables()
            .into_iter()
            .filter(|var| !assignment.is_assigned(var))
            .collect();

        if let Some(var) = unassigned_vars.first() {
            let var = var.clone();

            // Try each value in the domain
            if let Some(domain) = csp.get_domain(&var) {
                for value in domain.values() {
                    // Try this assignment
                    assignment.assign(var.clone(), value.clone());

                    // Check if it's consistent with all constraints
                    if csp.is_consistent(assignment) {
                        // Recursive call
                        let result = Self::backtrack(assignment, csp);
                        if result.is_some() {
                            return result;
                        }
                    }

                    // If we get here, this assignment didn't work
                    assignment.unassign(&var);
                }
            }
        }

        // No solution found
        None
    }
}
