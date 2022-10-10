use crate::ir::Expression;

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
                Expression::Input => unreachable!(),
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
    StartLoop(usize),
    EndLoop(usize),
    Print,
}

#[derive(Debug)]
pub struct Vm {
    opcodes: Vec<Opcode>,
    index: usize,
    pointer: usize,
    memory: [u8; 30000],
}

impl Vm {
    pub fn from(opcodes: &[Opcode]) -> Self {
        Self {
            pointer: 0,
            index: 0,
            opcodes: opcodes.to_vec(),
            memory: [0; 30_000],
        }
    }

    pub fn run(&mut self) {
        while let Some(_) = self.step() {}
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