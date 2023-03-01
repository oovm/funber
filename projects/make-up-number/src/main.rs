#![feature(generators)]

use catalan::{FullBinaryTrees, OperatorPermutation};
use gen_iter::GenIter;
use itertools::Itertools;

use make_up_number::{evaluate, ExpressionAction, ExpressionPool, NodeID};

#[derive(Default)]
pub struct ExpressionCache {
    pool: ExpressionPool,
    catalan: FullBinaryTrees,
    values: Vec<usize>,
    operators: Vec<ExpressionAction>,
}

impl ExpressionCache {
    pub fn task(&mut self, values: Vec<usize>, operators: Vec<ExpressionAction>) -> Self {
        Self { pool: ExpressionPool::default(), catalan: FullBinaryTrees::default(), values, operators }
    }
    pub fn build(&mut self) -> impl Iterator<Item = NodeID> + '_ {
        GenIter(move || {
            for tree in self.catalan.build_trees(self.values.len()) {
                for operator in OperatorPermutation::new(&self.operators, self.values.len() - 1) {
                    yield self.pool.register_binary_node(&tree, self.values.clone(), operator);
                }
            }
        })
    }
}

fn main() {
    // 3 items
    let mut cache = ExpressionCache::default();
    let values = vec![1, 2, 3, 4];
    let operators = vec![ExpressionAction::Plus, ExpressionAction::Minus, ExpressionAction::Divide];
    for task in cache.task(values, operators).build() {
        println!("{:?}", task);
    }
}
