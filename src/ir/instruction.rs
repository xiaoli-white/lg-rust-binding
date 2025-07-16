use crate::ir::base::{IRCondition, IRNode};
use crate::ir::operand::IROperand;
use crate::ir::types::IRType;
use crate::ir::IRVisitor;
use std::fmt::{Display, Formatter, Result};

pub struct IRGoto {
    pub target: String,
}

impl IRGoto {
    pub fn new(target: String) -> IRGoto {
        IRGoto { target }
    }
}

impl Display for IRGoto {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "goto {}", self.target)
    }
}

impl IRNode for IRGoto {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_goto(self);
    }
}

pub struct IRConditionalJump<'a> {
    pub is_atomic: bool,
    pub _type: &'a dyn IRType,
    pub condition: IRCondition,
    pub operand1: &'a dyn IROperand,
    pub operand2: &'a dyn IROperand,
    pub target: String,
}
impl<'a> IRConditionalJump<'a> {
    pub fn new(
        is_atomic: bool,
        _type: &'a dyn IRType,
        condition: IRCondition,
        operand1: &'a dyn IROperand,
        operand2: &'a dyn IROperand,
        target: String,
    ) -> Self {
        Self {
            is_atomic,
            _type,
            condition,
            operand1,
            operand2,
            target,
        }
    }
}
impl Display for IRConditionalJump<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s = (if self.is_atomic { "atomic_" } else { "" }).to_string()
            + &format!(
                "conditional_jump {} {}, {}, {}, #{}",
                self._type.to_string(),
                self.condition,
                self.operand1.to_string(),
                self.operand2.to_string(),
                self.target
            );
        write!(f, "{}", s)
    }
}

impl IRNode for IRConditionalJump<'_> {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
