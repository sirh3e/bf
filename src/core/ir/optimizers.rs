use crate::core::ir::Expression;

fn replace_last<T>(vec: &mut Vec<T>, expression: T) {
    vec.pop();
    vec.push(expression);
}

trait Optimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression>;
}

struct ConcatOptimizer;

impl ConcatOptimizer {
    fn optimize_stage_01(expressions: &[Expression]) -> Vec<Expression> {
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
    fn optimize_stage_02(expressions: &[Expression]) -> Vec<Expression> {
        let mut optimized = vec![];
        //ToDo optimize [IncVal(5), DecVal(6)] -> [DecVal(1)]
        //ToDo optimize [IncVal(5), DecVal(4)] -> [IncVal(1)]
        //ToDo optimize [IncVal(5), DecVal(5)] -> ()
        //ToDo optimize [IncPtr(5), DecPtr(6)] -> [DecPtr(1)]
        //ToDo optimize [IncPtr(6), DecPtr(5)] -> [IncPtr(1)]
        //ToDo optimize [IncPtr(5), DecPtr(5)] -> ()
        //ToDo optimize []                     -> ()

        for expr in expressions.chunks(2) {
            match expr {
                [Expression::DecVal(lhs), Expression::DecVal(rhs)] => {
                    optimized.push(Expression::DecVal(lhs + rhs))
                }
                [Expression::DecVal(lhs), Expression::IncVal(rhs)] => {
                    if lhs < rhs {
                        optimized.push(Expression::IncVal(rhs - lhs))
                    } else if lhs > rhs {
                        optimized.push(Expression::DecVal(lhs - rhs))
                    }
                }
                [Expression::IncVal(lhs), Expression::DecVal(rhs)] => {
                    if lhs < rhs {
                        optimized.push(Expression::DecVal(rhs - lhs))
                    } else if lhs > rhs {
                        optimized.push(Expression::IncVal(lhs - rhs))
                    }
                }
                [Expression::IncVal(lhs), Expression::IncVal(rhs)] => {
                    optimized.push(Expression::IncVal(lhs + rhs))
                }
                [Expression::Loop(lhs_expressions), Expression::Loop(rhs_expressions)] => {
                    optimized.push(Expression::Loop(Self::optimize_stage_02(lhs_expressions)));
                    optimized.push(Expression::Loop(Self::optimize_stage_02(rhs_expressions)));
                }
                [Expression::Loop(expressions), expression] => {
                    optimized.push(Expression::Loop(Self::optimize_stage_02(expressions)));
                    optimized.push(expression.clone()); //ToDo not sure loop inc 1 dec 1 => should be 0
                }
                [Expression::Loop(expressions)] => {
                    optimized.push(Expression::Loop(Self::optimize_stage_02(expressions)))
                }
                _ => optimized.extend(Vec::from(expressions)),
            }
        }

        optimized
    }
}

impl Optimizer for ConcatOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let expressions = ConcatOptimizer::optimize_stage_01(expressions);
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
