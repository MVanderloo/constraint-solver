use crate::constraint::Constraint;
use crate::variable::Variable;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

/// Represents an assignment of values to variables in a CSP
#[derive(Debug, Clone)]
pub struct Assignment<T: Clone + Eq + Hash + Debug> {
    assignments: HashMap<Variable<T>, T>,
}

impl<T: Clone + Eq + Hash + Debug> Assignment<T> {
    /// Create a new empty assignment
    pub fn new() -> Self {
        Assignment {
            assignments: HashMap::new(),
        }
    }

    /// Assign a value to a variable
    pub fn assign(&mut self, var: Variable<T>, value: T) {
        self.assignments.insert(var, value);
    }

    /// Remove the assignment for a variable
    pub fn unassign(&mut self, var: &Variable<T>) {
        self.assignments.remove(var);
    }

    /// Check if a variable is assigned
    pub fn is_assigned(&self, var: &Variable<T>) -> bool {
        self.assignments.contains_key(var)
    }

    /// Get the value assigned to a variable, if any
    pub fn get(&self, var: &Variable<T>) -> Option<&T> {
        self.assignments.get(var)
    }

    /// Get the number of assigned variables
    pub fn size(&self) -> usize {
        self.assignments.len()
    }

    /// Check if this is a complete assignment (for a given number of variables)
    pub fn is_complete(&self, num_variables: usize) -> bool {
        self.size() == num_variables
    }

    /// Returns an iterator over all variable names
    pub fn variables(&self) -> impl Iterator<Item = &Variable<T>> {
        self.assignments.keys()
    }

    /// Returns an iterator over all variable-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&Variable<T>, &T)> {
        self.assignments.iter()
    }

    /// Get a copy of all assignments as a HashMap
    pub fn get_assignments(&self) -> HashMap<Variable<T>, T> {
        self.assignments.clone()
    }

    /// Check if this assignment is consistent with all given constraints
    pub fn is_consistent(&self, constraints: &[Constraint<T>]) -> bool {
        for constraint in constraints {
            if !constraint.is_satisfied(self) {
                return false;
            }
        }
        true
    }
}

impl<T: Clone + Eq + Hash + Debug + Display> Display for Assignment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (var, val) in &self.assignments {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", var, val)?;
            first = false;
        }
        write!(f, "}}")
    }
}
