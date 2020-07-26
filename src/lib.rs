// use arrow::datatypes::{DataType, Field, Schema};
use std::collections::BTreeSet;

pub mod logical_nodes;
pub mod optimization_rules;
pub mod planning;

#[derive(PartialEq, Debug)]
pub struct Cols(BTreeSet<usize>);

impl Cols {
    pub fn new(indices: Vec<usize>) -> Self {
        let mut set = BTreeSet::new();
        for i in indices {
            set.insert(i);
        }
        Cols(set)
    }

    pub fn intersection(&self, other: &Cols) -> Cols {
        Cols::new(self.0.intersection(&other.0).copied().collect())
    }

    pub fn to_string(&self) -> String {
        let index_strs: Vec<String> = self.0.iter().map(|i| i.to_string()).collect();
        index_strs.join(",")
    }

    pub fn contains(&self, col: &usize) -> bool {
        self.0.contains(col)
    }

    pub fn is_subset(&self, other: &Cols) -> bool {
        self.0.is_subset(&other.0)
    }
}
