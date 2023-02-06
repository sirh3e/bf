use crate::core::ir::Expression;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(expressions: &[Expression]) -> Vec<Opcode> {
        Self::do_interpret(0, expressions)
    }

    fn do_interpret(offset: usize, expressions: &[Expression]) -> Vec<Opcode> {
        let mut opcodes = vec![];

        for expression in expressions {
            match expression {
                &Expression::IncVal(amount) => {
                    opcodes.push(Opcode::IncVal(amount));
                }
                &Expression::DecVal(amount) => {
                    opcodes.push(Opcode::DecVal(amount));
                }
                &Expression::IncPtr(amount) => {
                    opcodes.push(Opcode::IncPtr(amount));
                }
                &Expression::DecPtr(amount) => {
                    opcodes.push(Opcode::DecPtr(amount));
                }
                Expression::Loop(_expressions) => {
                    let start_index = offset + opcodes.len();
                    let _opcodes = Self::do_interpret(start_index + 1, _expressions);
                    let end_index = start_index + _opcodes.len() + 1;

                    opcodes.push(Opcode::StartLoop(end_index));
                    opcodes.extend(_opcodes);
                    opcodes.push(Opcode::EndLoop(start_index));
                }
                Expression::Output => {
                    opcodes.push(Opcode::Print);
                }
                Expression::Input => todo!(),
                Expression::Copy(offset) => {
                    opcodes.push(Opcode::Copy(*offset));
                }
                Expression::Clear => {
                    opcodes.push(Opcode::Clear);
                }
                Expression::MulVal(offset, val) => opcodes.push(Opcode::MulVal(*offset, *val)),
                _ => todo!(),
            };
        }

        opcodes
    }
}

#[derive(Clone, Debug)]
pub enum Opcode {
    DecVal(u8),
    IncVal(u8),
    DecPtr(usize),
    IncPtr(usize),
    MulVal(isize, u8),
    Copy(usize),
    Clear,
    StartLoop(usize),
    EndLoop(usize),
    Print,
}

pub struct Opcodes(pub Vec<Opcode>);

impl Opcodes {
    fn fmt_with_indent(
        indent: &mut usize,
        opcode: &Opcode,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        if let Opcode::EndLoop(_) = opcode {
            *indent -= 1;
        }

        for _ in 0..*indent {
            write!(f, "\t")?;
        }

        if let Opcode::StartLoop(_) = opcode {
            *indent += 1;
        }

        write!(f, " {}", &format!("{opcode:?}\n"))
    }
}

impl std::fmt::Display for Opcodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut index = 0;
        let mut indent: usize = 0;

        for opcode in &self.0 {
            write!(f, "{index:0>5}")?;
            index += 1;

            Opcodes::fmt_with_indent(&mut indent, opcode, f)?;
        }

        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Vm {
    pub opcodes: Vec<Opcode>,
    index: usize,
    pointer: usize,
    memory: [u8; 30_000],
}

impl Vm {
    pub fn from(opcodes: &[Opcode]) -> Self {
        Self {
            pointer: 100,
            index: 0,
            opcodes: opcodes.to_vec(),
            memory: [0; 30_000],
        }
    }
    pub fn run(&mut self) {
        while self.step().is_some() {}
    }

    pub fn step(&mut self) -> Option<()> {
        match self.opcodes.get(self.index) {
            None => return None,
            Some(opcode) => match opcode {
                Opcode::DecVal(amount) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*amount);
                    self.index += 1;
                }
                Opcode::IncVal(amount) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*amount);
                    self.index += 1;
                }
                Opcode::DecPtr(amount) => {
                    self.pointer -= amount;
                    self.index += 1;
                }
                Opcode::IncPtr(amount) => {
                    self.pointer += amount;
                    self.index += 1;
                }
                Opcode::MulVal(offset, val) => {
                    let offset = self.pointer.checked_add_signed(*offset)?;

                    self.memory[offset] = self.memory[offset]
                        .wrapping_add(self.memory[self.pointer].wrapping_mul(*val));
                    self.index += 1;
                }
                Opcode::Copy(offset) => {
                    self.memory[self.pointer + offset] =
                        self.memory[self.pointer + offset].wrapping_add(self.memory[self.pointer]);

                    self.index += 1;
                }
                Opcode::Clear => {
                    self.memory[self.pointer] = 0;

                    self.index += 1;
                }
                &Opcode::StartLoop(index) => {
                    let index = match self.memory[self.pointer] {
                        0 => index,
                        _ => self.index + 1,
                    };
                    self.index = index;
                }
                &Opcode::EndLoop(index) => {
                    let index = match self.memory[self.pointer] {
                        0 => self.index + 1,
                        _ => index,
                    };
                    self.index = index;
                }
                Opcode::Print => {
                    print!("{}", self.memory[self.pointer] as char); //ToDo reactor this
                    self.index += 1;
                }
            },
        };
        Some(())
    }
}
