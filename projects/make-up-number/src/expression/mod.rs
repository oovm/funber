use catalan::BinaryNode;

use crate::{ExpressionAction, ExpressionPool, NodeID};

trait BinaryExt {
    fn register_expression(
        &self,
        cache: &mut ExpressionPool,
        values: &mut Vec<usize>,
        actions: &mut Vec<ExpressionAction>,
    ) -> NodeID;
}
impl BinaryExt for BinaryNode {
    fn register_expression(
        &self,
        cache: &mut ExpressionPool,
        values: &mut Vec<usize>,
        actions: &mut Vec<ExpressionAction>,
    ) -> NodeID {
        match self {
            BinaryNode::Atomic => {
                let atom = values.remove(0);
                cache.insert_atomic(atom)
            }
            BinaryNode::Binary { lhs, rhs } => {
                let operator = actions[0].clone();
                let lhs = lhs.register_expression(cache, values, actions);
                let rhs = rhs.register_expression(cache, values, actions);
                cache.insert_binary(operator, lhs, rhs)
            }
        }
    }
}

impl ExpressionPool {
    pub fn register_binary_node(
        &mut self,
        node: &BinaryNode,
        mut values: Vec<usize>,
        mut actions: Vec<ExpressionAction>,
    ) -> NodeID {
        node.register_expression(self, &mut values, &mut actions)
    }
}
