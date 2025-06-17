use crate::{backends, core::ir::Expression};

const MEMORY: &str = "MEMORY";
const POINTER: &str = "POINTER";

const RUNTIME: &str = include_str!("../../runtimes/runtime.c");

pub struct Transpiler;

impl Transpiler {
    fn do_transpile(depth: usize, expressions: &[Expression]) -> String {
        let mut buffer = String::with_capacity(1024);

        for expression in expressions {
            for _ in 0..depth {
                buffer.push('\t')
            }

            match expression {
                Expression::IncVal(amount) => {
                    buffer.push_str(&format!("INC_VAL_BY({amount})"));
                }
                Expression::DecVal(amount) => {
                    buffer.push_str(&format!("DEC_VAL_BY({amount})"));
                }
                Expression::IncPtr(amount) => {
                    buffer.push_str(&format!("INC_PTR_BY({amount})"));
                }
                Expression::DecPtr(amount) => {
                    buffer.push_str(&format!("DEC_PTR_BY({amount})"));
                }
                Expression::Loop(expression) => {
                    buffer.push_str("LOOP(\n");
                    buffer.push_str(&Self::do_transpile(depth + 1, expression));

                    for _ in 0..depth {
                        buffer.push('\t');
                    }
                    buffer.push(')');
                }
                Expression::Output => {
                    buffer.push_str("OUTPUT");
                }
                Expression::Input => {
                    todo!()
                }

                Expression::Clear => {
                    buffer.push_str("CLEAR");
                }

                Expression::MulVal(offset, amount) => {
                    buffer.push_str(&format!("MUL_VAL_BY({offset}, {amount})"));
                }
            }

            buffer.push_str(";\n");
        }
        buffer
    }
}

impl backends::transpilers::Transpiler for Transpiler {
    fn transpile(expressions: &[Expression]) -> String {
        let code = Self::do_transpile(1, expressions);
        RUNTIME.replace("<CODE>", &code)
    }
}
