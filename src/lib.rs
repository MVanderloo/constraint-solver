pub mod csp;
pub mod examples;
pub mod solver;

pub use csp::{Assignment, Constraint, Domain, Variable, csp::Csp};
pub use solver::backtracking::BacktrackingSolver;
