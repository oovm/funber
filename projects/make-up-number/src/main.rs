use dashu::integer::IBig;
use make_up_number::{ExpressionAction, ExpressionCache};

fn main() {
    // 3 items
    let mut cache = ExpressionCache::default();
    let values = vec![1, 1, 4, 5, 1, 4];
    let operators = vec![
        ExpressionAction::Concat,
        ExpressionAction::Plus,
        ExpressionAction::Minus,
        // ExpressionAction::Times,
        // ExpressionAction::Divide,
    ];
    cache.task(values, operators);
    for task in cache.sequence() {
        match cache.run_expression(task) {
            Ok(o) => {
                if o.eq(&IBig::from(10usize)) {
                    println!("{:#40} == {}", cache.get_display(task), o);
                }
            }
            Err(_) => continue,
        }
    }
}
