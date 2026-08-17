//! Typed instructions, lowered from the generic [`crate::ast::Operation`]
//! tree. Every CIR op we interpret structurally gets its own [`Instruction`]
//! variant; anything else (atomics, inline asm, builtins like `clz`/`ctz`,
//! bitfield ops, ...) falls back to [`Instruction::Other`], which keeps the
//! original [`Operation`] so no information is ever lost — only unmodeled.
//!
//! Lowering never fails: if a known mnemonic's shape doesn't match what we
//! expect (missing operand, undecodable enum value, ...), that operation
//! degrades to `Other` rather than erroring, mirroring the two-phase
//! "structural-with-fallback" approach used for types/attributes.

use crate::ast::{Attribute, Block, Operation, Region, Type, ValueId};
use crate::model::enums::{AsmFlavor, CaseOpKind, CastKind, CmpOpKind, FpClassFlags, SideEffect};

/// A lowered region: almost always a single unlabeled block, but CIR can
/// still produce multiple blocks within a region (e.g. `goto` crossing
/// scopes lowers to plain block successors via `cir.br`/`cir.brcond`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Body {
    pub blocks: Vec<InstBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InstBlock {
    pub label: Option<String>,
    pub args: Vec<(ValueId, Type)>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    FAdd,
    FSub,
    FMul,
    FDiv,
}

impl std::str::FromStr for BinaryOp {
    type Err = crate::model::enums::ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(BinaryOp::Add),
            "sub" => Ok(BinaryOp::Sub),
            "mul" => Ok(BinaryOp::Mul),
            "div" => Ok(BinaryOp::Div),
            "rem" => Ok(BinaryOp::Rem),
            "and" => Ok(BinaryOp::And),
            "or" => Ok(BinaryOp::Or),
            "xor" => Ok(BinaryOp::Xor),
            "fadd" => Ok(BinaryOp::FAdd),
            "fsub" => Ok(BinaryOp::FSub),
            "fmul" => Ok(BinaryOp::FMul),
            "fdiv" => Ok(BinaryOp::FDiv),
            other => Err(crate::model::enums::ParseEnumError::new("BinaryOp", other)),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self {
            BinaryOp::Add => "add",
            BinaryOp::Sub => "sub",
            BinaryOp::Mul => "mul",
            BinaryOp::Div => "div",
            BinaryOp::Rem => "rem",
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
            BinaryOp::Xor => "xor",
            BinaryOp::FAdd => "fadd",
            BinaryOp::FSub => "fsub",
            BinaryOp::FMul => "fmul",
            BinaryOp::FDiv => "fdiv",
        };
        write!(f, "{kw}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOp {
    Inc,
    Dec,
    Minus,
    Not,
    FNeg,
}

impl std::str::FromStr for UnaryOp {
    type Err = crate::model::enums::ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(UnaryOp::Inc),
            "dec" => Ok(UnaryOp::Dec),
            "minus" => Ok(UnaryOp::Minus),
            "not" => Ok(UnaryOp::Not),
            "fneg" => Ok(UnaryOp::FNeg),
            other => Err(crate::model::enums::ParseEnumError::new("UnaryOp", other)),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self {
            UnaryOp::Inc => "inc",
            UnaryOp::Dec => "dec",
            UnaryOp::Minus => "minus",
            UnaryOp::Not => "not",
            UnaryOp::FNeg => "fneg",
        };
        write!(f, "{kw}")
    }
}

/// A single-operand math/bit-count builtin. `result`'s type can differ from
/// `operand`'s (e.g. `Signbit` takes a float and returns `bool`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MathUnaryKind {
    Fabs,
    Floor,
    Ffs,
    Clz,
    Ctz,
    Abs,
    Signbit,
}

impl std::str::FromStr for MathUnaryKind {
    type Err = crate::model::enums::ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fabs" => Ok(MathUnaryKind::Fabs),
            "floor" => Ok(MathUnaryKind::Floor),
            "ffs" => Ok(MathUnaryKind::Ffs),
            "clz" => Ok(MathUnaryKind::Clz),
            "ctz" => Ok(MathUnaryKind::Ctz),
            "abs" => Ok(MathUnaryKind::Abs),
            "signbit" => Ok(MathUnaryKind::Signbit),
            other => Err(crate::model::enums::ParseEnumError::new(
                "MathUnaryKind",
                other,
            )),
        }
    }
}

impl std::fmt::Display for MathUnaryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self {
            MathUnaryKind::Fabs => "fabs",
            MathUnaryKind::Floor => "floor",
            MathUnaryKind::Ffs => "ffs",
            MathUnaryKind::Clz => "clz",
            MathUnaryKind::Ctz => "ctz",
            MathUnaryKind::Abs => "abs",
            MathUnaryKind::Signbit => "signbit",
        };
        write!(f, "{kw}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BitfieldInfo {
    pub name: String,
    pub storage_type: Type,
    pub size: u32,
    pub offset: u32,
    pub is_signed: bool,
}

