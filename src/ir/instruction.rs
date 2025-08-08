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
        visitor.visit_conditional_jump(self);
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
        visitor.visit_no_operate(self);
    }
}
impl IRInstruction for IRNoOperate {}
pub struct IRReturn {
    pub operand: Option<Box<dyn IROperand>>,
}

impl IRReturn {
    pub fn new(operand: Option<Box<dyn IROperand>>) -> Self {
        IRReturn { operand }
    }
}
impl Display for IRReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(operand) = &self.operand {
            write!(f, "return {}", operand)
        } else {
            write!(f, "return")
        }
    }
}

impl IRNode for IRReturn {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_return(self);
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
        visitor.visit_malloc(self);
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
        visitor.visit_free(self);
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
        visitor.visit_realloc(self);
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
        visitor.visit_set(self);
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
        visitor.visit_get(self);
    }
}
impl IRInstruction for IRGet {}

pub struct IRSetVirtualRegister {
    pub source: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}

impl IRSetVirtualRegister {
    pub fn new(source: Box<dyn IROperand>, target: Box<IRVirtualRegister>) -> Self {
        Self { source, target }
    }
}
impl Display for IRSetVirtualRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.target, self.source)
    }
}
impl IRNode for IRSetVirtualRegister {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_set_virtual_register(self);
    }
}

impl IRInstruction for IRSetVirtualRegister {}

pub enum IRTypeCastKind {
    ZeroExtend,
    SignExtend,
    Truncate,
    IntToFloat,
    FloatToInt,
    FloatExtend,
    FloatTruncate,
}
impl Display for IRTypeCastKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            IRTypeCastKind::ZeroExtend => "zext",
            IRTypeCastKind::SignExtend => "sext",
            IRTypeCastKind::Truncate => "trunc",
            IRTypeCastKind::IntToFloat => "itof",
            IRTypeCastKind::FloatToInt => "ftoi",
            IRTypeCastKind::FloatExtend => "fext",
            IRTypeCastKind::FloatTruncate => "ftrunc",
        };
        write!(f, "{}", s)
    }
}
pub struct IRTypeCast {
    pub kind: IRTypeCastKind,
    pub original_type: Box<dyn IRType>,
    pub source: Box<dyn IROperand>,
    pub target_type: Box<dyn IRType>,
    pub target: Box<IRVirtualRegister>,
}
impl IRTypeCast {
    pub fn new(
        kind: IRTypeCastKind,
        original_type: Box<dyn IRType>,
        source: Box<dyn IROperand>,
        target_type: Box<dyn IRType>,
        target: Box<IRVirtualRegister>,
    ) -> Self {
        Self {
            kind,
            original_type,
            source,
            target_type,
            target,
        }
    }
}
impl Display for IRTypeCast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {} {} {} to {}",
            self.target, self.kind, self.original_type, self.source, self.target_type
        )
    }
}
impl IRNode for IRTypeCast {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_type_cast(self);
    }
}
impl IRInstruction for IRTypeCast {}

pub struct IRStackAllocate {
    pub size: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}
impl IRStackAllocate {
    pub fn new(size: Box<dyn IROperand>, target: Box<IRVirtualRegister>) -> Self {
        Self { size, target }
    }
}
impl Display for IRStackAllocate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = stack_alloc {}", self.target, self.size)
    }
}
impl IRNode for IRStackAllocate {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_stack_allocate(self);
    }
}
impl IRInstruction for IRStackAllocate {}
pub enum IRCalculateOperator {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    OR,
    XOR,
    SHL,
    SHR,
    USHR,
}
impl Display for IRCalculateOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            IRCalculateOperator::ADD => "add",
            IRCalculateOperator::SUB => "sub",
            IRCalculateOperator::MUL => "mul",
            IRCalculateOperator::DIV => "div",
            IRCalculateOperator::MOD => "mod",
            IRCalculateOperator::AND => "and",
            IRCalculateOperator::OR => "or",
            IRCalculateOperator::XOR => "xor",
            IRCalculateOperator::SHL => "shl",
            IRCalculateOperator::SHR => "shr",
            IRCalculateOperator::USHR => "ushr",
        };
        write!(f, "{}", s)
    }
}
pub struct IRCalculate {
    pub is_atomic: bool,
    pub operator: IRCalculateOperator,
    pub _type: Box<dyn IRType>,
    pub operand1: Box<dyn IROperand>,
    pub operand2: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}
impl IRCalculate {
    pub fn new(
        is_atomic: bool,
        operator: IRCalculateOperator,
        _type: Box<dyn IRType>,
        operand1: Box<dyn IROperand>,
        operand2: Box<dyn IROperand>,
        target: Box<IRVirtualRegister>,
    ) -> Self {
        Self {
            is_atomic,
            operator,
            _type,
            operand1,
            operand2,
            target,
        }
    }
}
impl Display for IRCalculate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {}{} {} {}, {}",
            self.target,
            if self.is_atomic { "atomic_" } else { "" },
            self.operator,
            self._type,
            self.operand1,
            self.operand2
        )
    }
}

