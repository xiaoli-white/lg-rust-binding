use clone_dyn::clone_dyn;

use crate::ir::IRVisitor;
use crate::ir::base::IRNode;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum IRIntegerTypeSize {
    OneBit = 1,      // i1
    OneByte = 8,     // i8
    TwoBytes = 16,   // i16
    FourBytes = 32,  // i32
    EightBytes = 64, // i64
}

#[clone_dyn]
pub trait IRType: IRNode {}

#[derive(Clone)]
pub struct IRIntegerType {
    pub size: IRIntegerTypeSize,
    pub unsigned: bool,
}

impl IRIntegerType {
    pub fn new(size: IRIntegerTypeSize, unsigned: bool) -> Self {
        Self { size, unsigned }
    }
}
impl Display for IRIntegerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = if self.unsigned { "u" } else { "i" }.to_owned()
            + &(self.size.clone() as usize).to_string();
        write!(f, "{}", s)
    }
}
impl IRNode for IRIntegerType {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_integer_type(self);
    }
}
impl IRType for IRIntegerType {}
#[derive(Clone)]
pub struct IRFloatType {}

impl IRFloatType {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for IRFloatType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "float")
    }
}
impl IRNode for IRFloatType {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_float_type(self);
    }
}
impl IRType for IRFloatType {}
#[derive(Clone)]
pub struct IRDoubleType {}
impl IRDoubleType {
    pub fn new() -> Self {
        Self {}
    }
}
impl Display for IRDoubleType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "double")
    }
}
impl IRNode for IRDoubleType {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_double_type(self);
    }
}
impl IRType for IRDoubleType {}
#[derive(Clone)]
pub struct IRVoidType {}

impl IRVoidType {
    pub fn new() -> Self {
        Self {}
    }
}
impl Display for IRVoidType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "void")
    }
}

impl IRNode for IRVoidType {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_void_type(self);
    }
}
impl IRType for IRVoidType {}
#[derive(Clone)]
pub struct IRPointerType {
    pub base: Box<dyn IRType>,
}

impl IRPointerType {
    pub fn new(base: Box<dyn IRType>) -> Self {
        Self { base }
    }
}
impl Display for IRPointerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}*", self.base)
    }
}
impl IRNode for IRPointerType {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_pointer_type(self);
    }
}
impl IRType for IRPointerType {}
