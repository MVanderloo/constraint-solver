use crate::csp::assignment::Assignment;
use crate::csp::csp::Csp;
use crate::csp::domain::Domain;
use crate::csp::variable::Variable;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Solver for constraint satisfaction problems
pub struct Solver;

impl Solver {
    /// Generic backtracking search that takes variable selection and value ordering strategies
    pub fn solve<T, D, VS, VO>(
        csp: &Csp<T, D>,
        select_variable: VS,
        order_values: VO,
    ) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        Self::backtrack(&mut Assignment::new(), csp, &select_variable, &order_values)
    }

    fn backtrack<T, D, VS, VO>(
        assignment: &mut Assignment<T>,
        csp: &Csp<T, D>,
        select_variable: &VS,
        order_values: &VO,
    ) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        // If assignment is complete, return it
        if assignment.is_complete(csp.num_variables()) {
            return Some(assignment.clone());
        }

        // Select an unassigned variable using the provided strategy
        let var = select_variable(assignment, csp);

        if let Some(var) = var {
            // Get domain for this variable
            if let Some(domain) = csp.get_domain(&var) {
                // Order values using the provided strategy
                let ordered_values = order_values(&var, domain, assignment, csp);

                for value in ordered_values {
                    // Try this assignment
                    assignment.assign(var.clone(), value);

                    // Check if it's consistent with all constraints
                    if csp.is_consistent(assignment) {
                        // Recursive call
                        let result =
                            Self::backtrack(assignment, csp, select_variable, order_values);
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

    /// Helper function: Standard variable selection (first unassigned)
    pub fn first_unassigned<T, D>(
        assignment: &Assignment<T>,
        csp: &Csp<T, D>,
    ) -> Option<Variable<T>>
    where
        T: Clone + Eq + Hash + Debug,
        D: Domain<T>,
    {
        csp.get_variables()
            .into_iter()
            .find(|var| !assignment.is_assigned(var))
    }

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

    /// Helper function: Standard value ordering (domain order)
    pub fn domain_order<T, D>(
        _var: &Variable<T>,
        domain: &D,
        _assignment: &Assignment<T>,
        _csp: &Csp<T, D>,
    ) -> Vec<T>
    where
        T: Clone + Eq + Debug + Hash,
        D: Domain<T>,
    {
        domain.values()
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

    /// Convenience method: Simple backtracking search
    pub fn backtrack_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::solve(csp, Self::first_unassigned, Self::domain_order)
    }

    /// Convenience method: MRV search
    pub fn mrv_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::solve(csp, Self::minimum_remaining_values, Self::domain_order)
    }

    /// Convenience method: LCV search
    pub fn lcv_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::solve(csp, Self::first_unassigned, Self::least_constraining_value)
    }

    /// Convenience method: MRV + LCV search
    pub fn mrv_lcv_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::solve(
            csp,
            Self::minimum_remaining_values,
            Self::least_constraining_value,
        )
    }
}
