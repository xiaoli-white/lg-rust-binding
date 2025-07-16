use crate::ir::IRVisitor;
use crate::ir::base::{IRCondition, IRNode};
use crate::ir::operand::{IROperand, IRVirtualRegister};
use crate::ir::types::IRType;
use std::fmt;
use std::fmt::{Display, Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "return {}", self.operand)
    }
}

impl IRNode for IRReturn {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRReturn {}

pub struct IRMalloc {
    pub size: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}
impl IRMalloc {
    pub fn new(size: Box<dyn IROperand>, target: Box<IRVirtualRegister>) -> Self {
        IRMalloc { size, target }
    }
}

impl Display for IRMalloc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = malloc {}", self.target, self.size)
    }
}
impl IRNode for IRMalloc {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRMalloc {}

pub struct IRFree {
    pub ptr: Box<dyn IROperand>,
}

impl IRFree {
    pub fn new(ptr: Box<dyn IROperand>) -> Self {
        IRFree { ptr }
    }
}
impl Display for IRFree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "free {}", self.ptr)
    }
}
impl IRNode for IRFree {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRFree {}

pub struct IRRealloc {
    pub ptr: Box<dyn IROperand>,
    pub size: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}

impl IRRealloc {
    pub fn new(
        ptr: Box<dyn IROperand>,
        size: Box<dyn IROperand>,
        target: Box<IRVirtualRegister>,
    ) -> Self {
        Self { ptr, size, target }
    }
}
impl Display for IRRealloc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = realloc {}, {}", self.target, self.ptr, self.size)
    }
}

impl IRNode for IRRealloc {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRRealloc {}

pub struct IRSet {
    pub _type: Box<dyn IRType>,
    pub address: Box<dyn IROperand>,
    pub value: Box<dyn IROperand>,
}
impl IRSet {
    pub fn new(
        _type: Box<dyn IRType>,
        address: Box<dyn IROperand>,
        value: Box<dyn IROperand>,
    ) -> Self {
        Self {
            _type,
            address,
            value,
        }
    }
}
impl Display for IRSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "set {}, {}, {}", self._type, self.address, self.value)
    }
}

impl IRNode for IRSet {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}

impl IRInstruction for IRSet {}

pub struct IRGet {
    pub _type: Box<dyn IRType>,
    pub address: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}
impl IRGet {
    pub fn new(
        _type: Box<dyn IRType>,
        address: Box<dyn IROperand>,
        target: Box<IRVirtualRegister>,
    ) -> Self {
        Self {
            _type,
            address,
            target,
        }
    }
}
impl Display for IRGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = get {}, {}", self.target, self._type, self.address)
    }
}
impl IRNode for IRGet {
    fn accept(&self, visitor: &dyn IRVisitor) {
        todo!()
    }
}
impl IRInstruction for IRGet {}
