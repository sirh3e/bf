pub mod optimizers;

use std::fmt::{write, Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Expression {
    IncVal(u8),
    DecVal(u8),
    IncPtr(usize),
    DecPtr(usize),
    Copy(usize),
    Clear,
    Loop(Vec<Expression>),
    Output,
    Input,
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            &Expression::IncVal(amount) => Expression::IncVal(amount),
            &Expression::DecVal(amount) => Expression::DecVal(amount),
            &Expression::IncPtr(amount) => Expression::IncPtr(amount),
            &Expression::DecPtr(amount) => Expression::DecPtr(amount),
            Expression::Loop(expressions) => Expression::Loop(expressions.clone()),
            &Expression::Output => Expression::Output,
            &Expression::Input => Expression::Input,
            &Expression::Copy(offset) => Expression::Copy(offset),
            &Expression::Clear => Expression::Clear,
        }
    }
}

fn replace_last<T>(vec: &mut Vec<T>, expression: T) {
    vec.pop();
    vec.push(expression);
}

pub struct Optimizer;

impl Optimizer {
    pub fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let mut optimized = vec![];

        //ToDo optimize [IncVal(5), DecVal(6)] -> [DecVal(1)]
        //ToDo optimize [IncVal(5), DecVal(4)] -> [IncVal(1)]
        //ToDo optimize [IncVal(5), DecVal(5)] -> ()
        //ToDo optimize [IncPtr(5), DecPtr(6)] -> [DecPtr(1)]
        //ToDo optimize [IncPtr(6), DecPtr(5)] -> [IncPtr(1)]
        //ToDo optimize [IncPtr(5), DecPtr(5)] -> ()
        //ToDo optimize []                     -> ()

        for expression in expressions {
            match (expression, optimized.last()) {
                (Expression::IncVal(1), Some(&Expression::IncVal(amount))) => {
                    replace_last(&mut optimized, Expression::IncVal(amount + 1))
                }
                (Expression::DecVal(1), Some(&Expression::DecVal(amount))) => {
                    replace_last(&mut optimized, Expression::DecVal(amount + 1))
                }
                (Expression::IncPtr(1), Some(&Expression::IncPtr(amount))) => {
                    replace_last(&mut optimized, Expression::IncPtr(amount + 1))
                }
                (Expression::DecPtr(1), Some(&Expression::DecPtr(amount))) => {
                    replace_last(&mut optimized, Expression::DecPtr(amount + 1))
                }
                (Expression::Loop(expressions), _) => {
                    optimized.push(Expression::Loop(Self::optimize(expressions)))
                }
                (expression, _) => optimized.push(expression.clone()),
            }
        }

        optimized
    }
}

trait Optimize {
    fn optimize(expressions: &[Expression]) -> Vec<Expression>;
}

struct MemsetOptimizer;

impl Optimize for MemsetOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let mut new_expressions = vec![];

        for expression in expressions {
            match expression {
                Expression::Loop(expressions) => match expressions[..] {
                    [Expression::DecVal(1)] | [Expression::IncVal(1)] => {
                        new_expressions.push(Expression::Memset)
                    }
                    _ => new_expressions.push(expression.clone()),
                },
                _ => {
                    new_expressions.push(expression.clone());
                }
            }
        }
        new_expressions
    }
}

#[cfg(test)]
mod test {
    use crate::core::ir::{Expression, MemsetOptimizer, Optimize};
    use pretty_assertions::{assert_eq, assert_ne};
    use test_case::test_case;

    macro_rules! test_loop {
        ($expressions:expr) => {
            &vec![Expression::Loop($expressions)]
        };
    }

    macro_rules! test_expr {
        ($expressions:expr) => {
            &vec![$expressions]
        };
    }

    #[test_case(test_loop!(vec![Expression::DecVal(1)]), test_expr!(Expression::Memset))]
    #[test_case(test_loop!(vec![Expression::IncVal(1)]), test_expr!(Expression::Memset))]
    #[test_case(test_loop!(vec![Expression::DecPtr(1)]), test_loop!(vec!(Expression::DecPtr(1))))]
    #[test_case(test_loop!(vec![Expression::IncPtr(1)]), test_loop!(vec!(Expression::IncPtr(1))))]
    #[test_case(test_loop!(vec![Expression::DecPtr(1), Expression::IncPtr(1)]), test_loop!(vec!(Expression::DecPtr(1), Expression::IncPtr(1))))]
    fn optimize_memset(expressions: &[Expression], should: &[Expression]) {
        let actual = MemsetOptimizer::optimize(expressions);

        assert_eq!(actual, should);
    }
}
