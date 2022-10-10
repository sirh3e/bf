use crate::ir::Expression;

const MEMORY: &'static str = "MEMORY";
const POINTER: &'static str = "POINTER";

const RUNTIME: &'static str = "#include <stdio.h>

typedef unsigned char byte;
typedef unsigned long long usize;

#define MEMORY memory
#define MEMORY_LENGTH 30000
#define MEMORY_DEFINE \\
    byte MEMORY[MEMORY_LENGTH]

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

#define LOOP(expressions)         \\
    while(MEMORY[POINTER] != 0) { \\
        expressions              \\
    }

#define OUTPUT printf(\"%c\", MEMORY[POINTER])


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
        RUNTIME
            .replace("<CODE>", &code)
    }

    fn do_transpile(depth: usize, expressions: &[Expression]) -> String {
        let mut buffer = String::with_capacity(1024);

        for expression in expressions {
            for _ in 0..depth {
                buffer.push('\t')
            }

            match expression {
                Expression::IncVal(amount) => {
                    buffer.push_str(&format!("INC_VAL_BY({})", amount));
                }
                Expression::DecVal(amount) => {
                    buffer.push_str(&format!("DEC_VAL_BY({})", amount));
                }
                Expression::IncPtr(amount) => {
                    buffer.push_str(&format!("INC_PTR_BY({})", amount));
                }
                Expression::DecPtr(amount) => {
                    buffer.push_str(&format!("DEC_PTR_BY({})", amount));
                }
                Expression::Loop(expression) => {
                    buffer.push_str(&format!("LOOP(\n"));
                    buffer.push_str(&Self::do_transpile(depth + 1, &expression));

                    //buffer.remove(buffer.len() - 1);
                    //buffer.remove(buffer.len() - 1);

                    for _ in 0..depth {
                        buffer.push_str("\t");
                    }
                    buffer.push_str(")");
                }
                Expression::Output => {
                    buffer.push_str(&format!("OUTPUT"));
                }
                Expression::Input => {}
            }

            let text = match depth {
                1 => ";\n",
                _ => ";\n",
            };
            buffer.push_str(text);
        }
        buffer
    }
}
