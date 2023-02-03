pub mod optimizers;

use std::fmt::{write, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    IncVal(u8),
    DecVal(u8),
    IncPtr(usize),
    DecPtr(usize),
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
            Expression::Loop(expressions) => Expression::Loop(expressions.clone()),
            &Expression::Output => Expression::Output,
            &Expression::Input => Expression::Input,
            &Expression::Copy(offset) => Expression::Copy(offset),
            &Expression::Clear => Expression::Clear,
        }
    }
}
