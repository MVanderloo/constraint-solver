use crate::csp::{Assignment, Domain, Variable, csp::Csp};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Helper function: MRV variable selection
pub fn minimum_remaining_values<T, D>(
    assignment: &Assignment<T>,
    csp: &Csp<T, D>,
) -> Option<Variable<T>>
where
    T: Clone + Eq + Hash + Debug + Display,
    D: Domain<T>,
{
    csp.get_variables()
        .into_iter()
        .filter(|var| !assignment.is_assigned(var))
        .min_by_key(|var| {
            if let Some(domain) = csp.get_domain(var) {
                domain
                    .values()
                    .into_iter()
                    .filter(|val| {
                        let mut test_assignment = assignment.clone();
                        test_assignment.assign(var.clone(), val.clone());
                        csp.is_consistent(&test_assignment)
                    })
                    .count()
            } else {
                usize::MAX
            }
        })
}

/// Helper function: LCV value ordering
pub fn least_constraining_value<T, D>(
    var: &Variable<T>,
    domain: &D,
    assignment: &Assignment<T>,
    csp: &Csp<T, D>,
) -> Vec<T>
where
    T: Clone + Eq + Hash + Debug + Display,
    D: Domain<T>,
{
    let mut values = domain.values();
    values.sort_by_key(|val| {
        // Count constraints imposed by this value
        let mut test_assignment = assignment.clone();
        test_assignment.assign(var.clone(), val.clone());

        let mut constraint_count = 0;

        // Check each unassigned neighboring variable
        for other_var in csp.get_variables() {
            if assignment.is_assigned(&other_var) || &other_var == var {
                continue;
            }

            if let Some(other_domain) = csp.get_domain(&other_var) {
                for other_val in other_domain.values() {
                    let mut extended_assignment = test_assignment.clone();
                    extended_assignment.assign(other_var.clone(), other_val);

                    if !csp.is_consistent(&extended_assignment) {
                        constraint_count += 1;
                    }
                }
            }
        }

        constraint_count
    });

    values
}
