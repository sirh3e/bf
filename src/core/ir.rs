pub mod optimizers;

use std::fmt::{write, Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Expression {
    IncVal(u8),
    DecVal(u8),
    IncPtr(usize),
    DecPtr(usize),
    MulVal(isize, u8),
    Copy(usize),
    Clear,
    Loop(Vec<Expression>),
    Output,
    Input,
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            &Expression::IncVal(amount) => Expression::IncVal(amount),
            &Expression::DecVal(amount) => Expression::DecVal(amount),
            &Expression::IncPtr(amount) => Expression::IncPtr(amount),
            &Expression::DecPtr(amount) => Expression::DecPtr(amount),
            &Expression::MulVal(offset, amount) => Expression::MulVal(offset, amount),
            &Expression::Copy(offset) => Expression::Copy(offset),
            &Expression::Clear => Expression::Clear,
            Expression::Loop(expressions) => Expression::Loop(expressions.clone()),
            &Expression::Output => Expression::Output,
            &Expression::Input => Expression::Input,
        }
    }
}
