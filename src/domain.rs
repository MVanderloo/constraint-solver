use std::cmp::Ord;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Trait defining the behavior of a variable domain
pub trait Domain<T: Clone + Eq + Debug>: Clone + Debug {
    /// Returns true if the domain contains the given value
    fn contains(&self, value: &T) -> bool;
    /// Returns the size of the domain (number of possible values)
    fn size(&self) -> usize;
    /// Returns true if the domain is empty
    fn is_empty(&self) -> bool;
    /// Returns all values in the domain as a vector
    fn values(&self) -> Vec<T>;
    /// Creates a copy of this domain with the given value removed
    fn remove(&self, value: &T) -> Self;
    /// Creates a copy of this domain with only the specified values kept
    fn restrict_to<I: IntoIterator<Item = T>>(&self, values_to_keep: I) -> Self;
}

/// Domain implementation using a HashSet
#[derive(Debug, Clone)]
pub struct HashSetDomain<T: Clone + Eq + Hash + Debug> {
    values: HashSet<T>,
}

impl<T: Clone + Eq + Hash + Debug> HashSetDomain<T> {
    /// Create a new domain from a collection of values
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        HashSetDomain {
            values: values.into_iter().collect(),
        }
    }

    /// Create a domain from a range (for integer domains)
    pub fn from_range(start: i32, end: i32) -> HashSetDomain<i32> {
        HashSetDomain {
            values: (start..=end).collect(),
        }
    }
}

impl<T: Clone + Eq + Hash + Debug> Domain<T> for HashSetDomain<T> {
    fn contains(&self, value: &T) -> bool {
        self.values.contains(value)
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn values(&self) -> Vec<T> {
        self.values.iter().cloned().collect()
    }

    fn remove(&self, value: &T) -> Self {
        let mut new_values = self.values.clone();
        new_values.remove(value);
        HashSetDomain { values: new_values }
    }

    fn restrict_to<I: IntoIterator<Item = T>>(&self, values_to_keep: I) -> Self {
        let keep_set: HashSet<T> = values_to_keep.into_iter().collect();
        let new_values: HashSet<T> = self
            .values
            .iter()
            .filter(|v| keep_set.contains(v))
            .cloned()
            .collect();
        HashSetDomain { values: new_values }
    }
}

/// Domain implementation using a sorted BTreeSet
#[derive(Debug, Clone)]
pub struct BTreeSetDomain<T: Clone + Eq + Ord + Debug> {
    values: BTreeSet<T>,
}

impl<T: Clone + Eq + Ord + Debug> BTreeSetDomain<T> {
    /// Create a new domain from a collection of values
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        BTreeSetDomain {
            values: values.into_iter().collect(),
        }
    }

    /// Create a domain from a range (for integer domains)
    pub fn from_range(start: i32, end: i32) -> BTreeSetDomain<i32> {
        BTreeSetDomain {
            values: (start..=end).collect(),
        }
    }
}

impl<T: Clone + Eq + Ord + Debug> Domain<T> for BTreeSetDomain<T> {
    fn contains(&self, value: &T) -> bool {
        self.values.contains(value)
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn values(&self) -> Vec<T> {
        self.values.iter().cloned().collect()
    }

    fn remove(&self, value: &T) -> Self {
        let mut new_values = self.values.clone();
        new_values.remove(value);
        BTreeSetDomain { values: new_values }
    }

    fn restrict_to<I: IntoIterator<Item = T>>(&self, values_to_keep: I) -> Self {
        let keep_set: BTreeSet<T> = values_to_keep.into_iter().collect();
        let new_values: BTreeSet<T> = self
            .values
            .iter()
            .filter(|v| keep_set.contains(v))
            .cloned()
            .collect();
        BTreeSetDomain { values: new_values }
    }
}

/// Domain implementation using a Vec (useful for small domains)
#[derive(Debug, Clone)]
pub struct VecDomain<T: Clone + Eq + Debug> {
    values: Vec<T>,
}

impl<T: Clone + Eq + Debug> VecDomain<T> {
    /// Create a new domain from a collection of values
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        VecDomain {
            values: values.into_iter().collect(),
        }
    }

