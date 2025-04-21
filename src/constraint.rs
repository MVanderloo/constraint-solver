use crate::assignment::Assignment;
use crate::variable::Variable;

use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::rc::Rc;

/// A constraint in a constraint satisfaction problem
pub struct Constraint<T: Clone + Eq + Hash + Debug> {
    /// The name of the constraint (for debugging and display)
    name: String,
    /// The variables involved in this constraint
    variables: Vec<Variable<T>>,
    /// The function that determines if the constraint is satisfied
    predicate: Rc<dyn Fn(&Assignment<T>) -> bool>,
}

impl<T: Clone + Eq + Hash + Debug> Constraint<T> {
    /// Creates a new constraint with the given name, variables, and predicate
    pub fn new<F>(name: &str, variables: Vec<Variable<T>>, predicate: F) -> Self
    where
        F: Fn(&Assignment<T>) -> bool + 'static,
    {
        Constraint {
            name: String::from(name),
            variables,
            predicate: Rc::new(predicate),
        }
    }

    /// Returns the name of this constraint
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the variables involved in this constraint
    pub fn variables(&self) -> &[Variable<T>] {
        &self.variables
    }

    /// Returns true if the constraint is satisfied by the given assignment
    pub fn is_satisfied(&self, assignment: &Assignment<T>) -> bool {
        // Check if all variables in the constraint are assigned
        let all_assigned = self.variables.iter().all(|var| assignment.is_assigned(var));

        // If all variables are assigned, check the predicate
        if all_assigned {
            (self.predicate)(assignment)
        } else {
            // If not all variables are assigned, the constraint is not violated
            true
        }
    }

    /// Returns true if the constraint is relevant to the given variable
    pub fn involves(&self, variable: &Variable<T>) -> bool {
        self.variables.contains(variable)
    }
}

impl<T: Clone + Eq + Hash + Debug> Display for Constraint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on [", self.name)?;

        for (i, var) in self.variables.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", var)?;
        }

        write!(f, "]")
    }
}

/// Module with common constraint factories
pub mod common {
    use super::*;

    /// Creates an "all different" constraint for the given variables
    pub fn all_different<T: Clone + Eq + Hash + Debug + 'static>(
        name: &str,
        variables: Vec<Variable<T>>,
    ) -> Constraint<T> {
        Constraint::new(name, variables.clone(), move |assignment| {
            let mut seen = HashSet::new();

            for var in &variables {
                if let Some(value) = assignment.get(var) {
                    if !seen.insert(value) {
                        return false; // Duplicate value found
                    }
                }
            }

            true
        })
    }

    /// Creates a binary constraint between two variables
    pub fn diff<T: Clone + Eq + Hash + Debug + 'static>(
        name: &str,
        var1: Variable<T>,
        var2: Variable<T>,
    ) -> Constraint<T> {
        let variables = vec![var1.clone(), var2.clone()];

        Constraint::new(name, variables, move |assignment| {
            let val1 = assignment.get(&var1);
            let val2 = assignment.get(&var2);

            match (val1, val2) {
                (Some(v1), Some(v2)) => v1 != v2,
                _ => true,
            }
        })
    }

    /// Creates a binary constraint between two variables
    pub fn same<T: Clone + Eq + Hash + Debug + 'static>(
        name: &str,
        var1: Variable<T>,
        var2: Variable<T>,
    ) -> Constraint<T> {
        let variables = vec![var1.clone(), var2.clone()];

        Constraint::new(name, variables, move |assignment| {
            let val1 = assignment.get(&var1);
            let val2 = assignment.get(&var2);

            match (val1, val2) {
                (Some(v1), Some(v2)) => v1 == v2,
                _ => true,
            }
        })
    }

    /// Creates a constraint for a sum of variables
    pub fn sum<T: Clone + Eq + Hash + Debug + Into<i32> + 'static>(
        name: &str,
        variables: Vec<Variable<T>>,
        target: i32,
    ) -> Constraint<T> {
        Constraint::new(name, variables.clone(), move |assignment| {
            let sum: i32 = variables
                .iter()
                .filter_map(|var| {
                    assignment.get(var).map(|v| {
                        let val: i32 = v.clone().into();
                        val
                    })
                })
                .sum();

            sum == target
        })
    }
}
