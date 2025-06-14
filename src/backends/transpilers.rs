use crate::core::ir::Expression;

pub mod c;
pub mod rust;

pub trait Transpiler {
    fn transpile(expressions: &[Expression]) -> String;
}
