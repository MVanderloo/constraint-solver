use std::fmt;
use std::hash::Hash;

/// A variable in a constraint satisfaction problem
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable<T> {
    /// The name of this variable
    pub name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Variable<T> {
    /// Creates a new variable with the given name
    pub fn new(name: &str) -> Self {
        Variable {
            name: String::from(name),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> fmt::Display for Variable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
