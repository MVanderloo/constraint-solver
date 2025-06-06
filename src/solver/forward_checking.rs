use crate::csp::{Assignment, Domain, Variable, csp::Csp};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub struct ForwardCheckingSolver;

impl ForwardCheckingSolver {
    pub fn solve<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        let mut assignment = Assignment::new();
        let mut domains: HashMap<Variable<T>, D> = csp
            .get_variables()
            .into_iter()
            .filter_map(|var| csp.get_domain(&var).map(|domain| (var, domain.clone())))
            .collect();

        if Self::backtrack_fc(&mut assignment, csp, &mut domains) {
            Some(assignment)
        } else {
            None
        }
    }

    fn backtrack_fc<T, D>(
        assignment: &mut Assignment<T>,
        csp: &Csp<T, D>,
        domains: &mut HashMap<Variable<T>, D>,
    ) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        if assignment.is_complete(csp.num_variables()) {
            return true;
        }

        let var = Self::select_variable(assignment, domains);
        if let Some(var) = var {
            let domain = domains.get(&var).unwrap().clone();

            for value in domain.values() {
                assignment.assign(var.clone(), value.clone());

                if csp.is_consistent(assignment) {
                    let saved_domains = domains.clone();

                    if Self::forward_check(&var, &value, assignment, csp, domains) {
                        if Self::backtrack_fc(assignment, csp, domains) {
                            return true;
                        }
                    }

                    *domains = saved_domains;
                }

                assignment.unassign(&var);
            }
        }

        false
    }

    fn select_variable<T, D>(
        assignment: &Assignment<T>,
        domains: &HashMap<Variable<T>, D>,
    ) -> Option<Variable<T>>
    where
        T: Clone + Eq + Hash + Debug,
        D: Domain<T>,
    {
        domains
            .keys()
            .filter(|var| !assignment.is_assigned(var))
            .min_by_key(|var| domains.get(var).unwrap().size())
            .cloned()
    }

    fn forward_check<T, D>(
        assigned_var: &Variable<T>,
        _assigned_value: &T,
        assignment: &Assignment<T>,
        csp: &Csp<T, D>,
        domains: &mut HashMap<Variable<T>, D>,
    ) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        // check all constraints involving the assigned variable
        for constraint in csp.get_constraints_for_variable(assigned_var) {
            for var in constraint.variables() {
                if assignment.is_assigned(var) || var == assigned_var {
                    continue;
                }

                let current_domain = domains.get(var).unwrap().clone();
                let mut valid_values = Vec::new();

                for value in current_domain.values() {
                    let mut test_assignment = assignment.clone();
                    test_assignment.assign(var.clone(), value.clone());

                    if constraint.is_satisfied(&test_assignment) {
                        valid_values.push(value);
                    }
                }

                if valid_values.is_empty() {
                    return false;
                }

                let new_domain = current_domain.restrict_to(valid_values);
                domains.insert(var.clone(), new_domain);
            }
        }

        true
    }
}
