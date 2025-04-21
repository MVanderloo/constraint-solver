use crate::csp::{Assignment, Domain, Variable, csp::Csp};
use std::fmt::Debug;
use std::hash::Hash;

/// Helper function: Standard variable selection (first unassigned)
pub fn first_unassigned<T, D>(assignment: &Assignment<T>, csp: &Csp<T, D>) -> Option<Variable<T>>
where
    T: Clone + Eq + Hash + Debug,
    D: Domain<T>,
{
    csp.get_variables()
        .into_iter()
        .find(|var| !assignment.is_assigned(var))
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
