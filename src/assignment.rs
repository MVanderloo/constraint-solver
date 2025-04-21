use crate::variable::Variable;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Represents an assignment of values to variables in a CSP
#[derive(Debug, Clone)]
pub struct Assignment<T: Clone + Eq + Hash + Debug> {
    assignments: HashMap<Variable<T>, T>,
}

