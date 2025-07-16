use crate::ir::IRVisitor;
use crate::ir::base::{IRCondition, IRNode};
use crate::ir::operand::IROperand;
use crate::ir::types::IRType;
use std::fmt::{Display, Formatter, Result};

pub trait IRInstruction: IRNode {}

pub struct IRGoto {
    pub target: String,
}

impl IRGoto {
    pub fn new(target: String) -> Self {
        Self { target }
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
impl IRInstruction for IRGoto {}

pub struct IRConditionalJump {
    pub is_atomic: bool,
    pub _type: Box<dyn IRType>,
    pub condition: IRCondition,
    pub operand1: Box<dyn IROperand>,
    pub operand2: Box<dyn IROperand>,
    pub target: String,
}
impl IRConditionalJump {
    pub fn new(
        is_atomic: bool,
        _type: Box<dyn IRType>,
        condition: IRCondition,
        operand1: Box<dyn IROperand>,
        operand2: Box<dyn IROperand>,
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
impl Display for IRConditionalJump {
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

impl IRNode for IRConditionalJump {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRConditionalJump {}
pub struct IRNoOperate {}

impl IRNoOperate {
    pub fn new() -> Self {
        IRNoOperate {}
    }
}

impl Display for IRNoOperate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "nop")
    }
}

impl IRNode for IRNoOperate {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRNoOperate {}
pub struct IRReturn {
    pub operand: Box<dyn IROperand>,
}

impl IRReturn {
    pub fn new(operand: Box<dyn IROperand>) -> Self {
        IRReturn { operand }
    }
}
impl Display for IRReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "return {}", self.operand)
    }
}

impl IRNode for IRReturn {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRReturn {}
