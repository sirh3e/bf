use std::vec;

use crate::core::ir::Expression;

fn replace_last<T>(vec: &mut Vec<T>, expression: T) {
    vec.pop();
    vec.push(expression);
}

trait Optimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression>;
}

struct ClearOptimizer;

impl Optimizer for ClearOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let mut optimized: Vec<Expression> = vec![];

        for expression in expressions {
            match expression {
                Expression::Loop(expressions) => match expressions[..] {
                    [Expression::DecVal(1)] | [Expression::IncVal(1)] => {
                        optimized.push(Expression::Clear)
                    }
                    _ => {
                        let mut sub_optimized = vec![];
                        let sub_expressions = ClearOptimizer::optimize(expressions);
                        sub_optimized.extend(sub_expressions);

                        if !sub_optimized.is_empty() {
                            optimized.push(Expression::Loop(sub_optimized));
                        }
                    }
                },
                _ => {
                    optimized.push(expression.clone());
                }
            }
        }
        optimized
    }
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
                    optimized.push(Expression::Loop(Self::optimize_stage_01(expressions)))
                }
                (expression, _) => optimized.push(expression.clone()),
            }
        }
        optimized
    }

    fn optimize_stage_02(expressions: &[Expression]) -> Vec<Expression> {
        let _optimized: Vec<Expression> = vec![];
        //ToDo optimize [IncVal(5), DecVal(6)] -> [DecVal(1)]
        //ToDo optimize [IncVal(5), DecVal(4)] -> [IncVal(1)]
        //ToDo optimize [IncVal(5), DecVal(5)] -> ()
        //ToDo optimize [IncPtr(5), DecPtr(6)] -> [DecPtr(1)]
        //ToDo optimize [IncPtr(6), DecPtr(5)] -> [IncPtr(1)]
        //ToDo optimize [IncPtr(5), DecPtr(5)] -> ()
        //ToDo optimize []                     -> ()

        let mut optimized = vec![];
        for expression in expressions {
            match (expression, optimized.last()) {
                (Expression::IncVal(val), Some(&Expression::IncVal(amount))) => {
                    replace_last(&mut optimized, Expression::IncVal(amount + val))
                }
                (Expression::IncVal(val), Some(&Expression::DecVal(amount))) => {
                    match (val < &amount, &amount < val) {
                        (true, _) => {
                            let expression = Expression::DecVal(amount - val);
                            replace_last(&mut optimized, expression);
                        }
                        (_, true) => {
                            let expression = Expression::IncVal(val - amount);
                            replace_last(&mut optimized, expression);
                        }
                        _ => {}
                    }
                }
                (Expression::DecVal(val), Some(&Expression::DecVal(amount))) => {
                    replace_last(&mut optimized, Expression::DecVal(amount + val))
                }
                (Expression::DecVal(val), Some(&Expression::IncVal(amount))) => {
                    match (val < &amount, &amount < val) {
                        (true, _) => {
                            let expression = Expression::IncVal(amount - val);
                            replace_last(&mut optimized, expression);
                        }
                        (_, true) => {
                            let expression = Expression::DecVal(val - amount);
                            replace_last(&mut optimized, expression);
                        }
                        _ => {}
                    }
                }
                (Expression::IncPtr(val), Some(&Expression::IncPtr(amount))) => {
                    replace_last(&mut optimized, Expression::IncPtr(amount + val))
                }
                (Expression::IncPtr(val), Some(&Expression::DecPtr(amount))) => {
                    match (val < &amount, &amount < val) {
                        (true, _) => {
                            let expression = Expression::DecPtr(amount - val);
                            replace_last(&mut optimized, expression);
                        }
                        (_, true) => {
                            let expression = Expression::IncPtr(val - amount);
                            replace_last(&mut optimized, expression);
                        }
                        _ => {}
                    }
                }
                (Expression::DecPtr(val), Some(&Expression::DecPtr(amount))) => {
                    replace_last(&mut optimized, Expression::DecPtr(amount + val))
                }
                (Expression::DecPtr(val), Some(&Expression::IncPtr(amount))) => {
                    match (val < &amount, &amount < val) {
                        (true, _) => {
                            let expression = Expression::IncPtr(amount - val);
                            replace_last(&mut optimized, expression);
                        }
                        (_, true) => {
                            let expression = Expression::DecPtr(val - amount);
                            replace_last(&mut optimized, expression);
                        }
                        _ => {}
                    }
                }
                (Expression::Loop(expressions), _) => {
                    let sub_expressions = Self::optimize_stage_02(expressions);
                    if !sub_expressions.is_empty() {
                        optimized.push(Expression::Loop(Self::optimize_stage_02(&sub_expressions)))
                    }
                }
                (expression, _) => optimized.push(expression.clone()),
            }
        }
        optimized
    }
}

