use crate::csp::{Assignment, Domain, Variable, csp::Csp};
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub struct ArcConsistencySolver;

impl ArcConsistencySolver {
    pub fn solve<T, D>(csp: &Csp<T, D>) -> Option<Assignment<T>>
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        let mut domains: HashMap<Variable<T>, D> = csp
            .get_variables()
            .into_iter()
            .filter_map(|var| csp.get_domain(&var).map(|domain| (var, domain.clone())))
            .collect();

        // apply ac-3 preprocessing
        if !Self::ac3(csp, &mut domains) {
            return None; // inconsistent
        }

        let mut assignment = Assignment::new();
        if Self::backtrack_ac(&mut assignment, csp, &mut domains) {
            Some(assignment)
        } else {
            None
        }
    }

    fn ac3<T, D>(csp: &Csp<T, D>, domains: &mut HashMap<Variable<T>, D>) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        let mut queue = VecDeque::new();

        // initialize queue with all arcs
        for constraint in csp.get_constraints() {
            let vars = constraint.variables();
            if vars.len() == 2 {
                queue.push_back((vars[0].clone(), vars[1].clone(), constraint));
                queue.push_back((vars[1].clone(), vars[0].clone(), constraint));
            }
        }

        while let Some((xi, xj, constraint)) = queue.pop_front() {
            if Self::revise(domains, &xi, &xj, constraint) {
                if domains.get(&xi).unwrap().is_empty() {
                    return false;
                }

                // add all arcs (xk, xi) for each neighbor xk of xi
                for other_constraint in csp.get_constraints_for_variable(&xi) {
                    for var in other_constraint.variables() {
                        if var != &xi && var != &xj {
                            queue.push_back((var.clone(), xi.clone(), other_constraint));
                        }
                    }
                }
            }
        }

        true
    }

    fn revise<T, D>(
        domains: &mut HashMap<Variable<T>, D>,
        xi: &Variable<T>,
        xj: &Variable<T>,
        constraint: &crate::csp::Constraint<T>,
    ) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        let mut revised = false;
        let xi_domain = domains.get(xi).unwrap().clone();
        let xj_domain = domains.get(xj).unwrap();

        let mut valid_values = Vec::new();

        for xi_value in xi_domain.values() {
            let mut satisfiable = false;

            for xj_value in xj_domain.values() {
                let mut test_assignment = Assignment::new();
                test_assignment.assign(xi.clone(), xi_value.clone());
                test_assignment.assign(xj.clone(), xj_value);

                if constraint.is_satisfied(&test_assignment) {
                    satisfiable = true;
                    break;
                }
            }

            if satisfiable {
                valid_values.push(xi_value);
            } else {
                revised = true;
            }
        }

        if revised {
            let new_domain = xi_domain.restrict_to(valid_values);
            domains.insert(xi.clone(), new_domain);
        }

        revised
    }

    fn backtrack_ac<T, D>(
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

        let var = domains
            .keys()
            .filter(|var| !assignment.is_assigned(var))
            .min_by_key(|var| domains.get(var).unwrap().size())
            .cloned();

        if let Some(var) = var {
            let domain = domains.get(&var).unwrap().clone();

            for value in domain.values() {
                assignment.assign(var.clone(), value.clone());

                if csp.is_consistent(assignment) {
                    let saved_domains = domains.clone();

                    // maintain arc consistency after assignment
                    if Self::maintain_arc_consistency(&var, &value, csp, domains) {
                        if Self::backtrack_ac(assignment, csp, domains) {
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

    fn maintain_arc_consistency<T, D>(
        assigned_var: &Variable<T>,
        assigned_value: &T,
        csp: &Csp<T, D>,
        domains: &mut HashMap<Variable<T>, D>,
    ) -> bool
    where
        T: Clone + Eq + Hash + Debug + Display,
        D: Domain<T>,
    {
        // reduce domain of assigned variable to single value
        let single_value_domain = domains
            .get(assigned_var)
            .unwrap()
            .restrict_to(vec![assigned_value.clone()]);
        domains.insert(assigned_var.clone(), single_value_domain);

        // run ac-3 with reduced domains
        Self::ac3(csp, domains)
    }
}
