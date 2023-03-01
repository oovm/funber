use catalan::{BinaryNode, FullBinaryTrees};
use itertools::Itertools;

use make_up_number::{ExpressionAction, ExpressionNode};

pub struct Arrangement {
    order: usize,
    length: usize,
    operators: Vec<ExpressionAction>,
}

impl Arrangement {
    pub fn new(mut operators: Vec<ExpressionAction>) -> Self {
        operators.dedup();
        Self { order: 0, length: operators.len(), operators }
    }
    pub fn with_order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }
}

impl Iterator for Arrangement {
    type Item = Vec<ExpressionAction>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.order >= self.operators.len().pow(self.length as u32) {
            return None;
        }
        let mut actions = vec![self.operators[0]; self.length];
        let mut pointer = self.order;
        for i in 0..actions.len() {
            actions[i] = self.operators[pointer % self.length];
            pointer /= self.length;
        }
        self.order += 1;
        Some(actions)
    }
}

fn main() {
    // 3 items
    let mut cache = FullBinaryTrees::default();
    let operators = Arrangement::new(vec![ExpressionAction::Add, ExpressionAction::Sub, ExpressionAction::Div]);
    for arrange in operators {
        println!("{:?}", arrange)
    }

    for tree in cache.build_trees(3) {
        match tree.as_ref() {
            BinaryNode::Atomic => {
                ExpressionNode::Atomic { number: 0 };
            }
            BinaryNode::Binary { lhs, rhs } => {
                ExpressionNode::Binary {
                    lhs: Box::new(ExpressionNode::Atomic { number: 0 }),
                    rhs: Box::new(ExpressionNode::Atomic { number: 0 }),
                    action: ExpressionAction::Add,
                };
            }
        }
    }

    // let mut items = vec![1, 2, 3, 4, 5, 6, 7];
    // let mut pool = ExpressionPool::default();
    // let tasks = ArithmeticTraverse::new(items);
    // 2 actions, full binary tree traversal
    // 1 + (2 + 3)
    // (1 + 2) + 3
    // for task in tasks {
    //     let mut lhs = pool.initial(task.first);
    // }
}
