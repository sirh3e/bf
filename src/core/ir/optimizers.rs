use crate::core::ir::{optimizers, Expression};
use std::char::MAX;

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

#[derive(Debug)]
struct CopyOptimizerContext {
    start_position: usize,
    has_side_effect: bool,
    dec_vals: Vec<u8>,
    inc_vals: Vec<u8>,
    dec_ptrs: Vec<usize>,
    inc_ptrs: Vec<usize>,
}

impl CopyOptimizerContext {
    pub fn new(start_position: usize) -> Self {
        Self {
            start_position,
            has_side_effect: false,
            dec_vals: vec![],
            inc_vals: vec![],
            dec_ptrs: vec![],
            inc_ptrs: vec![],
        }
    }

    pub fn add_dec_val(&mut self, amount: u8) {
        self.dec_vals.push(amount);
    }

    pub fn add_inc_val(&mut self, amount: u8) {
        self.inc_vals.push(amount);
    }

    pub fn add_dec_ptrs(&mut self, offset: usize) {
        self.dec_ptrs.push(offset);
    }

    pub fn add_inc_ptrs(&mut self, offset: usize) {
        self.inc_ptrs.push(offset);
    }

    pub fn set_side_effect(&mut self, is_side_effect: bool) {
        self.has_side_effect = is_side_effect;
    }

    fn is_valid(&self) -> bool {
        if self.has_side_effect {
            return false;
        }

        //ToDo check if this is enough

        let dec_ptrs_sum = self.dec_ptrs.iter().sum::<usize>();
        let inc_ptrs_sum = self.inc_ptrs.iter().sum::<usize>();

        dec_ptrs_sum == inc_ptrs_sum
    }

    pub fn generate_expressions(&self) -> Option<Vec<Expression>> {
        if self.is_valid().eq(&false) {
            return None;
        }

        let mut total_inc_offset = 0;
        let mut expressions = vec![];

        for delta_inc_offset in &self.inc_ptrs {
            total_inc_offset += delta_inc_offset;
            expressions.push(Expression::Copy(total_inc_offset));
        }
        expressions.push(Expression::Clear);
        Some(expressions)
    }
}

struct CopyOptimizer;

impl Optimizer for CopyOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let mut optimized = vec![];

        for expression in expressions {
            match expression {
                Expression::Loop(r#loop) => {
                    let mut context =
                        CopyOptimizerContext::new(optimized.len().wrapping_sub(1) % usize::MAX);
                    for expression in r#loop {
                        match expression {
                            Expression::Copy(_) => {
                                optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::Clear => {
                                optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::IncVal(val) => {
                                optimized.push(expression.clone());
                                context.add_inc_val(*val);
                            }
                            Expression::DecVal(val) => {
                                optimized.push(expression.clone());
                                context.add_dec_val(*val);
                            }
                            Expression::IncPtr(val) => {
                                optimized.push(expression.clone());
                                context.add_inc_ptrs(*val);
                            }
                            Expression::DecPtr(val) => {
                                optimized.push(expression.clone());
                                context.add_dec_ptrs(*val);
                            }
                            Expression::Loop(r#loop) => {
                                optimized.push(Expression::Loop(Self::optimize(r#loop)));
                                context.set_side_effect(true);
                            }
                            Expression::Output => {
                                optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::Input => {
                                optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                        }
                    }

                    if let Some(expressions) = context.generate_expressions() {
                        let _ = optimized.split_off(context.start_position);
                        optimized.push(Expression::Loop(expressions));
                    }
                }
                _ => {
                    optimized.push(expression.clone());
                }
            }
        }

        optimized
    }
}

pub struct Optimizers;

impl Optimizers {
    pub fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        ConcatOptimizer::optimize(expressions)
    }
}

#[cfg(test)]
mod test {
    use crate::core::ir::optimizers::{CopyOptimizer, Optimizer};
    use crate::core::ir::Expression;
    use pretty_assertions::{assert_eq, assert_ne};
    use test_case::test_case;

    #[test_case(vec![Expression::Loop(vec![Expression::DecVal(1), Expression::IncPtr(1), Expression::IncVal(1), Expression::DecPtr(1)])], vec![Expression::Loop(vec!(Expression::Copy(1), Expression::Clear))])]
    fn copy_optimizer(input: Vec<Expression>, excepted: Vec<Expression>) {
        let actual = CopyOptimizer::optimize(&input);
        assert_eq!(actual, excepted);
    }
}
