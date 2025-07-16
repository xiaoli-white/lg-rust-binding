use crate::ir::IRVisitor;
use std::fmt;

pub trait IRNode: fmt::Display {
    fn accept(&self, visitor: &dyn IRVisitor);
}

pub enum IRCondition {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
impl fmt::Display for IRCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            IRCondition::Equal => "e".to_string(),
            IRCondition::NotEqual => "ne".to_string(),
            IRCondition::Less => "l".to_string(),
            IRCondition::LessEqual => "le".to_string(),
            IRCondition::Greater => "g".to_string(),
            IRCondition::GreaterEqual => "ge".to_string(),
        };
        write!(f, "{}", s)
    }
}
