use crate::core::ir::Expression;

const MEMORY: &str = "MEMORY";
const POINTER: &str = "POINTER";

const RUNTIME: &str = "#include <stdio.h>

typedef unsigned char byte;
typedef unsigned long long usize;

#define MEMORY memory
#define MEMORY_LENGTH 30000
#define MEMORY_DEFINE \\
    byte MEMORY[MEMORY_LENGTH] = { 0 }

#define POINTER pointer
#define POINTER_DEFINE \\
    usize POINTER = 0

#define DEC_VAL_BY(amount) \\
    MEMORY[POINTER] -= amount

#define INC_VAL_BY(amount) \\
    MEMORY[POINTER] += amount

#define DEC_PTR_BY(amount) \\
    POINTER -= amount

#define INC_PTR_BY(amount) \\
    POINTER += amount

#define CLEAR \\
    MEMORY[POINTER] = 0

#define MUL_VAL_BY(offset, amount) \\
    MEMORY[POINTER + offset] += MEMORY[POINTER] * amount

#define LOOP(expressions)         \\
    while(MEMORY[POINTER] != 0) { \\
        expressions               \\
    }

#define OUTPUT \\
    printf(\"%c\", MEMORY[POINTER])

int main() {
    POINTER_DEFINE;
    MEMORY_DEFINE;

    <CODE>

    return 0;
}";

pub struct Transpiler;

impl Transpiler {
    pub fn transpile(expressions: &[Expression]) -> String {
        let code = Self::do_transpile(1, expressions);
        RUNTIME.replace("<CODE>", &code)
    }

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

                Expression::Copy(_) => {
                    todo!()
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
