use crate::csp::{Assignment, Domain, Variable, csp::Csp};
use std::fmt::{Debug, Display};
use std::hash::Hash;

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
                let valid_count = domain
                    .values()
                    .into_iter()
                    .filter(|val| {
                        let all_consistent =
                            csp.get_constraints_for_variable(var)
                                .iter()
                                .all(|constraint| {
                                    let mut temp_assignment = assignment.clone();
                                    temp_assignment.assign(var.clone(), val.clone());
                                    constraint.is_satisfied(&temp_assignment)
                                });
                        all_consistent
                    })
                    .count();
                valid_count
            } else {
                usize::MAX
            }
        })
}

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
    let mut value_scores: Vec<(T, usize)> = domain
        .values()
        .into_iter()
        .map(|val| {
            let constraints_imposed = csp
                .get_variables()
                .into_iter()
                .filter(|other_var| !assignment.is_assigned(other_var) && other_var != var)
                .map(|other_var| {
                    if let Some(other_domain) = csp.get_domain(&other_var) {
                        other_domain
                            .values()
                            .into_iter()
                            .filter(|other_val| {
                                let mut test_assignment = assignment.clone();
                                test_assignment.assign(var.clone(), val.clone());
                                test_assignment.assign(other_var.clone(), other_val.clone());
                                !csp.is_consistent(&test_assignment)
                            })
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>();

            (val, constraints_imposed)
        })
        .collect();

    value_scores.sort_by_key(|(_, score)| *score);
    value_scores.into_iter().map(|(val, _)| val).collect()
}

// degree heuristic for tie-breaking with mrv
pub fn degree_heuristic<T, D>(assignment: &Assignment<T>, csp: &Csp<T, D>) -> Option<Variable<T>>
where
    T: Clone + Eq + Hash + Debug + Display,
    D: Domain<T>,
{
    csp.get_variables()
        .into_iter()
        .filter(|var| !assignment.is_assigned(var))
        .max_by_key(|var| {
            csp.get_constraints_for_variable(var)
                .iter()
                .map(|constraint| {
                    constraint
                        .variables()
                        .iter()
                        .filter(|v| !assignment.is_assigned(v))
                        .count()
                })
                .sum::<usize>()
        })
}

pub fn mrv_degree<T, D>(assignment: &Assignment<T>, csp: &Csp<T, D>) -> Option<Variable<T>>
where
    T: Clone + Eq + Hash + Debug + Display,
    D: Domain<T>,
{
    let unassigned: Vec<_> = csp
        .get_variables()
        .into_iter()
        .filter(|var| !assignment.is_assigned(var))
        .collect();

    if unassigned.is_empty() {
        return None;
    }

    let min_remaining = unassigned
        .iter()
        .map(|var| {
            if let Some(domain) = csp.get_domain(var) {
                domain
                    .values()
                    .into_iter()
                    .filter(|val| {
                        let mut temp_assignment = assignment.clone();
                        temp_assignment.assign(var.clone(), val.clone());
                        csp.is_consistent(&temp_assignment)
                    })
                    .count()
            } else {
                usize::MAX
            }
        })
        .min()
        .unwrap_or(usize::MAX);

    unassigned
        .into_iter()
        .filter(|var| {
            if let Some(domain) = csp.get_domain(var) {
                let remaining = domain
                    .values()
                    .into_iter()
                    .filter(|val| {
                        let mut temp_assignment = assignment.clone();
                        temp_assignment.assign(var.clone(), val.clone());
                        csp.is_consistent(&temp_assignment)
                    })
                    .count();
                remaining == min_remaining
            } else {
                false
            }
        })
        .max_by_key(|var| {
            csp.get_constraints_for_variable(var)
                .iter()
                .map(|constraint| {
                    constraint
                        .variables()
                        .iter()
                        .filter(|v| !assignment.is_assigned(v))
                        .count()
                })
                .sum::<usize>()
        })
}