impl TryFrom<&Attribute> for BitfieldInfo {
    type Error = ();
    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        match attr {
            Attribute::BitfieldInfo {
                name,
                storage_type,
                size,
                offset,
                is_signed,
            } => Ok(BitfieldInfo {
                name: name.clone(),
                storage_type: storage_type.clone(),
                size: *size,
                offset: *offset,
                is_signed: *is_signed,
            }),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for BitfieldInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: {} bits @{} in {}{}",
            self.name,
            self.size,
            self.offset,
            if self.is_signed { "signed " } else { "" },
            self.storage_type
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwitchCase {
    pub kind: CaseOpKind,
    /// Case labels (empty for `default`, one value for `equal`, several for
    /// `anyof`, two for `range`).
    pub values: Vec<Attribute>,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Callee {
    Direct(String),
    Indirect(ValueId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Instruction {
    // -- memory --
    Alloca {
        result: ValueId,
        pointee_ty: Type,
        name: String,
        alignment: Option<u64>,
        dynamic_size: Option<ValueId>,
    },
    Load {
        result: ValueId,
        ty: Type,
        addr: ValueId,
        alignment: Option<u64>,
        is_volatile: bool,
    },
    Store {
        value: ValueId,
        addr: ValueId,
        alignment: Option<u64>,
        is_volatile: bool,
    },
    /// Aggregate (struct/array) copy: `cir.copy(dst, src)`.
    Copy {
        dst: ValueId,
        src: ValueId,
    },
    GetBitfield {
        result: ValueId,
        ty: Type,
        addr: ValueId,
        bitfield: BitfieldInfo,
        alignment: Option<u64>,
    },
    /// Result is the stored value viewed back through the field's logical
    /// type (sign-extended/truncated as CIR's semantics require), not the
    /// (possibly wider) storage type.
    SetBitfield {
        result: ValueId,
        ty: Type,
        addr: ValueId,
        value: ValueId,
        bitfield: BitfieldInfo,
        alignment: Option<u64>,
    },

    // -- constants / globals --
    Const {
        result: ValueId,
        ty: Type,
        value: Attribute,
    },
    GetGlobal {
        result: ValueId,
        ty: Type,
        name: String,
    },

    // -- casts / conversions --
    Cast {
        result: ValueId,
        ty: Type,
        kind: CastKind,
        operand: ValueId,
    },

    // -- arithmetic --
    Binary {
        result: ValueId,
        ty: Type,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
        no_signed_wrap: bool,
        no_unsigned_wrap: bool,
        saturated: bool,
    },
    Unary {
        result: ValueId,
        ty: Type,
        op: UnaryOp,
        operand: ValueId,
        no_signed_wrap: bool,
        no_unsigned_wrap: bool,
    },
    Shift {
        result: ValueId,
        ty: Type,
        value: ValueId,
        amount: ValueId,
        left: bool,
    },
    Rotate {
        result: ValueId,
        ty: Type,
        value: ValueId,
        amount: ValueId,
        left: bool,
    },
    Cmp {
        result: ValueId,
        ty: Type,
        kind: CmpOpKind,
        lhs: ValueId,
        rhs: ValueId,
    },
    Select {
        result: ValueId,
        ty: Type,
        condition: ValueId,
        true_value: ValueId,
        false_value: ValueId,
    },

    // -- calls --
    Call {
        result: Option<(ValueId, Type)>,
        callee: Callee,
        args: Vec<ValueId>,
        side_effect: Option<SideEffect>,
    },

    // -- aggregate access --
    GetMember {
        result: ValueId,
        ty: Type,
        addr: ValueId,
        index: u64,
        name: Option<String>,
    },
    ExtractMember {
        result: ValueId,
        ty: Type,
        value: ValueId,
        index: u64,
    },
    GetElement {
        result: ValueId,
        ty: Type,
        addr: ValueId,
        index: ValueId,
    },
    PtrStride {
        result: ValueId,
        ty: Type,
        addr: ValueId,
        stride: ValueId,
    },
    PtrDiff {
        result: ValueId,
        ty: Type,
        lhs: ValueId,
        rhs: ValueId,
    },

    // -- vectors --
    /// Broadcasts a scalar into every lane of a vector: `cir.vec.splat`.
    VecSplat {
        result: ValueId,
        ty: Type,
        value: ValueId,
    },
    VecExtract {
        result: ValueId,
        ty: Type,
        vec: ValueId,
        index: ValueId,
    },
    VecCreate {
        result: ValueId,
        ty: Type,
        elements: Vec<ValueId>,
    },

    // -- math / bit-count builtins --
    MathUnary {
        result: ValueId,
        ty: Type,
        kind: MathUnaryKind,
        operand: ValueId,
        /// `poison_zero` (clz/ctz) or `min_is_poison` (abs); always `false`
        /// for kinds that don't carry one.
        poison_flag: bool,
    },
    /// `__builtin_isfpclass`-style test: `cir.is_fp_class`.
    IsFpClass {
        result: ValueId,
        ty: Type,
        operand: ValueId,
        flags: FpClassFlags,
    },
    Copysign {
        result: ValueId,
        ty: Type,
        magnitude: ValueId,
        sign: ValueId,
    },
    Fmuladd {
        result: ValueId,
        ty: Type,
        a: ValueId,
        b: ValueId,
        c: ValueId,
    },
    MulOverflow {
        result: ValueId,
        overflow: ValueId,
        ty: Type,
        lhs: ValueId,
        rhs: ValueId,
    },
    AddOverflow {
        result: ValueId,
        overflow: ValueId,
        ty: Type,
        lhs: ValueId,
        rhs: ValueId,
    },

    // -- complex numbers --
    ComplexCreate {
        result: ValueId,
        ty: Type,
        real: ValueId,
        imag: ValueId,
    },
    ComplexReal {
        result: ValueId,
        ty: Type,
        operand: ValueId,
    },
    ComplexImag {
        result: ValueId,
        ty: Type,
        operand: ValueId,
    },
    /// Address of a complex value's real component: `cir.complex.real_ptr`.
    ComplexRealPtr {
        result: ValueId,
        ty: Type,
        operand: ValueId,
    },

    // -- varargs --
    VaStart {
        addr: ValueId,
    },
    VaEnd {
        addr: ValueId,
    },
    VaCopy {
        dst: ValueId,
        src: ValueId,
    },
    VaArg {
        result: ValueId,
        ty: Type,
        addr: ValueId,
    },

    // -- misc runtime builtins --
    FrameAddress {
        result: ValueId,
        ty: Type,
        level: ValueId,
    },
    ReturnAddress {
        result: ValueId,
        ty: Type,
        level: ValueId,
    },
    Prefetch {
        addr: ValueId,
        locality: u32,
        is_write: bool,
    },
    /// C/C++ inline asm: `cir.asm`. `result` is `None` when there's no
    /// (single-)output operand shape a caller reads back.
    InlineAsm {
        result: Option<(ValueId, Type)>,
        outputs: Vec<ValueId>,
        inputs: Vec<ValueId>,
        in_outs: Vec<ValueId>,
        asm_string: String,
        constraints: String,
        side_effects: bool,
        flavor: AsmFlavor,
    },

    // -- structured control flow --
    Scope {
        body: Body,
    },
    If {
        condition: ValueId,
        then_body: Body,
        else_body: Body,
    },
    While {
        cond_body: Body,
        body: Body,
    },
    DoWhile {
        body: Body,
        cond_body: Body,
    },
    For {
        cond_body: Body,
        body: Body,
        step_body: Body,
    },
    Switch {
        condition: ValueId,
        cases: Vec<SwitchCase>,
    },
    /// `result` is `None` for a statement-position ternary used only for its
    /// side effects (e.g. inside an `assert`-style macro) - `cir.ternary`
    /// carries no result value in that shape.
    Ternary {
        result: Option<(ValueId, Type)>,
        condition: ValueId,
        true_body: Body,
        false_body: Body,
    },

    // -- control transfer --
    Return {
        value: Option<ValueId>,
    },
    Yield {
        value: Option<ValueId>,
    },
    Condition {
        value: ValueId,
    },
    Break,
    Continue,
    /// Unconditional branch to a block in the same region (e.g. from `goto`
    /// crossing scopes): `cir.br[^label]`.
    Br {
        dest: String,
    },
    BrCond {
        condition: ValueId,
        true_dest: String,
        false_dest: String,
    },
    /// Named jump to a `cir.label` elsewhere (possibly in an enclosing
    /// region), resolved by a later CIR pass rather than by block successors.
    Goto {
        label: String,
    },
    Label {
        name: String,
    },
    IndirectGoto {
        addr: ValueId,
    },
    Unreachable,

    /// Anything not modeled above: the original operation is preserved
    /// verbatim (including any nested regions, which are *not* recursively
    /// lowered).
    Other(Operation),
}

pub fn lower_region(region: &Region) -> Body {
    Body {
        blocks: region.blocks.iter().map(lower_block).collect(),
    }
}

pub fn lower_block(block: &Block) -> InstBlock {
    InstBlock {
        label: block.label.clone(),
        args: block.args.clone(),
        body: block.ops.iter().map(lower_op).collect(),
    }
}

pub fn lower_op(op: &Operation) -> Instruction {
    try_lower(op).unwrap_or_else(|| Instruction::Other(op.clone()))
}

fn operand(op: &Operation, i: usize) -> Option<&ValueId> {
    op.operands.get(i)
}

fn single_result(op: &Operation) -> Option<&(ValueId, Type)> {
    op.results.first()
}

fn flag(op: &Operation, key: &str) -> bool {
    op.attr(key)
        .is_some_and(|a| !matches!(a, Attribute::Bool(false)))
}

fn alignment(op: &Operation) -> Option<u64> {
    op.attr("alignment")?.as_int().map(|v| v as u64)
}

fn region(op: &Operation, i: usize) -> Option<&Region> {
    op.regions.get(i)
}

/// Decodes a builtin `DenseI32ArrayAttr` (`array<i32: 2, 1, 0>`), which the
/// parser leaves as unstructured raw text since it's not CIR-specific.
fn dense_i32_array(attr: &Attribute) -> Option<Vec<i64>> {
    match attr {
        Attribute::Dialect {
            dialect,
            mnemonic,
            raw: Some(raw),
            ..
        } if dialect == "builtin" && mnemonic == "array" => {
            let (_elem_ty, list) = raw.split_once(':')?;
            list.split(',')
                .map(|v| v.trim().parse::<i64>().ok())
                .collect()
        }
        _ => None,
    }
}

/// Attempts a structural interpretation of `op`; `None` means "fall back to
/// `Other`" (missing operand/attr, or an enum value we don't recognize).
fn try_lower(op: &Operation) -> Option<Instruction> {
    match op.mnemonic() {
        "alloca" => {
            let (result, ty) = single_result(op)?;
            let pointee_ty = match ty {
                Type::Ptr(inner) => (**inner).clone(),
                other => other.clone(),
            };
            let name = op.attr("name").and_then(Attribute::as_str)?.to_string();
            Some(Instruction::Alloca {
                result: result.clone(),
                pointee_ty,
                name,
                alignment: alignment(op),
                dynamic_size: operand(op, 0).cloned(),
            })
        }
        "load" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Load {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
                alignment: alignment(op),
                is_volatile: flag(op, "is_volatile"),
            })
        }
        "store" => Some(Instruction::Store {
            value: operand(op, 0)?.clone(),
            addr: operand(op, 1)?.clone(),
            alignment: alignment(op),
            is_volatile: flag(op, "is_volatile"),
        }),
        "copy" => Some(Instruction::Copy {
            dst: operand(op, 0)?.clone(),
            src: operand(op, 1)?.clone(),
        }),
        "get_bitfield" => {
            let (result, ty) = single_result(op)?;
            let bitfield = BitfieldInfo::try_from(op.attr("bitfield_info")?).ok()?;
            Some(Instruction::GetBitfield {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
                bitfield,
                alignment: alignment(op),
            })
        }
        "set_bitfield" => {
            let (result, ty) = single_result(op)?;
            let bitfield = BitfieldInfo::try_from(op.attr("bitfield_info")?).ok()?;
            Some(Instruction::SetBitfield {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
                value: operand(op, 1)?.clone(),
                bitfield,
                alignment: alignment(op),
            })
        }
        "const" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Const {
                result: result.clone(),
                ty: ty.clone(),
                value: op.attr("value")?.clone(),
            })
        }
        "get_global" => {
            let (result, ty) = single_result(op)?;
            let name = op
                .attr("name")
                .and_then(Attribute::as_symbol_ref)?
                .to_string();
            Some(Instruction::GetGlobal {
                result: result.clone(),
                ty: ty.clone(),
                name,
            })
        }
        "cast" => {
            let (result, ty) = single_result(op)?;
            let kind = CastKind::try_from(op.attr("kind")?).ok()?;
            Some(Instruction::Cast {
                result: result.clone(),
                ty: ty.clone(),
                kind,
                operand: operand(op, 0)?.clone(),
            })
        }
        "add" | "sub" | "mul" | "div" | "rem" | "and" | "or" | "xor" | "fadd" | "fsub" | "fmul"
        | "fdiv" => {
            let (result, ty) = single_result(op)?;
            let bop = match op.mnemonic() {
                "add" => BinaryOp::Add,
                "sub" => BinaryOp::Sub,
                "mul" => BinaryOp::Mul,
                "div" => BinaryOp::Div,
                "rem" => BinaryOp::Rem,
                "and" => BinaryOp::And,
                "or" => BinaryOp::Or,
                "xor" => BinaryOp::Xor,
                "fadd" => BinaryOp::FAdd,
                "fsub" => BinaryOp::FSub,
                "fmul" => BinaryOp::FMul,
                "fdiv" => BinaryOp::FDiv,
                _ => unreachable!(),
            };
            Some(Instruction::Binary {
                result: result.clone(),
                ty: ty.clone(),
                op: bop,
                lhs: operand(op, 0)?.clone(),
                rhs: operand(op, 1)?.clone(),
                no_signed_wrap: flag(op, "no_signed_wrap"),
                no_unsigned_wrap: flag(op, "no_unsigned_wrap"),
                saturated: flag(op, "saturated"),
            })
        }
        "inc" | "dec" | "minus" | "not" | "fneg" => {
            let (result, ty) = single_result(op)?;
            let uop = match op.mnemonic() {
                "inc" => UnaryOp::Inc,
                "dec" => UnaryOp::Dec,
                "minus" => UnaryOp::Minus,
                "not" => UnaryOp::Not,
                "fneg" => UnaryOp::FNeg,
                _ => unreachable!(),
            };
            Some(Instruction::Unary {
                result: result.clone(),
                ty: ty.clone(),
                op: uop,
                operand: operand(op, 0)?.clone(),
                no_signed_wrap: flag(op, "no_signed_wrap"),
                no_unsigned_wrap: flag(op, "no_unsigned_wrap"),
            })
        }
        "shift" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Shift {
                result: result.clone(),
                ty: ty.clone(),
                value: operand(op, 0)?.clone(),
                amount: operand(op, 1)?.clone(),
                left: flag(op, "isShiftleft"),
            })
        }
        "rotate" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Rotate {
                result: result.clone(),
                ty: ty.clone(),
                value: operand(op, 0)?.clone(),
                amount: operand(op, 1)?.clone(),
                left: flag(op, "rotateLeft"),
            })
        }
        "cmp" => {
            let (result, ty) = single_result(op)?;
            let kind = CmpOpKind::try_from(op.attr("kind")?).ok()?;
            Some(Instruction::Cmp {
                result: result.clone(),
                ty: ty.clone(),
                kind,
                lhs: operand(op, 0)?.clone(),
                rhs: operand(op, 1)?.clone(),
            })
        }
        "select" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Select {
                result: result.clone(),
                ty: ty.clone(),
                condition: operand(op, 0)?.clone(),
                true_value: operand(op, 1)?.clone(),
                false_value: operand(op, 2)?.clone(),
            })
        }
        "call" => {
            let callee = match op.attr("callee").and_then(Attribute::as_symbol_ref) {
                Some(name) => Callee::Direct(name.to_string()),
                // Indirect calls take the callee as the first operand.
                None => Callee::Indirect(operand(op, 0)?.clone()),
            };
            let arg_start = if matches!(callee, Callee::Indirect(_)) {
                1
            } else {
                0
            };
            let side_effect = op
                .attr("side_effect")
                .and_then(|a| SideEffect::try_from(a).ok());
            Some(Instruction::Call {
                result: single_result(op).cloned(),
                callee,
                args: op.operands[arg_start.min(op.operands.len())..].to_vec(),
                side_effect,
            })
        }
        "get_member" => {
            let (result, ty) = single_result(op)?;
            let index = op.attr("index_attr")?.as_int()? as u64;
            let name = op
                .attr("name")
                .and_then(Attribute::as_str)
                .map(str::to_string);
            Some(Instruction::GetMember {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
                index,
                name,
            })
        }
        "extract_member" => {
            let (result, ty) = single_result(op)?;
            let index = op.attr("index_attr")?.as_int()? as u64;
            Some(Instruction::ExtractMember {
                result: result.clone(),
                ty: ty.clone(),
                value: operand(op, 0)?.clone(),
                index,
            })
        }
        "get_element" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::GetElement {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
                index: operand(op, 1)?.clone(),
            })
        }
        "ptr_stride" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::PtrStride {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
                stride: operand(op, 1)?.clone(),
            })
        }
        "ptr_diff" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::PtrDiff {
                result: result.clone(),
                ty: ty.clone(),
                lhs: operand(op, 0)?.clone(),
                rhs: operand(op, 1)?.clone(),
            })
        }
        "vec.splat" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::VecSplat {
                result: result.clone(),
                ty: ty.clone(),
                value: operand(op, 0)?.clone(),
            })
        }
        "vec.extract" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::VecExtract {
                result: result.clone(),
                ty: ty.clone(),
                vec: operand(op, 0)?.clone(),
                index: operand(op, 1)?.clone(),
            })
        }
        "vec.create" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::VecCreate {
                result: result.clone(),
                ty: ty.clone(),
                elements: op.operands.clone(),
            })
        }
        "is_fp_class" => {
            let (result, ty) = single_result(op)?;
            let flags = FpClassFlags::try_from(op.attr("flags")?).ok()?;
            Some(Instruction::IsFpClass {
                result: result.clone(),
                ty: ty.clone(),
                operand: operand(op, 0)?.clone(),
                flags,
            })
        }
        "fabs" | "floor" | "ffs" | "clz" | "ctz" | "abs" | "signbit" => {
            let (result, ty) = single_result(op)?;
            let kind: MathUnaryKind = op.mnemonic().parse().ok()?;
            let poison_flag = flag(op, "poison_zero") || flag(op, "min_is_poison");
            Some(Instruction::MathUnary {
                result: result.clone(),
                ty: ty.clone(),
                kind,
                operand: operand(op, 0)?.clone(),
                poison_flag,
            })
        }
        "copysign" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Copysign {
                result: result.clone(),
                ty: ty.clone(),
                magnitude: operand(op, 0)?.clone(),
                sign: operand(op, 1)?.clone(),
            })
        }
        "fmuladd" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::Fmuladd {
                result: result.clone(),
                ty: ty.clone(),
                a: operand(op, 0)?.clone(),
                b: operand(op, 1)?.clone(),
                c: operand(op, 2)?.clone(),
            })
        }
        "mul.overflow" | "add.overflow" => {
            let (result, ty) = op.results.first()?;
            let overflow = op.results.get(1)?.0.clone();
            let lhs = operand(op, 0)?.clone();
            let rhs = operand(op, 1)?.clone();
            Some(if op.mnemonic() == "mul.overflow" {
                Instruction::MulOverflow {
                    result: result.clone(),
                    overflow,
                    ty: ty.clone(),
                    lhs,
                    rhs,
                }
            } else {
                Instruction::AddOverflow {
                    result: result.clone(),
                    overflow,
                    ty: ty.clone(),
                    lhs,
                    rhs,
                }
            })
        }
        "complex.create" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::ComplexCreate {
                result: result.clone(),
                ty: ty.clone(),
                real: operand(op, 0)?.clone(),
                imag: operand(op, 1)?.clone(),
            })
        }
        "complex.real" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::ComplexReal {
                result: result.clone(),
                ty: ty.clone(),
                operand: operand(op, 0)?.clone(),
            })
        }
        "complex.imag" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::ComplexImag {
                result: result.clone(),
                ty: ty.clone(),
                operand: operand(op, 0)?.clone(),
            })
        }
        "complex.real_ptr" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::ComplexRealPtr {
                result: result.clone(),
                ty: ty.clone(),
                operand: operand(op, 0)?.clone(),
            })
        }
        "va_start" => Some(Instruction::VaStart {
            addr: operand(op, 0)?.clone(),
        }),
        "va_end" => Some(Instruction::VaEnd {
            addr: operand(op, 0)?.clone(),
        }),
        "va_copy" => Some(Instruction::VaCopy {
            dst: operand(op, 0)?.clone(),
            src: operand(op, 1)?.clone(),
        }),
        "va_arg" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::VaArg {
                result: result.clone(),
                ty: ty.clone(),
                addr: operand(op, 0)?.clone(),
            })
        }
        "frame_address" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::FrameAddress {
                result: result.clone(),
                ty: ty.clone(),
                level: operand(op, 0)?.clone(),
            })
        }
        "return_address" => {
            let (result, ty) = single_result(op)?;
            Some(Instruction::ReturnAddress {
                result: result.clone(),
                ty: ty.clone(),
                level: operand(op, 0)?.clone(),
            })
        }
        "prefetch" => Some(Instruction::Prefetch {
            addr: operand(op, 0)?.clone(),
            locality: op.attr("locality").and_then(Attribute::as_int).unwrap_or(0) as u32,
            is_write: flag(op, "isWrite"),
        }),
        "asm" => {
            // `asm_operands` is a VariadicOfVariadic (output, input, in_out
            // groups, in that order) flattened into `op.operands`, split back
            // up via the `operands_segments` dense-i32-array property.
            let segments = dense_i32_array(op.attr("operands_segments")?)?;
            let [out_n, in_n, inout_n] = <[i64; 3]>::try_from(segments).ok()?;
            let (out_n, in_n, inout_n) = (
                usize::try_from(out_n).ok()?,
                usize::try_from(in_n).ok()?,
                usize::try_from(inout_n).ok()?,
            );
            if out_n + in_n + inout_n != op.operands.len() {
                return None;
            }
            let (outputs, rest) = op.operands.split_at(out_n);
            let (inputs, in_outs) = rest.split_at(in_n);
            Some(Instruction::InlineAsm {
                result: single_result(op).cloned(),
                outputs: outputs.to_vec(),
                inputs: inputs.to_vec(),
                in_outs: in_outs.to_vec(),
                asm_string: op.attr("asm_string").and_then(Attribute::as_str)?.to_string(),
                constraints: op.attr("constraints").and_then(Attribute::as_str)?.to_string(),
                side_effects: op.attr("side_effects").is_some(),
                flavor: AsmFlavor::try_from(op.attr("asm_flavor")?).ok()?,
            })
        }
        "scope" => Some(Instruction::Scope {
            body: lower_region(region(op, 0)?),
        }),
        "if" => Some(Instruction::If {
            condition: operand(op, 0)?.clone(),
            then_body: lower_region(region(op, 0)?),
            else_body: lower_region(region(op, 1)?),
        }),
        "while" => Some(Instruction::While {
            cond_body: lower_region(region(op, 0)?),
            body: lower_region(region(op, 1)?),
        }),
        "do" => Some(Instruction::DoWhile {
            body: lower_region(region(op, 0)?),
            cond_body: lower_region(region(op, 1)?),
        }),
        "for" => Some(Instruction::For {
            cond_body: lower_region(region(op, 0)?),
            body: lower_region(region(op, 1)?),
            step_body: lower_region(region(op, 2)?),
        }),
        "switch" => {
            let cases = op
                .regions
                .first()?
                .blocks
                .first()?
                .ops
                .iter()
                .filter(|inner| inner.mnemonic() == "case")
                .map(|case_op| {
                    let kind = CaseOpKind::try_from(case_op.attr("kind")?).ok()?;
                    let values = case_op
                        .attr("value")
                        .and_then(Attribute::as_array)
                        .unwrap_or(&[])
                        .to_vec();
                    let body = lower_region(case_op.regions.first()?);
                    Some(SwitchCase { kind, values, body })
                })
                .collect::<Option<Vec<_>>>()?;
            Some(Instruction::Switch {
                condition: operand(op, 0)?.clone(),
                cases,
            })
        }
        "ternary" => Some(Instruction::Ternary {
            result: single_result(op).cloned(),
            condition: operand(op, 0)?.clone(),
            true_body: lower_region(region(op, 0)?),
            false_body: lower_region(region(op, 1)?),
        }),
        "return" => Some(Instruction::Return {
            value: operand(op, 0).cloned(),
        }),
        "yield" => Some(Instruction::Yield {
            value: operand(op, 0).cloned(),
        }),
        "condition" => Some(Instruction::Condition {
            value: operand(op, 0)?.clone(),
        }),
        "break" => Some(Instruction::Break),
        "continue" => Some(Instruction::Continue),
        "br" => Some(Instruction::Br {
            dest: op.successors.first()?.clone(),
        }),
        "brcond" => Some(Instruction::BrCond {
            condition: operand(op, 0)?.clone(),
            true_dest: op.successors.first()?.clone(),
            false_dest: op.successors.get(1)?.clone(),
        }),
        "goto" => Some(Instruction::Goto {
            label: op.attr("label").and_then(Attribute::as_str)?.to_string(),
        }),
        "label" => Some(Instruction::Label {
            name: op.attr("label").and_then(Attribute::as_str)?.to_string(),
        }),
        "indirect_goto" => Some(Instruction::IndirectGoto {
            addr: operand(op, 0)?.clone(),
        }),
        "unreachable" => Some(Instruction::Unreachable),
        _ => None,
    }
}

