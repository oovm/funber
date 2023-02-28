use make_up_number::{evaluate, ArithmeticAction, ArithmeticTraverse, ExpressionNode, ExpressionPool};

fn main() {
    // 3 items
    let mut items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut pool = ExpressionPool::default();
    let tasks = ArithmeticTraverse::new(items);
    // 2 actions
    for task in tasks {
        let mut lhs = pool.initial(task.first);
        for (op, num) in task.items {
            let rhs = pool.initial(num);
            match op {
                ArithmeticAction::Concat => {
                    lhs = pool.expression(ExpressionNode::Concat { lhs, rhs });
                }
                ArithmeticAction::Add => {
                    lhs = pool.expression(ExpressionNode::Add { lhs, rhs });
                }
                ArithmeticAction::Sub => {
                    lhs = pool.expression(ExpressionNode::Sub { lhs, rhs });
                }
                ArithmeticAction::Mul => {
                    lhs = pool.expression(ExpressionNode::Mul { lhs, rhs });
                }
                ArithmeticAction::Div => {
                    lhs = pool.expression(ExpressionNode::Div { lhs, rhs });
                }
            }
        }
        match evaluate(lhs, &mut pool) {
            Ok(o) => {
                let mut expr = String::new();
                pool.rewrite(&lhs, &mut expr).unwrap();
                println!("{:020} = {:?}", expr, o);
            }
            Err(_) => {}
        }
    }
}