impl IRNode for IRCalculate {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_calculate(self);
    }
}
impl IRInstruction for IRCalculate {}
pub struct IRIncrease {
    pub _type: Box<dyn IRType>,
    pub operand: Box<dyn IROperand>,
    pub target: Option<Box<IRVirtualRegister>>,
}
impl IRIncrease {
    pub fn new(
        _type: Box<dyn IRType>,
        operand: Box<dyn IROperand>,
        target: Option<Box<IRVirtualRegister>>,
    ) -> Self {
        Self {
            _type,
            operand,
            target,
        }
    }
}
impl Display for IRIncrease {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(target) = &self.target {
            write!(f, "{} = increase {} {}", target, self._type, self.operand)
        } else {
            write!(f, "atomic_increase {} {}", self._type, self.operand)
        }
    }
}
impl IRNode for IRIncrease {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_increase(self);
    }
}
impl IRInstruction for IRIncrease {}
pub struct IRDecrease {
    pub _type: Box<dyn IRType>,
    pub operand: Box<dyn IROperand>,
    pub target: Option<Box<IRVirtualRegister>>,
}
impl IRDecrease {
    pub fn new(
        _type: Box<dyn IRType>,
        operand: Box<dyn IROperand>,
        target: Option<Box<IRVirtualRegister>>,
    ) -> Self {
        Self {
            _type,
            operand,
            target,
        }
    }
}
impl Display for IRDecrease {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(target) = &self.target {
            write!(f, "{} = decrease {} {}", target, self._type, self.operand)
        } else {
            write!(f, "atomic_decrease {} {}", self._type, self.operand)
        }
    }
}
impl IRNode for IRDecrease {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_decrease(self);
    }
}
impl IRInstruction for IRDecrease {}
pub struct IRNot {
    pub is_atomic: bool,
    pub _type: Box<dyn IRType>,
    pub operand: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}
impl IRNot {
    pub fn new(
        is_atomic: bool,
        _type: Box<dyn IRType>,
        operand: Box<dyn IROperand>,
        target: Box<IRVirtualRegister>,
    ) -> Self {
        Self {
            is_atomic,
            _type,
            operand,
            target,
        }
    }
}
impl Display for IRNot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {}not {} {}",
            self.target,
            if self.is_atomic { "atomic_" } else { "" },
            self._type,
            self.operand
        )
    }
}
impl IRNode for IRNot {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_not(self);
    }
}
impl IRInstruction for IRNot {}
pub struct IRNegate {
    pub is_atomic: bool,
    pub _type: Box<dyn IRType>,
    pub operand: Box<dyn IROperand>,
    pub target: Box<IRVirtualRegister>,
}
impl IRNegate {
    pub fn new(
        is_atomic: bool,
        _type: Box<dyn IRType>,
        operand: Box<dyn IROperand>,
        target: Box<IRVirtualRegister>,
    ) -> Self {
        Self {
            is_atomic,
            _type,
            operand,
            target,
        }
    }
}
impl Display for IRNegate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {}negate {} {}",
            self.target,
            if self.is_atomic { "atomic_" } else { "" },
            self._type,
            self.operand
        )
    }
}
impl IRNode for IRNegate {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_negate(self);
    }
}
impl IRInstruction for IRNegate {}
pub struct IRInvoke {
    pub return_type: Box<dyn IRType>,
    pub address: Box<dyn IROperand>,
    pub argument_types: Vec<Box<dyn IRType>>,
    pub arguments: Vec<Box<dyn IROperand>>,
    pub target: Option<Box<IRVirtualRegister>>,
}

impl IRInvoke {
    pub fn new(
        return_type: Box<dyn IRType>,
        address: Box<dyn IROperand>,
        argument_types: Vec<Box<dyn IRType>>,
        arguments: Vec<Box<dyn IROperand>>,
        target: Option<Box<IRVirtualRegister>>,
    ) -> Self {
        Self {
            return_type,
            address,
            argument_types,
            arguments,
            target,
        }
    }
}
impl Display for IRInvoke {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = if self.argument_types.len() == self.arguments.len() {
            self.argument_types
                .iter()
                .zip(self.arguments.iter())
                .map(|(t, a)| format!(", [{}, {}]", t, a))
                .collect::<String>()
        } else {
            panic!("argument types and arguments length mismatch")
        };
        if let Some(target) = &self.target {
            write!(
                f,
                "{} = invoke {} {}{}",
                target, self.return_type, self.address, s
            )
        } else {
            write!(f, "invoke {} {}{}", self.return_type, self.address, s)
        }
    }
}

impl IRNode for IRInvoke {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_invoke(self);
    }
}
impl IRInstruction for IRInvoke {}

pub struct IRAsm {
    pub code: String,
    pub types: Vec<Box<dyn IRType>>,
    pub resources: Vec<Box<dyn IROperand>>,
    pub names: Vec<String>,
}
impl IRAsm {
    pub fn new(
        code: String,
        types: Vec<Box<dyn IRType>>,
        resources: Vec<Box<dyn IROperand>>,
        names: Vec<String>,
    ) -> Self {
        Self {
            code,
            types,
            resources,
            names,
        }
    }
}
impl Display for IRAsm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self
            .types
            .iter()
            .zip(self.resources.iter())
            .zip(self.names.iter())
            .map(|((t, r), n)| format!(", [{}, {}, {}]", t, r, n))
            .collect::<String>();
        write!(f, "asm \"{}\"{}", self.code, s)
    }
}
impl IRNode for IRAsm {
    fn accept(&self, visitor: &dyn IRVisitor) {
        visitor.visit_asm(self);
    }
}
impl IRInstruction for IRAsm {}