impl std::fmt::Display for Callee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callee::Direct(name) => write!(f, "@{name}"),
            Callee::Indirect(v) => write!(f, "%{v}"),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_instruction(self, f, 0)
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_body(self, f, 0)
    }
}

fn write_indent(f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    for _ in 0..level {
        write!(f, "    ")?;
    }
    Ok(())
}

pub(crate) fn write_body(
    body: &Body,
    f: &mut std::fmt::Formatter<'_>,
    level: usize,
) -> std::fmt::Result {
    for block in &body.blocks {
        match &block.label {
            Some(label) => {
                write_indent(f, level)?;
                write!(f, "^{label}(")?;
                for (i, (id, ty)) in block.args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "%{id}: {ty}")?;
                }
                writeln!(f, "):")?;
                for instr in &block.body {
                    write_instruction(instr, f, level + 1)?;
                }
            }
            None => {
                for instr in &block.body {
                    write_instruction(instr, f, level)?;
                }
            }
        }
    }
    Ok(())
}

/// Writes an indented `keyword { <nested body> }` block, e.g. for `scope`/`if`/`while`.
fn write_nested(
    f: &mut std::fmt::Formatter<'_>,
    level: usize,
    header: &str,
    body: &Body,
) -> std::fmt::Result {
    write_indent(f, level)?;
    writeln!(f, "{header} {{")?;
    write_body(body, f, level + 1)?;
    write_indent(f, level)?;
    writeln!(f, "}}")
}

