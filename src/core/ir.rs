use std::fmt::{write, Display, Formatter};

#[derive(Debug)]
pub enum Expression {
    IncVal(u8),
    DecVal(u8),
    IncPtr(usize),
    DecPtr(usize),
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
        }
    }
}

fn replace_last<T>(vec: &mut Vec<T>, expression: T) {
    vec.pop();
    vec.push(expression);
}

trait Optimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression>;
}

struct ConcatOptimizer;

impl ConcatOptimizer {
    fn optimize_stage_01(expressions: &[Expression]) -> Vec<Expression>{
        let mut optimized = vec![];
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
                    optimized.push(Expression::Loop(Self::optimize_stage_01(&expressions)))
                }
                (expression, _) => optimized.push(expression.clone()),
            }
        }
        optimized
    }
    fn optimize_stage_02(expression: &[Expression]) -> Vec<Expression>{
        let mut optimized = vec![];
        //ToDo optimize [IncVal(5), DecVal(6)] -> [DecVal(1)]
        //ToDo optimize [IncVal(5), DecVal(4)] -> [IncVal(1)]
        //ToDo optimize [IncVal(5), DecVal(5)] -> ()
        //ToDo optimize [IncPtr(5), DecPtr(6)] -> [DecPtr(1)]
        //ToDo optimize [IncPtr(6), DecPtr(5)] -> [IncPtr(1)]
        //ToDo optimize [IncPtr(5), DecPtr(5)] -> ()
        //ToDo optimize []                     -> ()

        optimized
    }
}

impl Optimizer for ConcatOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let expressions = ConcatOptimizer::optimize_state_01(expressions);
        let expressions = ConcatOptimizer::optimize_stage_02(&expressions);
        expressions
    }
}

pub struct Optimizers;

impl Optimizers {
    pub fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        ConcatOptimizer::optimize(expressions)
    }
}
