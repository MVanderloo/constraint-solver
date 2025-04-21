use crate::assignment::Assignment;
use crate::constraint::Constraint;
use crate::domain::Domain;
use crate::variable::Variable;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// A Constraint Satisfaction Problem
pub struct Csp<T: Clone + Eq + Debug + Hash, D: Domain<T>> {
    domains: HashMap<Variable<T>, D>,
    constraints: Vec<Constraint<T>>,
}

impl<T: Clone + Eq + Debug + Hash, D: Domain<T>> Csp<T, D> {
    /// Create a new empty CSP
    pub fn new() -> Self {
        Csp {
            domains: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Add a variable with its domain to the CSP
    pub fn add_variable(&mut self, variable: Variable<T>, domain: D) -> Result<(), String> {
        if self.domains.contains_key(&variable) {
            return Err(format!("Variable {} already exists", variable.name.clone()));
        }
        self.domains.insert(variable, domain);
        Ok(())
    }

    /// Add a constraint to the CSP
    pub fn add_constraint(&mut self, constraint: Constraint<T>) -> Result<(), String> {
        for var in constraint.variables() {
            if !self.domains.contains_key(var) {
                return Err(format!("Variable {} does not exist in the CSP", var.name));
            }
        }
        self.constraints.push(constraint);
        Ok(())
    }

    /// Get the domain for the given variable
    pub fn get_domain(&self, variable: &Variable<T>) -> Option<&D> {
        self.domains.get(variable)
    }

    /// Get all constraints that involve the given variable
    pub fn get_constraints_for_variable(&self, var: &Variable<T>) -> Vec<&Constraint<T>> {
        self.constraints
            .iter()
            .filter(|c| c.involves(var))
            .collect()
    }

    /// Get all variables
    pub fn get_variables(&self) -> Vec<Variable<T>> {
        self.domains.keys().cloned().collect()
    }

    /// Get all constraints
    pub fn get_constraints(&self) -> &[Constraint<T>] {
        &self.constraints
    }

    /// Get the number of variables
    pub fn num_variables(&self) -> usize {
        self.domains.len()
    }

    /// Get the number of constraints
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    /// Check if the given assignment is consistent with all constraints
    pub fn is_consistent(&self, assignment: &Assignment<T>) -> bool {
        for constraint in &self.constraints {
            if !constraint.is_satisfied(assignment) {
                return false;
            }
        }
        true
    }

    /// Check if the assignment is complete and consistent
    pub fn is_solution(&self, assignment: &Assignment<T>) -> bool {
        assignment.is_complete(self.num_variables()) && self.is_consistent(assignment)
    }
}

impl<T: Clone + Eq + Debug + Display + Hash, D: Domain<T>> Display for Csp<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "CSP with {} variables and {} constraints:",
            self.num_variables(),
            self.num_constraints()
        )?;
        writeln!(f, "Variables:")?;
        for (var, domain) in &self.domains {
            write!(f, "  {} with domain of size {}: {{", var, domain.size())?;
            let mut first = true;
            for val in domain.values() {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", val)?;
                first = false;
            }
            writeln!(f, "}}")?;
        }
        writeln!(f, "Constraints:")?;
        for (i, constraint) in self.constraints.iter().enumerate() {
            writeln!(f, "  {}: {}", i + 1, constraint)?;
        }
        Ok(())
    }
}
