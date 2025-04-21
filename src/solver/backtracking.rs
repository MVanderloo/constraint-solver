use super::heuristics::{least_constraining_value, minimum_remaining_values};
use super::utils::{domain_order, first_unassigned};
use crate::csp::{csp::Csp, Assignment, Domain, Variable};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Base Backtracking solver implementation that other solvers build upon
pub struct BacktrackingSolver;

impl BacktrackingSolver {
    /// Generic solve method that uses backtracking to find solutions
    /// Takes variable selection and value ordering strategies
    /// The `collect_all` parameter determines whether to return the first solution
    /// or continue searching for all solutions
    fn solve_internal<T, D, VS, VO>(
        csp: &Csp<T, D>,
        select_variable: VS,
        order_values: VO,
        collect_all: bool,
    ) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        let mut solutions = Vec::new();
        Self::backtrack(
            &mut Assignment::new(),
            csp,
            &select_variable,
            &order_values,
            &mut solutions,
            collect_all,
        );
        solutions
    }

    /// Core backtracking algorithm
    fn backtrack<T, D, VS, VO>(
        assignment: &mut Assignment<T>,
        csp: &Csp<T, D>,
        select_variable: &VS,
        order_values: &VO,
        solutions: &mut Vec<Assignment<T>>,
        collect_all: bool,
    ) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        // If assignment is complete, add it to solutions
        if assignment.is_complete(csp.num_variables()) {
            solutions.push(assignment.clone());
            // If we're not collecting all solutions, we can stop after the first one
            return !collect_all;
        }

        // Select an unassigned variable using the provided strategy
        if let Some(var) = select_variable(assignment, csp) {
            // Get domain for this variable
            if let Some(domain) = csp.get_domain(&var) {
                // Order values using the provided strategy
                let ordered_values = order_values(&var, domain, assignment, csp);

                for value in ordered_values {
                    // Try this assignment
                    assignment.assign(var.clone(), value);

                    // Check if it's consistent with all constraints
                    if csp.is_consistent(assignment) {
                        // Recursive call to continue the search
                        if Self::backtrack(
                            assignment,
                            csp,
                            select_variable,
                            order_values,
                            solutions,
                            collect_all,
                        ) {
                            return true;
                        }
                    }

                    // Remove the assignment to try next value
                    assignment.unassign(&var);
                }
            }
        }

        false
    }

    /// Find a single solution using the provided heuristics
    pub fn find_solution<T, D, VS, VO>(
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
        let solutions = Self::solve_internal(csp, select_variable, order_values, false);
        solutions.into_iter().next()
    }

    /// Find all solutions using the provided heuristics
    pub fn find_all_solutions<T, D, VS, VO>(
        csp: &Csp<T, D>,
        select_variable: VS,
        order_values: VO,
    ) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        Self::solve_internal(csp, select_variable, order_values, true)
    }

    /// Find a limited number of solutions
    pub fn find_limited_solutions<T, D, VS, VO>(
        csp: &Csp<T, D>,
        select_variable: VS,
        order_values: VO,
        limit: usize,
    ) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        if limit == 0 {
            return Vec::new();
        }

        let mut solutions = Vec::with_capacity(limit);
        Self::backtrack_limited(
            &mut Assignment::new(),
            csp,
            &select_variable,
            &order_values,
            &mut solutions,
            limit,
        );
        solutions
    }

    fn backtrack_limited<T, D, VS, VO>(
        assignment: &mut Assignment<T>,
        csp: &Csp<T, D>,
        select_variable: &VS,
        order_values: &VO,
        solutions: &mut Vec<Assignment<T>>,
        limit: usize,
    ) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
        VS: Fn(&Assignment<T>, &Csp<T, D>) -> Option<Variable<T>>,
        VO: Fn(&Variable<T>, &D, &Assignment<T>, &Csp<T, D>) -> Vec<T>,
    {
        // Stop if we've reached the solution limit
        if solutions.len() >= limit {
            return true;
        }

        // If assignment is complete, add it to solutions
        if assignment.is_complete(csp.num_variables()) {
            solutions.push(assignment.clone());
            return solutions.len() >= limit;
        }

        // Select an unassigned variable using the provided strategy
        if let Some(var) = select_variable(assignment, csp) {
            // Get domain for this variable
            if let Some(domain) = csp.get_domain(&var) {
                // Order values using the provided strategy
                let ordered_values = order_values(&var, domain, assignment, csp);

                for value in ordered_values {
                    // Try this assignment
                    assignment.assign(var.clone(), value);

                    // Check if it's consistent with all constraints
                    if csp.is_consistent(assignment) {
                        // Recursive call to continue the search
                        if Self::backtrack_limited(
                            assignment,
                            csp,
                            select_variable,
                            order_values,
                            solutions,
                            limit,
                        ) {
                            return true;
                        }
                    }

                    // Remove the assignment to try next value
                    assignment.unassign(&var);
                }
            }
        }

        false
    }

    // Convenience methods for common use cases

    /// Simple backtracking search - finds a single solution
    pub fn backtrack_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_solution(csp, first_unassigned, domain_order)
    }

    /// MRV search - finds a single solution using MRV heuristic
    pub fn mrv_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_solution(csp, minimum_remaining_values, domain_order)
    }

    /// LCV search - finds a single solution using LCV heuristic
    pub fn lcv_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_solution(csp, first_unassigned, least_constraining_value)
    }

    /// MRV+LCV search - finds a single solution with both heuristics
    pub fn mrv_lcv_search<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_solution(csp, minimum_remaining_values, least_constraining_value)
    }

    /// Find all solutions using simple backtracking
    pub fn find_all_backtracking<T, D>(csp: &Csp<T, D>) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_all_solutions(csp, first_unassigned, domain_order)
    }

    /// Find all solutions using MRV heuristic
    pub fn find_all_mrv<T, D>(csp: &Csp<T, D>) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_all_solutions(csp, minimum_remaining_values, domain_order)
    }

    /// Find all solutions using LCV heuristic
    pub fn find_all_lcv<T, D>(csp: &Csp<T, D>) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_all_solutions(csp, first_unassigned, least_constraining_value)
    }

    /// Find all solutions using MRV+LCV heuristics
    pub fn find_all_mrv_lcv<T, D>(csp: &Csp<T, D>) -> Vec<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        Self::find_all_solutions(csp, minimum_remaining_values, least_constraining_value)
    }
}