fn write_instruction(
    instr: &Instruction,
    f: &mut std::fmt::Formatter<'_>,
    level: usize,
) -> std::fmt::Result {
    use Instruction::*;
    match instr {
        Alloca {
            result,
            pointee_ty,
            name,
            alignment,
            dynamic_size,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = alloca {pointee_ty}, {name:?}")?;
            if let Some(a) = alignment {
                write!(f, ", align {a}")?;
            }
            if let Some(d) = dynamic_size {
                write!(f, ", dynamic %{d}")?;
            }
            writeln!(f)
        }
        Load {
            result,
            ty,
            addr,
            alignment,
            is_volatile,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = load %{addr} : {ty}")?;
            if let Some(a) = alignment {
                write!(f, ", align {a}")?;
            }
            if *is_volatile {
                write!(f, ", volatile")?;
            }
            writeln!(f)
        }
        Store {
            value,
            addr,
            alignment,
            is_volatile,
        } => {
            write_indent(f, level)?;
            write!(f, "store %{value}, %{addr}")?;
            if let Some(a) = alignment {
                write!(f, ", align {a}")?;
            }
            if *is_volatile {
                write!(f, ", volatile")?;
            }
            writeln!(f)
        }
        Copy { dst, src } => {
            write_indent(f, level)?;
            writeln!(f, "copy %{dst}, %{src}")
        }
        GetBitfield {
            result,
            ty,
            addr,
            bitfield,
            alignment,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = get_bitfield %{addr}, {bitfield} : {ty}")?;
            if let Some(a) = alignment {
                write!(f, ", align {a}")?;
            }
            writeln!(f)
        }
        SetBitfield {
            result,
            ty,
            addr,
            value,
            bitfield,
            alignment,
        } => {
            write_indent(f, level)?;
            write!(
                f,
                "%{result} = set_bitfield %{addr}, %{value}, {bitfield} : {ty}"
            )?;
            if let Some(a) = alignment {
                write!(f, ", align {a}")?;
            }
            writeln!(f)
        }
        Const { result, ty, value } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = const {value} : {ty}")
        }
        GetGlobal { result, ty, name } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = get_global @{name} : {ty}")
        }
        Cast {
            result,
            ty,
            kind,
            operand,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = cast({kind}) %{operand} : {ty}")
        }
        Binary {
            result,
            ty,
            op,
            lhs,
            rhs,
            no_signed_wrap,
            no_unsigned_wrap,
            saturated,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = {op} %{lhs}, %{rhs} : {ty}")?;
            write_flags(
                f,
                &[
                    ("nsw", *no_signed_wrap),
                    ("nuw", *no_unsigned_wrap),
                    ("sat", *saturated),
                ],
            )?;
            writeln!(f)
        }
        Unary {
            result,
            ty,
            op,
            operand,
            no_signed_wrap,
            no_unsigned_wrap,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = {op} %{operand} : {ty}")?;
            write_flags(f, &[("nsw", *no_signed_wrap), ("nuw", *no_unsigned_wrap)])?;
            writeln!(f)
        }
        Shift {
            result,
            ty,
            value,
            amount,
            left,
        } => {
            write_indent(f, level)?;
            writeln!(
                f,
                "%{result} = shift {} %{value}, %{amount} : {ty}",
                if *left { "left" } else { "right" }
            )
        }
        Rotate {
            result,
            ty,
            value,
            amount,
            left,
        } => {
            write_indent(f, level)?;
            writeln!(
                f,
                "%{result} = rotate {} %{value}, %{amount} : {ty}",
                if *left { "left" } else { "right" }
            )
        }
        Cmp {
            result,
            ty,
            kind,
            lhs,
            rhs,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = cmp {kind} %{lhs}, %{rhs} : {ty}")
        }
        Select {
            result,
            ty,
            condition,
            true_value,
            false_value,
        } => {
            write_indent(f, level)?;
            writeln!(
                f,
                "%{result} = select %{condition}, %{true_value}, %{false_value} : {ty}"
            )
        }
        Call {
            result,
            callee,
            args,
            side_effect,
        } => {
            write_indent(f, level)?;
            if let Some((r, _)) = result {
                write!(f, "%{r} = ")?;
            }
            write!(f, "call {callee}(")?;
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "%{a}")?;
            }
            write!(f, ")")?;
            if let Some((_, ty)) = result {
                write!(f, " : {ty}")?;
            }
            if let Some(se) = side_effect {
                write!(f, " [{se}]")?;
            }
            writeln!(f)
        }
        GetMember {
            result,
            ty,
            addr,
            index,
            name,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = get_member %{addr}[{index}]")?;
            if let Some(name) = name {
                write!(f, " {name:?}")?;
            }
            writeln!(f, " : {ty}")
        }
        ExtractMember {
            result,
            ty,
            value,
            index,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = extract_member %{value}[{index}] : {ty}")
        }
        GetElement {
            result,
            ty,
            addr,
            index,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = get_element %{addr}[%{index}] : {ty}")
        }
        PtrStride {
            result,
            ty,
            addr,
            stride,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = ptr_stride %{addr}, %{stride} : {ty}")
        }
        PtrDiff {
            result,
            ty,
            lhs,
            rhs,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = ptr_diff %{lhs}, %{rhs} : {ty}")
        }
        VecSplat { result, ty, value } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = vec.splat %{value} : {ty}")
        }
        VecExtract {
            result,
            ty,
            vec,
            index,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = vec.extract %{vec}[%{index}] : {ty}")
        }
        VecCreate {
            result,
            ty,
            elements,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = vec.create(")?;
            write_value_list(f, elements)?;
            writeln!(f, ") : {ty}")
        }
        MathUnary {
            result,
            ty,
            kind,
            operand,
            poison_flag,
        } => {
            write_indent(f, level)?;
            write!(f, "%{result} = {kind} %{operand} : {ty}")?;
            write_flags(f, &[("poison", *poison_flag)])?;
            writeln!(f)
        }
        IsFpClass {
            result,
            ty,
            operand,
            flags,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = is_fp_class %{operand}, {flags} : {ty}")
        }
        Copysign {
            result,
            ty,
            magnitude,
            sign,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = copysign %{magnitude}, %{sign} : {ty}")
        }
        Fmuladd {
            result,
            ty,
            a,
            b,
            c,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = fmuladd %{a}, %{b}, %{c} : {ty}")
        }
        MulOverflow {
            result,
            overflow,
            ty,
            lhs,
            rhs,
        } => {
            write_indent(f, level)?;
            writeln!(
                f,
                "%{result}, %{overflow} = mul.overflow %{lhs}, %{rhs} : {ty}"
            )
        }
        AddOverflow {
            result,
            overflow,
            ty,
            lhs,
            rhs,
        } => {
            write_indent(f, level)?;
            writeln!(
                f,
                "%{result}, %{overflow} = add.overflow %{lhs}, %{rhs} : {ty}"
            )
        }
        ComplexCreate {
            result,
            ty,
            real,
            imag,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = complex.create %{real}, %{imag} : {ty}")
        }
        ComplexReal {
            result,
            ty,
            operand,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = complex.real %{operand} : {ty}")
        }
        ComplexImag {
            result,
            ty,
            operand,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = complex.imag %{operand} : {ty}")
        }
        ComplexRealPtr {
            result,
            ty,
            operand,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = complex.real_ptr %{operand} : {ty}")
        }
        VaStart { addr } => {
            write_indent(f, level)?;
            writeln!(f, "va_start %{addr}")
        }
        VaEnd { addr } => {
            write_indent(f, level)?;
            writeln!(f, "va_end %{addr}")
        }
        VaCopy { dst, src } => {
            write_indent(f, level)?;
            writeln!(f, "va_copy %{dst}, %{src}")
        }
        VaArg { result, ty, addr } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = va_arg %{addr} : {ty}")
        }
        FrameAddress {
            result,
            ty,
            level: addr_level,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = frame_address(%{addr_level}) : {ty}")
        }
        ReturnAddress {
            result,
            ty,
            level: addr_level,
        } => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = return_address(%{addr_level}) : {ty}")
        }
        Prefetch {
            addr,
            locality,
            is_write,
        } => {
            write_indent(f, level)?;
            write!(f, "prefetch %{addr}, locality({locality})")?;
            write_flags(f, &[("write", *is_write)])?;
            writeln!(f)
        }
        InlineAsm {
            result,
            outputs,
            inputs,
            in_outs,
            asm_string,
            constraints,
            side_effects,
            flavor,
        } => {
            write_indent(f, level)?;
            if let Some((r, _)) = result {
                write!(f, "%{r} = ")?;
            }
            write!(f, "asm({flavor}, out = [")?;
            write_value_list(f, outputs)?;
            write!(f, "], in = [")?;
            write_value_list(f, inputs)?;
            write!(f, "], in_out = [")?;
            write_value_list(f, in_outs)?;
            write!(f, "], {asm_string:?}, {constraints:?}")?;
            write_flags(f, &[("side_effects", *side_effects)])?;
            if let Some((_, ty)) = result {
                write!(f, " : {ty}")?;
            }
            writeln!(f)
        }
        Scope { body } => write_nested(f, level, "scope", body),
        If {
            condition,
            then_body,
            else_body,
        } => {
            write_indent(f, level)?;
            writeln!(f, "if %{condition} {{")?;
            write_body(then_body, f, level + 1)?;
            if else_body.blocks.iter().any(|b| !b.body.is_empty()) {
                write_indent(f, level)?;
                writeln!(f, "}} else {{")?;
                write_body(else_body, f, level + 1)?;
            }
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        While { cond_body, body } => {
            write_indent(f, level)?;
            writeln!(f, "while {{")?;
            write_body(cond_body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} do {{")?;
            write_body(body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        DoWhile { body, cond_body } => {
            write_indent(f, level)?;
            writeln!(f, "do {{")?;
            write_body(body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} while {{")?;
            write_body(cond_body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        For {
            cond_body,
            body,
            step_body,
        } => {
            write_indent(f, level)?;
            writeln!(f, "for cond: {{")?;
            write_body(cond_body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} step: {{")?;
            write_body(step_body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} do {{")?;
            write_body(body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Switch { condition, cases } => {
            write_indent(f, level)?;
            writeln!(f, "switch %{condition} {{")?;
            for case in cases {
                write_indent(f, level + 1)?;
                write!(f, "case {}", case.kind)?;
                if !case.values.is_empty() {
                    write!(f, " ")?;
                    for (i, v) in case.values.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{v}")?;
                    }
                }
                writeln!(f, " {{")?;
                write_body(&case.body, f, level + 2)?;
                write_indent(f, level + 1)?;
                writeln!(f, "}}")?;
            }
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Ternary {
            result,
            condition,
            true_body,
            false_body,
        } => {
            write_indent(f, level)?;
            match result {
                Some((r, ty)) => writeln!(f, "%{r} = ternary %{condition} ? {{ : {ty}")?,
                None => writeln!(f, "ternary %{condition} ? {{")?,
            }
            write_body(true_body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} : {{")?;
            write_body(false_body, f, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Return { value } => {
            write_indent(f, level)?;
            write!(f, "return")?;
            if let Some(v) = value {
                write!(f, " %{v}")?;
            }
            writeln!(f)
        }
        Yield { value } => {
            write_indent(f, level)?;
            write!(f, "yield")?;
            if let Some(v) = value {
                write!(f, " %{v}")?;
            }
            writeln!(f)
        }
        Condition { value } => {
            write_indent(f, level)?;
            writeln!(f, "condition %{value}")
        }
        Break => {
            write_indent(f, level)?;
            writeln!(f, "break")
        }
        Continue => {
            write_indent(f, level)?;
            writeln!(f, "continue")
        }
        Br { dest } => {
            write_indent(f, level)?;
            writeln!(f, "br ^{dest}")
        }
        BrCond {
            condition,
            true_dest,
            false_dest,
        } => {
            write_indent(f, level)?;
            writeln!(f, "brcond %{condition}, ^{true_dest}, ^{false_dest}")
        }
        Goto { label } => {
            write_indent(f, level)?;
            writeln!(f, "goto {label}")
        }
        Label { name } => {
            write_indent(f, level)?;
            writeln!(f, "label {name}:")
        }
        IndirectGoto { addr } => {
            write_indent(f, level)?;
            writeln!(f, "indirect_goto %{addr}")
        }
        Unreachable => {
            write_indent(f, level)?;
            writeln!(f, "unreachable")
        }
        // The fallback case: only the mnemonic is shown (not the full raw
        // operation tree) to keep this readable; use `Instruction::Other`'s
        // inner `Operation` directly for full detail.
        Other(op) => {
            write_indent(f, level)?;
            writeln!(f, "<unmodeled: {}>", op.name)
        }
    }
}

fn write_value_list(f: &mut std::fmt::Formatter<'_>, ids: &[ValueId]) -> std::fmt::Result {
    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "%{id}")?;
    }
    Ok(())
}

fn write_flags(f: &mut std::fmt::Formatter<'_>, flags: &[(&str, bool)]) -> std::fmt::Result {
    let set: Vec<&str> = flags
        .iter()
        .filter(|(_, v)| *v)
        .map(|(name, _)| *name)
        .collect();
    if !set.is_empty() {
        write!(f, " [{}]", set.join(", "))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lower_single_op(src: &str) -> Instruction {
        let module = crate::parser::parse_generic_module(src).unwrap();
        lower_op(&module.ops[0])
    }

    #[test]
    fn vec_splat_lowers() {
        let instr = lower_single_op(
            r#"%1 = "cir.vec.splat"(%0) : (!cir.int<s, 32>) -> !cir.vector<4 x !cir.int<s, 32>>"#,
        );
        assert!(matches!(
            instr,
            Instruction::VecSplat { ref result, ref value, .. }
                if result == "1" && value == "0"
        ));
        assert_eq!(instr.to_string(), "%1 = vec.splat %0 : vector<4 x s32>\n");
    }

    #[test]
    fn vec_extract_lowers() {
        let instr = lower_single_op(
            r#"%2 = "cir.vec.extract"(%0, %1) : (!cir.vector<4 x !cir.int<s, 32>>, !cir.int<s, 32>) -> !cir.int<s, 32>"#,
        );
        assert!(matches!(
            instr,
            Instruction::VecExtract { ref result, ref vec, ref index, .. }
                if result == "2" && vec == "0" && index == "1"
        ));
        assert_eq!(instr.to_string(), "%2 = vec.extract %0[%1] : s32\n");
    }

    #[test]
    fn is_fp_class_lowers() {
        let instr = lower_single_op(
            r#"%1 = "cir.is_fp_class"(%0) <{flags = 3 : i32}> : (!cir.double) -> !cir.bool"#,
        );
        assert!(matches!(
            instr,
            Instruction::IsFpClass { ref result, ref operand, flags, .. }
                if result == "1" && operand == "0" && flags.0 == 3
        ));
        assert_eq!(
            instr.to_string(),
            "%1 = is_fp_class %0, fcSNan|fcQNan : bool\n"
        );
    }

    #[test]
    fn unrecognized_flags_still_lower_via_raw_bitmask() {
        // Not a documented composite group, just an arbitrary bit combination
        // (positive infinity | negative zero) - still a valid `flags` value.
        let instr = lower_single_op(
            r#"%1 = "cir.is_fp_class"(%0) <{flags = 544 : i32}> : (!cir.float) -> !cir.bool"#,
        );
        assert!(matches!(instr, Instruction::IsFpClass { flags, .. } if flags.0 == 544));
    }

    #[test]
    fn vec_create_lowers() {
        let instr = lower_single_op(
            r#"%2 = "cir.vec.create"(%0, %1) : (!cir.int<s, 32>, !cir.int<s, 32>) -> !cir.vector<2 x !cir.int<s, 32>>"#,
        );
        assert!(matches!(
            instr,
            Instruction::VecCreate { ref result, ref elements, .. }
                if result == "2" && elements == &["0".to_string(), "1".to_string()]
        ));
        assert_eq!(
            instr.to_string(),
            "%2 = vec.create(%0, %1) : vector<2 x s32>\n"
        );
    }

    #[test]
    fn complex_real_ptr_lowers() {
        let instr = lower_single_op(
            r#"%1 = "cir.complex.real_ptr"(%0) : (!cir.ptr<!cir.complex<!cir.double>>) -> !cir.ptr<!cir.double>"#,
        );
        assert!(matches!(
            instr,
            Instruction::ComplexRealPtr { ref result, ref operand, .. }
                if result == "1" && operand == "0"
        ));
        assert_eq!(
            instr.to_string(),
            "%1 = complex.real_ptr %0 : double*\n"
        );
    }

    #[test]
    fn asm_lowers_with_no_operands() {
        let instr = lower_single_op(
            r#""cir.asm"() <{asm_flavor = 0 : i32, asm_string = "foo", constraints = "~{dirflag},~{fpsr},~{flags}", operand_attrs = [], operands_segments = array<i32: 0, 0, 0>, side_effects}> : () -> ()"#,
        );
        assert!(matches!(
            instr,
            Instruction::InlineAsm {
                ref result,
                ref outputs,
                ref inputs,
                ref in_outs,
                ref asm_string,
                ref constraints,
                side_effects: true,
                flavor: AsmFlavor::AttSyntax,
            } if result.is_none()
                && outputs.is_empty()
                && inputs.is_empty()
                && in_outs.is_empty()
                && asm_string == "foo"
                && constraints == "~{dirflag},~{fpsr},~{flags}"
        ));
        assert_eq!(
            instr.to_string(),
            "asm(x86_att, out = [], in = [], in_out = [], \"foo\", \"~{dirflag},~{fpsr},~{flags}\" [side_effects]\n"
        );
    }

    #[test]
    fn asm_lowers_with_in_out_operand_and_result() {
        let instr = lower_single_op(
            r#"%1 = "cir.asm"(%0) <{asm_flavor = 0 : i32, asm_string = "bar $$42 $0", constraints = "=r,=&r,1,~{dirflag},~{fpsr},~{flags}", operand_attrs = [], operands_segments = array<i32: 0, 0, 1>}> : (!cir.int<s, 32>) -> !cir.int<s, 32>"#,
        );
        assert!(matches!(
            instr,
            Instruction::InlineAsm {
                ref result,
                ref outputs,
                ref inputs,
                ref in_outs,
                side_effects: false,
                flavor: AsmFlavor::AttSyntax,
                ..
            } if result.as_ref().is_some_and(|(r, _)| r == "1")
                && outputs.is_empty()
                && inputs.is_empty()
                && in_outs == &["0".to_string()]
        ));
    }
}