impl Optimizer for ConcatOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let expressions = ConcatOptimizer::optimize_stage_01(expressions);

        ConcatOptimizer::optimize_stage_02(&expressions)
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
    off_ptrs: Vec<isize>,
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
            off_ptrs: vec![],
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
        self.off_ptrs.push(-(offset as isize));
    }

    pub fn add_inc_ptrs(&mut self, offset: usize) {
        self.inc_ptrs.push(offset);
        self.off_ptrs.push(offset as isize);
    }

    pub fn set_side_effect(&mut self, is_side_effect: bool) {
        self.has_side_effect = is_side_effect;
    }

    fn is_valid(&self) -> bool {
        //ToDo check if this is enough

        let dec_ptrs_sum = self.dec_ptrs.iter().sum::<usize>();
        let inc_ptrs_sum = self.inc_ptrs.iter().sum::<usize>();

        match (
            self.has_side_effect,
            dec_ptrs_sum == inc_ptrs_sum,
            self.dec_vals.len() == 1,
            self.dec_vals.first(),
        ) {
            (false, true, true, Some(1)) => true,
            _ => false,
        }
    }

    pub fn generate_expressions(&self) -> Option<Vec<Expression>> {
        if self.is_valid().eq(&false) {
            return None;
        }

        let mut total_inc_offset = 0;
        let mut expressions = vec![];

        //println!("offset pointers: {:?}", self.off_ptrs);

        for (offset, val) in self.off_ptrs.iter().zip(&self.inc_vals) {
            total_inc_offset += offset;
            expressions.push(Expression::MulVal(total_inc_offset, *val));
        }
        expressions.push(Expression::Clear);
        Some(expressions)

        /*
        for (delta_inc_offset, val) in self.inc_ptrs.iter().zip(&self.inc_vals) {

            println!("{:?} {:?}", total_inc_offset, delta_inc_offset);

            total_inc_offset += delta_inc_offset;

            println!("{:?}", total_inc_offset);

            let expression = match val {
                1 => Expression::Copy(total_inc_offset),
                _ => Expression::MulVal(total_inc_offset, *val)
            };

            expressions.push(expression);
        }
        expressions.push(Expression::Clear);
        Some(expressions)
         */
    }
}

struct CopyOptimizer;

impl Optimizer for CopyOptimizer {
    fn optimize(expressions: &[Expression]) -> Vec<Expression> {
        let mut optimized = vec![];

        for expression in expressions {
            match expression {
                Expression::Loop(r#loop) => {
                    let mut loop_optimized = vec![];
                    let mut context =
                        CopyOptimizerContext::new(optimized.len().wrapping_sub(1) % usize::MAX);
                    for expression in r#loop {
                        match expression {
                            Expression::Copy(_) => {
                                loop_optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::Clear => {
                                loop_optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::IncVal(val) => {
                                loop_optimized.push(expression.clone());
                                context.add_inc_val(*val);
                            }
                            Expression::DecVal(val) => {
                                loop_optimized.push(expression.clone());
                                context.add_dec_val(*val);
                            }
                            Expression::MulVal(_, _) => {
                                loop_optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::IncPtr(val) => {
                                loop_optimized.push(expression.clone());
                                context.add_inc_ptrs(*val);
                            }
                            Expression::DecPtr(val) => {
                                loop_optimized.push(expression.clone());
                                context.add_dec_ptrs(*val);
                            }
                            Expression::Loop(r#loop) => {
                                loop_optimized
                                    .extend(Self::optimize(&[Expression::Loop(r#loop.clone())]));
                                context.set_side_effect(true);
                            }
                            Expression::Output => {
                                loop_optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                            Expression::Input => {
                                loop_optimized.push(expression.clone());
                                context.set_side_effect(true);
                            }
                        }
                    }

                    if let Some(expressions) = context.generate_expressions() {
                        optimized.extend(expressions);
                    } else {
                        optimized.push(Expression::Loop(loop_optimized))
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
        let expressions = ConcatOptimizer::optimize(expressions);
        let expressions = CopyOptimizer::optimize(&expressions);

        ClearOptimizer::optimize(&expressions)
    }
}

#[cfg(test)]
mod test {
    use crate::core::ir::optimizers::{ClearOptimizer, CopyOptimizer, Optimizer};
    use crate::core::ir::Expression;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(vec![Expression::Loop(vec![Expression::DecVal(1), Expression::IncPtr(1), Expression::IncVal(1), Expression::DecPtr(1)])], vec![Expression::Copy(1), Expression::Clear])]
    fn copy_optimizer(input: Vec<Expression>, excepted: Vec<Expression>) {
        let actual = CopyOptimizer::optimize(&input);
        assert_eq!(actual, excepted);
    }

    macro_rules! test_loop {
        ($expressions:expr) => {
            &vec![Expression::Loop($expressions)]
        };
    }

    macro_rules! test_expr {
        ($expressions:expr) => {
            &[Expression::Clear]
        };
    }

    #[test_case(test_loop!(vec![Expression::DecVal(1)]), test_expr!(Expression::Clear))]
    #[test_case(test_loop!(vec![Expression::IncVal(1)]), test_expr!(Expression::Clear))]
    #[test_case(test_loop!(vec![Expression::DecPtr(1)]), test_loop!(vec!(Expression::DecPtr(1))))]
    #[test_case(test_loop!(vec![Expression::IncPtr(1)]), test_loop!(vec!(Expression::IncPtr(1))))]
    #[test_case(test_loop!(vec![Expression::DecPtr(1), Expression::IncPtr(1)]), test_loop!(vec![Expression::DecPtr(1), Expression::IncPtr(1)]))]
    fn optimize_clear(expressions: &[Expression], should: &[Expression]) {
        let actual = ClearOptimizer::optimize(expressions);
        assert_eq!(actual, should);
    }
}
