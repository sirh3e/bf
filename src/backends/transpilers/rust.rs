use crate::core::ir::Expression;

pub struct Transpiler;

const POINTER: &str = "pointer";
const MEMORY: &str = "memory";

const RUNTIME: &str = "
macro_rules! inc_val_by {
    ($memory:expr, $index:expr, $amount:expr) => {
        $memory[$index] = $memory[$index].wrapping_add($amount);
    };
}

macro_rules! dec_val_by {
    ($memory:expr, $index:expr, $amount:expr) => {
        $memory[$index] = $memory[$index].wrapping_sub($amount);
    };
}

macro_rules! mul_val_by {
    ($memory:expr, $index:expr, $offset:expr, $amount:expr) => {
        let offset = $index.checked_add_signed($offset).unwrap();

        $memory[offset] = $memory[offset].wrapping_add($memory[$index].wrapping_mul($amount));
    };
}

macro_rules! inc_ptr_by {
    ($pointer:expr, $amount:expr) => {
        $pointer += $amount
    };
}

macro_rules! dec_ptr_by {
    ($pointer:expr, $amount:expr) => {
        $pointer -= $amount
    };
}

macro_rules! r#loop {
     ($memory:expr, $index:expr, $( $expression:expr ),*) => {
        while $memory[$index] != 0 {
         $(
             $expression;
         )*
        }
     };
}

macro_rules! clear {
    ($memory:expr, $index:expr) => {
		$memory[$index] = 0;
    };
}

macro_rules! output {
    ($memory:expr, $pointer:expr) => {
        print!(\"{}\", $memory[$pointer] as char);
    };
}

fn main() {
    let mut <POINTER> = 100 as usize;
    let mut <MEMORY> = [0 as u8; 30_000];

<CODE>
}";

impl Transpiler {
    pub fn transpile(expressions: &[Expression]) -> String {
        let code = Self::do_transpile(1, expressions);
        RUNTIME
            .replace("<POINTER>", POINTER)
            .replace("<MEMORY>", MEMORY)
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
