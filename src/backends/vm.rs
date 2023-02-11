use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::num::Wrapping;
use std::ops::Add;

#[feature(wrapping_int_impl)]

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
                Expression::Clear => {
                    opcodes.push(Opcode::Clear);
                }
                Expression::MulVal(offset, val) => opcodes.push(Opcode::MulVal(*offset, *val)),
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
        let mut indent: usize = 0;

        for (index, opcode) in self.0.iter().enumerate() {
            write!(f, "{index:0>5}")?;

            Opcodes::fmt_with_indent(&mut indent, opcode, f)?;
        }

        write!(f, "")
    }
}

const VM_MEMORY_LENGTH: usize = 30_000;

//type VmCell = u8;
//type VmMemory = [VmCell; VM_MEMORY_LENGTH];

type VmCell<T> = std::num::Wrapping<T>;
type VmPointer = usize;

#[derive(Debug)]
struct VmMemory<C : Debug + Default, const N: usize = 30_000>
{
    pointer: VmPointer,
    cells: [VmCell<C>; N],
}

impl<C: Debug + Default, const N: usize> Default for VmMemory<C, N> {
    fn default() -> Self {
        let cells: [Wrapping<C>; N] = Default::default();
        Self {
            cells,
            pointer: 0
        }
    }
}


impl<C: Debug + Default, const N: usize> VmMemory<C, N>
{
    fn add(&mut self, amount: C) {
        let a = &self.cells[self.pointer];
    }
}

type VmDefaultMemory = VmMemory<u8, 30_000>;

//ToDo change that the memory can be any size maybe a vec?
struct VmState {
    index: usize,
    pointer: usize,
    memory: VmDefaultMemory,
}

/*
impl VmState {
    pub fn set_index(&self, index: usize) -> Self {
        let mut other = self.clone();
        other.index = index;

        other
    }

    pub fn set_pointer(&self, pointer: usize) -> Self {
        let mut other = self.clone();
        other.pointer = pointer;

        other
    }

    pub fn set_memory_by_index_value(&self, index: usize, cell: u8) -> Self {
        todo!()
    }
}
*/

impl Default for VmState {
    fn default() -> Self {
        todo!();
        /*
        Self {
            index: 0,
            pointer: 0,
            memory: VmMemory<u8, 30_000>;
        }
         */
    }
}

#[derive(Debug, Default)]
pub struct Vm {
    pub opcodes: Vec<Opcode>,
    index: usize,
    pointer: usize,
    memory: VmDefaultMemory,
}

impl Vm {
    pub fn from(opcodes: &[Opcode]) -> Self {
        todo!();
        /*
        Self {
            pointer: 100,
            index: 0,
            opcodes: opcodes.to_vec(),
            memory: VmDefaultMemory::default(),
        }

         */
    }
    pub fn run(&mut self) {
        while self.step().is_some() {}
    }

    pub fn step(&mut self) -> Option<()> {
        /*
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
         */
        Some(())
    }
}


struct VmBreakpoint(usize);
type VmBreakpoints = HashSet<VmBreakpoint>;

#[derive(Default)]
struct Debugger {
    vm: Vm,
    states: Vec<VmState>,
    breakpoints: VmBreakpoints,
}

impl Debugger {
    fn new(vm: Vm) -> Debugger {
        Self {
            vm,
            states: Vec::default(),
            ..Default::default()
        }
    }
}