    /// Create a domain from a range (for integer domains)
    pub fn from_range(start: i32, end: i32) -> VecDomain<i32> {
        VecDomain {
            values: (start..=end).collect(),
        }
    }
}

impl<T: Clone + Eq + Debug> Domain<T> for VecDomain<T> {
    fn contains(&self, value: &T) -> bool {
        self.values.contains(value)
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn values(&self) -> Vec<T> {
        self.values.clone()
    }

    fn remove(&self, value: &T) -> Self {
        let new_values: Vec<T> = self
            .values
            .iter()
            .filter(|v| *v != value)
            .cloned()
            .collect();
        VecDomain { values: new_values }
    }

    fn restrict_to<I: IntoIterator<Item = T>>(&self, values_to_keep: I) -> Self {
        let keep_vec: Vec<T> = values_to_keep.into_iter().collect();
        let new_values: Vec<T> = self
            .values
            .iter()
            .filter(|v| keep_vec.contains(v))
            .cloned()
            .collect();
        VecDomain { values: new_values }
    }
}

/// Domain implementation using a sorted Vec
#[derive(Debug, Clone)]
pub struct SortedVecDomain<T: Clone + Eq + Ord + Debug> {
    values: Vec<T>,
}

impl<T: Clone + Eq + Ord + Debug> SortedVecDomain<T> {
    /// Create a new domain from a collection of values
    pub fn new<I: IntoIterator<Item = T>>(values: I) -> Self {
        let mut values_vec: Vec<T> = values.into_iter().collect();
        values_vec.sort();
        values_vec.dedup();
        SortedVecDomain { values: values_vec }
    }

    /// Create a domain from a range (for integer domains)
    pub fn from_range(start: i32, end: i32) -> SortedVecDomain<i32> {
        SortedVecDomain {
            values: (start..=end).collect(),
        }
    }
}

impl<T: Clone + Eq + Ord + Debug> Domain<T> for SortedVecDomain<T> {
    fn contains(&self, value: &T) -> bool {
        self.values.binary_search(value).is_ok()
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn values(&self) -> Vec<T> {
        self.values.clone()
    }

    fn remove(&self, value: &T) -> Self {
        match self.values.binary_search(value) {
            Ok(index) => {
                let mut new_values = self.values.clone();
                new_values.remove(index);
                SortedVecDomain { values: new_values }
            }
            Err(_) => self.clone(),
        }
    }

    fn restrict_to<I: IntoIterator<Item = T>>(&self, values_to_keep: I) -> Self {
        let mut keep_vec: Vec<T> = values_to_keep.into_iter().collect();
        keep_vec.sort();
        let mut new_values = Vec::new();
        let mut keep_iter = keep_vec.iter();
        let mut current_keep = keep_iter.next();
        for value in &self.values {
            while let Some(keep) = current_keep {
                match keep.cmp(value) {
                    std::cmp::Ordering::Less => {
                        current_keep = keep_iter.next();
                    }
                    std::cmp::Ordering::Equal => {
                        new_values.push(value.clone());
                        current_keep = keep_iter.next();
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        break;
                    }
                }
            }
            if current_keep.is_none() {
                break;
            }
        }
        SortedVecDomain { values: new_values }
    }
}

/// Factory methods to create domains
pub fn hash_set_domain<T: Clone + Eq + Hash + Debug, I: IntoIterator<Item = T>>(
    values: I,
) -> HashSetDomain<T> {
    HashSetDomain::new(values)
}

pub fn btree_set_domain<T: Clone + Eq + Ord + Debug, I: IntoIterator<Item = T>>(
    values: I,
) -> BTreeSetDomain<T> {
    BTreeSetDomain::new(values)
}

pub fn vec_domain<T: Clone + Eq + Debug, I: IntoIterator<Item = T>>(values: I) -> VecDomain<T> {
    VecDomain::new(values)
}

pub fn sorted_vec_domain<T: Clone + Eq + Ord + Debug, I: IntoIterator<Item = T>>(
    values: I,
) -> SortedVecDomain<T> {
    SortedVecDomain::new(values)
}
