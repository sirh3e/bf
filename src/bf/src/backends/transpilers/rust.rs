use crate::{backends, core::ir::Expression};

pub struct Transpiler;

const POINTER: &str = "pointer";
const MEMORY: &str = "memory";

const RUNTIME: &str = include_str!("../../runtimes/runtime.rs");

impl Transpiler {
    fn do_transpile(depth: usize, expressions: &[Expression]) -> String {
        let mut buffer = String::with_capacity(1024);

        for expression in expressions {
            for _ in 0..depth {
                buffer.push('\t')
            }

            match expression {
                Expression::IncVal(amount) => {
                    buffer.push_str(&format!("inc_val_by!({MEMORY}, {POINTER}, {amount})"));
                }
                Expression::DecVal(amount) => {
                    buffer.push_str(&format!("dec_val_by!({MEMORY}, {POINTER}, {amount})"));
                }
                Expression::IncPtr(amount) => {
                    buffer.push_str(&format!("inc_ptr_by!({POINTER}, {amount})"));
                }
                Expression::DecPtr(amount) => {
                    buffer.push_str(&format!("dec_ptr_by!({POINTER}, {amount})"));
                }
                Expression::Loop(expression) => {
                    buffer.push_str(&format!("r#loop!({MEMORY}, {POINTER},\n"));
                    buffer.push_str(&Self::do_transpile(depth + 1, expression));

                    buffer.remove(buffer.len() - 1);
                    buffer.remove(buffer.len() - 1);
                    buffer.push('\n');

                    for _ in 0..depth {
                        buffer.push('\t');
                    }
                    buffer.push(')');
                }
                Expression::Output => {
                    buffer.push_str(&format!("output!({MEMORY}, {POINTER})"));
                }
                Expression::Input => {}
                Expression::Clear => {
                    buffer.push_str(&format!("clear!({MEMORY}, {POINTER})"));
                }
                Expression::MulVal(offset, amount) => {
                    buffer.push_str(&format!(
                        "mul_val_by!({MEMORY}, {POINTER}, {offset}, {amount})"
                    ));
                }
            }

            let text = match depth {
                1 => ";\n",
                _ => ",\n",
            };
            buffer.push_str(text);
        }
        buffer
    }
}

impl backends::transpilers::Transpiler for Transpiler {
    fn transpile(expressions: &[Expression]) -> String {
        let code = Self::do_transpile(1, expressions);
        RUNTIME
            .replace("<POINTER>", POINTER)
            .replace("<MEMORY>", MEMORY)
            .replace("<CODE>", &code)
    }
}
