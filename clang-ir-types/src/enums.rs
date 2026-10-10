//! CIR enum attributes generated from TableGen enum definitions.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnumError {
    type_name: &'static str,
    input: String,
}
impl ParseEnumError {
    pub fn new(type_name: &'static str, input: impl Into<String>) -> Self {
        Self {
            type_name,
            input: input.into(),
        }
    }
}
impl std::fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` is not a valid {}", self.input, self.type_name)
    }
}
impl std::error::Error for ParseEnumError {}
/// record argument passing eligibility
/// Enum cases:
/// * can_pass_in_regs (`CanPassInRegs`)
/// * cannot_pass_in_regs (`CannotPassInRegs`)
/// * can_never_pass_in_regs (`CanNeverPassInRegs`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum ArgPassingKind {
    CanPassInRegs = 0i64,
    CannotPassInRegs = 1i64,
    CanNeverPassInRegs = 2i64,
}
impl TryFrom<i128> for ArgPassingKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::CanPassInRegs),
            1i64 => Ok(Self::CannotPassInRegs),
            2i64 => Ok(Self::CanNeverPassInRegs),
            _ => Err(ParseEnumError::new("ArgPassingKind", value.to_string())),
        }
    }
}
impl From<ArgPassingKind> for i128 {
    fn from(value: ArgPassingKind) -> i128 {
        match value {
            ArgPassingKind::CanPassInRegs => i128::from(0i64),
            ArgPassingKind::CannotPassInRegs => i128::from(1i64),
            ArgPassingKind::CanNeverPassInRegs => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for ArgPassingKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "can_pass_in_regs" => Ok(Self::CanPassInRegs),
            "cannot_pass_in_regs" => Ok(Self::CannotPassInRegs),
            "can_never_pass_in_regs" => Ok(Self::CanNeverPassInRegs),
            _ => Err(ParseEnumError::new("ArgPassingKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for ArgPassingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::CanPassInRegs => "can_pass_in_regs",
            Self::CannotPassInRegs => "cannot_pass_in_regs",
            Self::CanNeverPassInRegs => "can_never_pass_in_regs",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for ArgPassingKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("ArgPassingKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("ArgPassingKind", format!("{other:?}"))),
        }
    }
}
/// ATT or Intel
/// Enum cases:
/// * x86_att (`x86_att`)
/// * x86_intel (`x86_intel`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum AsmFlavor {
    X86Att = 0i64,
    X86Intel = 1i64,
}
impl TryFrom<i128> for AsmFlavor {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::X86Att),
            1i64 => Ok(Self::X86Intel),
            _ => Err(ParseEnumError::new("AsmFlavor", value.to_string())),
        }
    }
}
impl From<AsmFlavor> for i128 {
    fn from(value: AsmFlavor) -> i128 {
        match value {
            AsmFlavor::X86Att => i128::from(0i64),
            AsmFlavor::X86Intel => i128::from(1i64),
        }
    }
}
impl std::str::FromStr for AsmFlavor {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86_att" => Ok(Self::X86Att),
            "x86_intel" => Ok(Self::X86Intel),
            _ => Err(ParseEnumError::new("AsmFlavor", s.to_string())),
        }
    }
}
impl std::fmt::Display for AsmFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::X86Att => "x86_att",
            Self::X86Intel => "x86_intel",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for AsmFlavor {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("AsmFlavor", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("AsmFlavor", format!("{other:?}"))),
        }
    }
}
/// CXX Assignment Operator Kind
/// Enum cases:
/// * copy (`Copy`)
/// * move (`Move`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum AssignKind {
    Copy = 0i64,
    Move = 1i64,
}
impl TryFrom<i128> for AssignKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Copy),
            1i64 => Ok(Self::Move),
            _ => Err(ParseEnumError::new("AssignKind", value.to_string())),
        }
    }
}
impl From<AssignKind> for i128 {
    fn from(value: AssignKind) -> i128 {
        match value {
            AssignKind::Copy => i128::from(0i64),
            AssignKind::Move => i128::from(1i64),
        }
    }
}
impl std::str::FromStr for AssignKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "copy" => Ok(Self::Copy),
            "move" => Ok(Self::Move),
            _ => Err(ParseEnumError::new("AssignKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for AssignKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Copy => "copy",
            Self::Move => "move",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for AssignKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("AssignKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("AssignKind", format!("{other:?}"))),
        }
    }
}
/// kind of cir.assume operand bundle
/// Enum cases:
/// * None (`None`)
/// * align (`Align`)
/// * separate_storage (`SeparateStorage`)
/// * dereferenceable (`Dereferenceable`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum AssumeBundleKind {
    None = 0i64,
    Align = 1i64,
    SeparateStorage = 2i64,
    Dereferenceable = 3i64,
}
impl TryFrom<i128> for AssumeBundleKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::None),
            1i64 => Ok(Self::Align),
            2i64 => Ok(Self::SeparateStorage),
            3i64 => Ok(Self::Dereferenceable),
            _ => Err(ParseEnumError::new("AssumeBundleKind", value.to_string())),
        }
    }
}
impl From<AssumeBundleKind> for i128 {
    fn from(value: AssumeBundleKind) -> i128 {
        match value {
            AssumeBundleKind::None => i128::from(0i64),
            AssumeBundleKind::Align => i128::from(1i64),
            AssumeBundleKind::SeparateStorage => i128::from(2i64),
            AssumeBundleKind::Dereferenceable => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for AssumeBundleKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "align" => Ok(Self::Align),
            "separate_storage" => Ok(Self::SeparateStorage),
            "dereferenceable" => Ok(Self::Dereferenceable),
            _ => Err(ParseEnumError::new("AssumeBundleKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for AssumeBundleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::None => "None",
            Self::Align => "align",
            Self::SeparateStorage => "separate_storage",
            Self::Dereferenceable => "dereferenceable",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for AssumeBundleKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("AssumeBundleKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("AssumeBundleKind", format!("{other:?}"))),
        }
    }
}
/// Binary opcode for atomic fetch-and-update operations
/// Enum cases:
/// * add (`Add`)
/// * sub (`Sub`)
/// * and (`And`)
/// * xor (`Xor`)
/// * or (`Or`)
/// * nand (`Nand`)
/// * max (`Max`)
/// * min (`Min`)
/// * uinc_wrap (`UIncWrap`)
/// * udec_wrap (`UDecWrap`)
/// * maximum (`Maximum`)
/// * minimum (`Minimum`)
/// * maximum_num (`MaximumNum`)
/// * minimum_num (`MinimumNum`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum AtomicFetchKind {
    Add = 0i64,
    Sub = 1i64,
    And = 2i64,
    Xor = 3i64,
    Or = 4i64,
    Nand = 5i64,
    Max = 6i64,
    Min = 7i64,
    UIncWrap = 8i64,
    UDecWrap = 9i64,
    Maximum = 10i64,
    Minimum = 11i64,
    MaximumNum = 12i64,
    MinimumNum = 13i64,
}
impl TryFrom<i128> for AtomicFetchKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Add),
            1i64 => Ok(Self::Sub),
            2i64 => Ok(Self::And),
            3i64 => Ok(Self::Xor),
            4i64 => Ok(Self::Or),
            5i64 => Ok(Self::Nand),
            6i64 => Ok(Self::Max),
            7i64 => Ok(Self::Min),
            8i64 => Ok(Self::UIncWrap),
            9i64 => Ok(Self::UDecWrap),
            10i64 => Ok(Self::Maximum),
            11i64 => Ok(Self::Minimum),
            12i64 => Ok(Self::MaximumNum),
            13i64 => Ok(Self::MinimumNum),
            _ => Err(ParseEnumError::new("AtomicFetchKind", value.to_string())),
        }
    }
}
impl From<AtomicFetchKind> for i128 {
    fn from(value: AtomicFetchKind) -> i128 {
        match value {
            AtomicFetchKind::Add => i128::from(0i64),
            AtomicFetchKind::Sub => i128::from(1i64),
            AtomicFetchKind::And => i128::from(2i64),
            AtomicFetchKind::Xor => i128::from(3i64),
            AtomicFetchKind::Or => i128::from(4i64),
            AtomicFetchKind::Nand => i128::from(5i64),
            AtomicFetchKind::Max => i128::from(6i64),
            AtomicFetchKind::Min => i128::from(7i64),
            AtomicFetchKind::UIncWrap => i128::from(8i64),
            AtomicFetchKind::UDecWrap => i128::from(9i64),
            AtomicFetchKind::Maximum => i128::from(10i64),
            AtomicFetchKind::Minimum => i128::from(11i64),
            AtomicFetchKind::MaximumNum => i128::from(12i64),
            AtomicFetchKind::MinimumNum => i128::from(13i64),
        }
    }
}
impl std::str::FromStr for AtomicFetchKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "and" => Ok(Self::And),
            "xor" => Ok(Self::Xor),
            "or" => Ok(Self::Or),
            "nand" => Ok(Self::Nand),
            "max" => Ok(Self::Max),
            "min" => Ok(Self::Min),
            "uinc_wrap" => Ok(Self::UIncWrap),
            "udec_wrap" => Ok(Self::UDecWrap),
            "maximum" => Ok(Self::Maximum),
            "minimum" => Ok(Self::Minimum),
            "maximum_num" => Ok(Self::MaximumNum),
            "minimum_num" => Ok(Self::MinimumNum),
            _ => Err(ParseEnumError::new("AtomicFetchKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for AtomicFetchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::And => "and",
            Self::Xor => "xor",
            Self::Or => "or",
            Self::Nand => "nand",
            Self::Max => "max",
            Self::Min => "min",
            Self::UIncWrap => "uinc_wrap",
            Self::UDecWrap => "udec_wrap",
            Self::Maximum => "maximum",
            Self::Minimum => "minimum",
            Self::MaximumNum => "maximum_num",
            Self::MinimumNum => "minimum_num",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for AtomicFetchKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("AtomicFetchKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("AtomicFetchKind", format!("{other:?}"))),
        }
    }
}
/// await kind
/// Enum cases:
/// * init (`Init`)
/// * user (`User`)
/// * yield (`Yield`)
/// * final (`Final`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum AwaitKind {
    Init = 0i64,
    User = 1i64,
    Yield = 2i64,
    Final = 3i64,
}
impl TryFrom<i128> for AwaitKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Init),
            1i64 => Ok(Self::User),
            2i64 => Ok(Self::Yield),
            3i64 => Ok(Self::Final),
            _ => Err(ParseEnumError::new("AwaitKind", value.to_string())),
        }
    }
}
impl From<AwaitKind> for i128 {
    fn from(value: AwaitKind) -> i128 {
        match value {
            AwaitKind::Init => i128::from(0i64),
            AwaitKind::User => i128::from(1i64),
            AwaitKind::Yield => i128::from(2i64),
            AwaitKind::Final => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for AwaitKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "init" => Ok(Self::Init),
            "user" => Ok(Self::User),
            "yield" => Ok(Self::Yield),
            "final" => Ok(Self::Final),
            _ => Err(ParseEnumError::new("AwaitKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for AwaitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Init => "init",
            Self::User => "user",
            Self::Yield => "yield",
            Self::Final => "final",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for AwaitKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("AwaitKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("AwaitKind", format!("{other:?}"))),
        }
    }
}
/// CUDA device variable kind
/// Enum cases:
/// * Variable (`Variable`)
/// * Surface (`Surface`)
/// * Texture (`Texture`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CudaDeviceVarKind {
    Variable = 0i64,
    Surface = 1i64,
    Texture = 2i64,
}
impl TryFrom<i128> for CudaDeviceVarKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Variable),
            1i64 => Ok(Self::Surface),
            2i64 => Ok(Self::Texture),
            _ => Err(ParseEnumError::new("CudaDeviceVarKind", value.to_string())),
        }
    }
}
impl From<CudaDeviceVarKind> for i128 {
    fn from(value: CudaDeviceVarKind) -> i128 {
        match value {
            CudaDeviceVarKind::Variable => i128::from(0i64),
            CudaDeviceVarKind::Surface => i128::from(1i64),
            CudaDeviceVarKind::Texture => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for CudaDeviceVarKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Variable" => Ok(Self::Variable),
            "Surface" => Ok(Self::Surface),
            "Texture" => Ok(Self::Texture),
            _ => Err(ParseEnumError::new("CudaDeviceVarKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for CudaDeviceVarKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Variable => "Variable",
            Self::Surface => "Surface",
            Self::Texture => "Texture",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CudaDeviceVarKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CudaDeviceVarKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CudaDeviceVarKind", format!("{other:?}"))),
        }
    }
}
/// calling convention
/// Enum cases:
/// * c (`C`)
/// * ptx_kernel (`PTXKernel`)
/// * spir_function (`SpirFunction`)
/// * spir_kernel (`SpirKernel`)
/// * amdgpu_kernel (`AMDGPUKernel`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CallingConv {
    C = 0i64,
    PtxKernel = 1i64,
    SpirFunction = 2i64,
    SpirKernel = 3i64,
    AmdgpuKernel = 4i64,
}
impl TryFrom<i128> for CallingConv {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::C),
            1i64 => Ok(Self::PtxKernel),
            2i64 => Ok(Self::SpirFunction),
            3i64 => Ok(Self::SpirKernel),
            4i64 => Ok(Self::AmdgpuKernel),
            _ => Err(ParseEnumError::new("CallingConv", value.to_string())),
        }
    }
}
impl From<CallingConv> for i128 {
    fn from(value: CallingConv) -> i128 {
        match value {
            CallingConv::C => i128::from(0i64),
            CallingConv::PtxKernel => i128::from(1i64),
            CallingConv::SpirFunction => i128::from(2i64),
            CallingConv::SpirKernel => i128::from(3i64),
            CallingConv::AmdgpuKernel => i128::from(4i64),
        }
    }
}
impl std::str::FromStr for CallingConv {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(Self::C),
            "ptx_kernel" => Ok(Self::PtxKernel),
            "spir_function" => Ok(Self::SpirFunction),
            "spir_kernel" => Ok(Self::SpirKernel),
            "amdgpu_kernel" => Ok(Self::AmdgpuKernel),
            _ => Err(ParseEnumError::new("CallingConv", s.to_string())),
        }
    }
}
impl std::fmt::Display for CallingConv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::C => "c",
            Self::PtxKernel => "ptx_kernel",
            Self::SpirFunction => "spir_function",
            Self::SpirKernel => "spir_kernel",
            Self::AmdgpuKernel => "amdgpu_kernel",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CallingConv {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CallingConv", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CallingConv", format!("{other:?}"))),
        }
    }
}
/// case kind
/// Enum cases:
/// * default (`Default`)
/// * equal (`Equal`)
/// * anyof (`Anyof`)
/// * range (`Range`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CaseOpKind {
    Default = 0i64,
    Equal = 1i64,
    Anyof = 2i64,
    Range = 3i64,
}
impl TryFrom<i128> for CaseOpKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Default),
            1i64 => Ok(Self::Equal),
            2i64 => Ok(Self::Anyof),
            3i64 => Ok(Self::Range),
            _ => Err(ParseEnumError::new("CaseOpKind", value.to_string())),
        }
    }
}
impl From<CaseOpKind> for i128 {
    fn from(value: CaseOpKind) -> i128 {
        match value {
            CaseOpKind::Default => i128::from(0i64),
            CaseOpKind::Equal => i128::from(1i64),
            CaseOpKind::Anyof => i128::from(2i64),
            CaseOpKind::Range => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for CaseOpKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "equal" => Ok(Self::Equal),
            "anyof" => Ok(Self::Anyof),
            "range" => Ok(Self::Range),
            _ => Err(ParseEnumError::new("CaseOpKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for CaseOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Default => "default",
            Self::Equal => "equal",
            Self::Anyof => "anyof",
            Self::Range => "range",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CaseOpKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CaseOpKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CaseOpKind", format!("{other:?}"))),
        }
    }
}
/// cast kind
/// Enum cases:
/// * bitcast (`bitcast`)
/// * array_to_ptrdecay (`array_to_ptrdecay`)
/// * member_ptr_to_bool (`member_ptr_to_bool`)
/// * int_to_ptr (`int_to_ptr`)
/// * ptr_to_int (`ptr_to_int`)
/// * ptr_to_bool (`ptr_to_bool`)
/// * integral (`integral`)
/// * int_to_bool (`int_to_bool`)
/// * int_to_float (`int_to_float`)
/// * float_to_int (`float_to_int`)
/// * float_to_bool (`float_to_bool`)
/// * bool_to_int (`bool_to_int`)
/// * floating (`floating`)
/// * float_to_complex (`float_to_complex`)
/// * float_complex_to_real (`float_complex_to_real`)
/// * float_complex_to_bool (`float_complex_to_bool`)
/// * float_complex (`float_complex`)
/// * float_complex_to_int_complex (`float_complex_to_int_complex`)
/// * int_to_complex (`int_to_complex`)
/// * int_complex_to_real (`int_complex_to_real`)
/// * int_complex_to_bool (`int_complex_to_bool`)
/// * int_complex (`int_complex`)
/// * int_complex_to_float_complex (`int_complex_to_float_complex`)
/// * address_space (`address_space`)
/// * bool_to_float (`bool_to_float`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CastKind {
    Bitcast = 1i64,
    ArrayToPtrdecay = 11i64,
    MemberPtrToBool = 17i64,
    IntToPtr = 21i64,
    PtrToInt = 22i64,
    PtrToBool = 23i64,
    Integral = 27i64,
    IntToBool = 28i64,
    IntToFloat = 29i64,
    FloatToInt = 36i64,
    FloatToBool = 37i64,
    BoolToInt = 38i64,
    Floating = 39i64,
    FloatToComplex = 44i64,
    FloatComplexToReal = 45i64,
    FloatComplexToBool = 46i64,
    FloatComplex = 47i64,
    FloatComplexToIntComplex = 48i64,
    IntToComplex = 49i64,
    IntComplexToReal = 50i64,
    IntComplexToBool = 51i64,
    IntComplex = 52i64,
    IntComplexToFloatComplex = 53i64,
    AddressSpace = 63i64,
    BoolToFloat = 1000i64,
}
impl TryFrom<i128> for CastKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            1i64 => Ok(Self::Bitcast),
            11i64 => Ok(Self::ArrayToPtrdecay),
            17i64 => Ok(Self::MemberPtrToBool),
            21i64 => Ok(Self::IntToPtr),
            22i64 => Ok(Self::PtrToInt),
            23i64 => Ok(Self::PtrToBool),
            27i64 => Ok(Self::Integral),
            28i64 => Ok(Self::IntToBool),
            29i64 => Ok(Self::IntToFloat),
            36i64 => Ok(Self::FloatToInt),
            37i64 => Ok(Self::FloatToBool),
            38i64 => Ok(Self::BoolToInt),
            39i64 => Ok(Self::Floating),
            44i64 => Ok(Self::FloatToComplex),
            45i64 => Ok(Self::FloatComplexToReal),
            46i64 => Ok(Self::FloatComplexToBool),
            47i64 => Ok(Self::FloatComplex),
            48i64 => Ok(Self::FloatComplexToIntComplex),
            49i64 => Ok(Self::IntToComplex),
            50i64 => Ok(Self::IntComplexToReal),
            51i64 => Ok(Self::IntComplexToBool),
            52i64 => Ok(Self::IntComplex),
            53i64 => Ok(Self::IntComplexToFloatComplex),
            63i64 => Ok(Self::AddressSpace),
            1000i64 => Ok(Self::BoolToFloat),
            _ => Err(ParseEnumError::new("CastKind", value.to_string())),
        }
    }
}
impl From<CastKind> for i128 {
    fn from(value: CastKind) -> i128 {
        match value {
            CastKind::Bitcast => i128::from(1i64),
            CastKind::ArrayToPtrdecay => i128::from(11i64),
            CastKind::MemberPtrToBool => i128::from(17i64),
            CastKind::IntToPtr => i128::from(21i64),
            CastKind::PtrToInt => i128::from(22i64),
            CastKind::PtrToBool => i128::from(23i64),
            CastKind::Integral => i128::from(27i64),
            CastKind::IntToBool => i128::from(28i64),
            CastKind::IntToFloat => i128::from(29i64),
            CastKind::FloatToInt => i128::from(36i64),
            CastKind::FloatToBool => i128::from(37i64),
            CastKind::BoolToInt => i128::from(38i64),
            CastKind::Floating => i128::from(39i64),
            CastKind::FloatToComplex => i128::from(44i64),
            CastKind::FloatComplexToReal => i128::from(45i64),
            CastKind::FloatComplexToBool => i128::from(46i64),
            CastKind::FloatComplex => i128::from(47i64),
            CastKind::FloatComplexToIntComplex => i128::from(48i64),
            CastKind::IntToComplex => i128::from(49i64),
            CastKind::IntComplexToReal => i128::from(50i64),
            CastKind::IntComplexToBool => i128::from(51i64),
            CastKind::IntComplex => i128::from(52i64),
            CastKind::IntComplexToFloatComplex => i128::from(53i64),
            CastKind::AddressSpace => i128::from(63i64),
            CastKind::BoolToFloat => i128::from(1000i64),
        }
    }
}
impl std::str::FromStr for CastKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitcast" => Ok(Self::Bitcast),
            "array_to_ptrdecay" => Ok(Self::ArrayToPtrdecay),
            "member_ptr_to_bool" => Ok(Self::MemberPtrToBool),
            "int_to_ptr" => Ok(Self::IntToPtr),
            "ptr_to_int" => Ok(Self::PtrToInt),
            "ptr_to_bool" => Ok(Self::PtrToBool),
            "integral" => Ok(Self::Integral),
            "int_to_bool" => Ok(Self::IntToBool),
            "int_to_float" => Ok(Self::IntToFloat),
            "float_to_int" => Ok(Self::FloatToInt),
            "float_to_bool" => Ok(Self::FloatToBool),
            "bool_to_int" => Ok(Self::BoolToInt),
            "floating" => Ok(Self::Floating),
            "float_to_complex" => Ok(Self::FloatToComplex),
            "float_complex_to_real" => Ok(Self::FloatComplexToReal),
            "float_complex_to_bool" => Ok(Self::FloatComplexToBool),
            "float_complex" => Ok(Self::FloatComplex),
            "float_complex_to_int_complex" => Ok(Self::FloatComplexToIntComplex),
            "int_to_complex" => Ok(Self::IntToComplex),
            "int_complex_to_real" => Ok(Self::IntComplexToReal),
            "int_complex_to_bool" => Ok(Self::IntComplexToBool),
            "int_complex" => Ok(Self::IntComplex),
            "int_complex_to_float_complex" => Ok(Self::IntComplexToFloatComplex),
            "address_space" => Ok(Self::AddressSpace),
            "bool_to_float" => Ok(Self::BoolToFloat),
            _ => Err(ParseEnumError::new("CastKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for CastKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Bitcast => "bitcast",
            Self::ArrayToPtrdecay => "array_to_ptrdecay",
            Self::MemberPtrToBool => "member_ptr_to_bool",
            Self::IntToPtr => "int_to_ptr",
            Self::PtrToInt => "ptr_to_int",
            Self::PtrToBool => "ptr_to_bool",
            Self::Integral => "integral",
            Self::IntToBool => "int_to_bool",
            Self::IntToFloat => "int_to_float",
            Self::FloatToInt => "float_to_int",
            Self::FloatToBool => "float_to_bool",
            Self::BoolToInt => "bool_to_int",
            Self::Floating => "floating",
            Self::FloatToComplex => "float_to_complex",
            Self::FloatComplexToReal => "float_complex_to_real",
            Self::FloatComplexToBool => "float_complex_to_bool",
            Self::FloatComplex => "float_complex",
            Self::FloatComplexToIntComplex => "float_complex_to_int_complex",
            Self::IntToComplex => "int_to_complex",
            Self::IntComplexToReal => "int_complex_to_real",
            Self::IntComplexToBool => "int_complex_to_bool",
            Self::IntComplex => "int_complex",
            Self::IntComplexToFloatComplex => "int_complex_to_float_complex",
            Self::AddressSpace => "address_space",
            Self::BoolToFloat => "bool_to_float",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CastKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CastKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CastKind", format!("{other:?}"))),
        }
    }
}
/// cleanup kind
/// Enum cases:
/// * normal (`Normal`)
/// * eh (`EH`)
/// * all (`All`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CleanupKind {
    Normal = 1i64,
    Eh = 2i64,
    All = 3i64,
}
impl TryFrom<i128> for CleanupKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            1i64 => Ok(Self::Normal),
            2i64 => Ok(Self::Eh),
            3i64 => Ok(Self::All),
            _ => Err(ParseEnumError::new("CleanupKind", value.to_string())),
        }
    }
}
impl From<CleanupKind> for i128 {
    fn from(value: CleanupKind) -> i128 {
        match value {
            CleanupKind::Normal => i128::from(1i64),
            CleanupKind::Eh => i128::from(2i64),
            CleanupKind::All => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for CleanupKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "eh" => Ok(Self::Eh),
            "all" => Ok(Self::All),
            _ => Err(ParseEnumError::new("CleanupKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for CleanupKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Normal => "normal",
            Self::Eh => "eh",
            Self::All => "all",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CleanupKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CleanupKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CleanupKind", format!("{other:?}"))),
        }
    }
}
/// compare operation kind
/// Enum cases:
/// * lt (`lt`)
/// * le (`le`)
/// * gt (`gt`)
/// * ge (`ge`)
/// * eq (`eq`)
/// * ne (`ne`)
/// * one (`one`)
/// * uno (`uno`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CmpOpKind {
    Lt = 0i64,
    Le = 1i64,
    Gt = 2i64,
    Ge = 3i64,
    Eq = 4i64,
    Ne = 5i64,
    One = 6i64,
    Uno = 7i64,
}
impl TryFrom<i128> for CmpOpKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Lt),
            1i64 => Ok(Self::Le),
            2i64 => Ok(Self::Gt),
            3i64 => Ok(Self::Ge),
            4i64 => Ok(Self::Eq),
            5i64 => Ok(Self::Ne),
            6i64 => Ok(Self::One),
            7i64 => Ok(Self::Uno),
            _ => Err(ParseEnumError::new("CmpOpKind", value.to_string())),
        }
    }
}
impl From<CmpOpKind> for i128 {
    fn from(value: CmpOpKind) -> i128 {
        match value {
            CmpOpKind::Lt => i128::from(0i64),
            CmpOpKind::Le => i128::from(1i64),
            CmpOpKind::Gt => i128::from(2i64),
            CmpOpKind::Ge => i128::from(3i64),
            CmpOpKind::Eq => i128::from(4i64),
            CmpOpKind::Ne => i128::from(5i64),
            CmpOpKind::One => i128::from(6i64),
            CmpOpKind::Uno => i128::from(7i64),
        }
    }
}
impl std::str::FromStr for CmpOpKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lt" => Ok(Self::Lt),
            "le" => Ok(Self::Le),
            "gt" => Ok(Self::Gt),
            "ge" => Ok(Self::Ge),
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "one" => Ok(Self::One),
            "uno" => Ok(Self::Uno),
            _ => Err(ParseEnumError::new("CmpOpKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for CmpOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Gt => "gt",
            Self::Ge => "ge",
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::One => "one",
            Self::Uno => "uno",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CmpOpKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CmpOpKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CmpOpKind", format!("{other:?}"))),
        }
    }
}
/// three-way comparison ordering kind
/// Enum cases:
/// * strong (`Strong`)
/// * weak (`Weak`)
/// * partial (`Partial`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CmpOrdering {
    Strong = 0i64,
    Weak = 1i64,
    Partial = 2i64,
}
impl TryFrom<i128> for CmpOrdering {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Strong),
            1i64 => Ok(Self::Weak),
            2i64 => Ok(Self::Partial),
            _ => Err(ParseEnumError::new("CmpOrdering", value.to_string())),
        }
    }
}
impl From<CmpOrdering> for i128 {
    fn from(value: CmpOrdering) -> i128 {
        match value {
            CmpOrdering::Strong => i128::from(0i64),
            CmpOrdering::Weak => i128::from(1i64),
            CmpOrdering::Partial => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for CmpOrdering {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "strong" => Ok(Self::Strong),
            "weak" => Ok(Self::Weak),
            "partial" => Ok(Self::Partial),
            _ => Err(ParseEnumError::new("CmpOrdering", s.to_string())),
        }
    }
}
impl std::fmt::Display for CmpOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Strong => "strong",
            Self::Weak => "weak",
            Self::Partial => "partial",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CmpOrdering {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CmpOrdering", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CmpOrdering", format!("{other:?}"))),
        }
    }
}
/// complex multiplication and division implementation
/// Enum cases:
/// * full (`Full`)
/// * improved (`Improved`)
/// * promoted (`Promoted`)
/// * basic (`Basic`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum ComplexRangeKind {
    Full = 0i64,
    Improved = 1i64,
    Promoted = 2i64,
    Basic = 3i64,
}
impl TryFrom<i128> for ComplexRangeKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Full),
            1i64 => Ok(Self::Improved),
            2i64 => Ok(Self::Promoted),
            3i64 => Ok(Self::Basic),
            _ => Err(ParseEnumError::new("ComplexRangeKind", value.to_string())),
        }
    }
}
impl From<ComplexRangeKind> for i128 {
    fn from(value: ComplexRangeKind) -> i128 {
        match value {
            ComplexRangeKind::Full => i128::from(0i64),
            ComplexRangeKind::Improved => i128::from(1i64),
            ComplexRangeKind::Promoted => i128::from(2i64),
            ComplexRangeKind::Basic => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for ComplexRangeKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(Self::Full),
            "improved" => Ok(Self::Improved),
            "promoted" => Ok(Self::Promoted),
            "basic" => Ok(Self::Basic),
            _ => Err(ParseEnumError::new("ComplexRangeKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for ComplexRangeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Full => "full",
            Self::Improved => "improved",
            Self::Promoted => "promoted",
            Self::Basic => "basic",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for ComplexRangeKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("ComplexRangeKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("ComplexRangeKind", format!("{other:?}"))),
        }
    }
}
/// CXX Constructor Kind
/// Enum cases:
/// * custom (`Custom`)
/// * default (`Default`)
/// * copy (`Copy`)
/// * move (`Move`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum CtorKind {
    Custom = 0i64,
    Default = 1i64,
    Copy = 2i64,
    Move = 3i64,
}
impl TryFrom<i128> for CtorKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Custom),
            1i64 => Ok(Self::Default),
            2i64 => Ok(Self::Copy),
            3i64 => Ok(Self::Move),
            _ => Err(ParseEnumError::new("CtorKind", value.to_string())),
        }
    }
}
impl From<CtorKind> for i128 {
    fn from(value: CtorKind) -> i128 {
        match value {
            CtorKind::Custom => i128::from(0i64),
            CtorKind::Default => i128::from(1i64),
            CtorKind::Copy => i128::from(2i64),
            CtorKind::Move => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for CtorKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "custom" => Ok(Self::Custom),
            "default" => Ok(Self::Default),
            "copy" => Ok(Self::Copy),
            "move" => Ok(Self::Move),
            _ => Err(ParseEnumError::new("CtorKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for CtorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Custom => "custom",
            Self::Default => "default",
            Self::Copy => "copy",
            Self::Move => "move",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for CtorKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("CtorKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("CtorKind", format!("{other:?}"))),
        }
    }
}
/// dynamic cast kind
/// Enum cases:
/// * ptr (`Ptr`)
/// * ref (`Ref`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum DynamicCastKind {
    Ptr = 0i64,
    Ref = 1i64,
}
impl TryFrom<i128> for DynamicCastKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Ptr),
            1i64 => Ok(Self::Ref),
            _ => Err(ParseEnumError::new("DynamicCastKind", value.to_string())),
        }
    }
}
impl From<DynamicCastKind> for i128 {
    fn from(value: DynamicCastKind) -> i128 {
        match value {
            DynamicCastKind::Ptr => i128::from(0i64),
            DynamicCastKind::Ref => i128::from(1i64),
        }
    }
}
impl std::str::FromStr for DynamicCastKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ptr" => Ok(Self::Ptr),
            "ref" => Ok(Self::Ref),
            _ => Err(ParseEnumError::new("DynamicCastKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for DynamicCastKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Ptr => "ptr",
            Self::Ref => "ref",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for DynamicCastKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("DynamicCastKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("DynamicCastKind", format!("{other:?}"))),
        }
    }
}
/// floating-point dynamic rounding mode
///
/// The known dynamic rounding mode at the point the instruction is executed.
/// If the actual dynamic rounding mode differs from this value, the behavior
/// is undefined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum FpDynamicRoundingMode {
    ToNearest = 0i64,
    Downward = 1i64,
    Upward = 2i64,
    UpwardZero = 3i64,
    ToNearestAway = 4i64,
    Unknown = 7i64,
}
impl TryFrom<i128> for FpDynamicRoundingMode {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::ToNearest),
            1i64 => Ok(Self::Downward),
            2i64 => Ok(Self::Upward),
            3i64 => Ok(Self::UpwardZero),
            4i64 => Ok(Self::ToNearestAway),
            7i64 => Ok(Self::Unknown),
            _ => Err(ParseEnumError::new("FpDynamicRoundingMode", value.to_string())),
        }
    }
}
impl From<FpDynamicRoundingMode> for i128 {
    fn from(value: FpDynamicRoundingMode) -> i128 {
        match value {
            FpDynamicRoundingMode::ToNearest => i128::from(0i64),
            FpDynamicRoundingMode::Downward => i128::from(1i64),
            FpDynamicRoundingMode::Upward => i128::from(2i64),
            FpDynamicRoundingMode::UpwardZero => i128::from(3i64),
            FpDynamicRoundingMode::ToNearestAway => i128::from(4i64),
            FpDynamicRoundingMode::Unknown => i128::from(7i64),
        }
    }
}
impl std::str::FromStr for FpDynamicRoundingMode {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tonearest" => Ok(Self::ToNearest),
            "downward" => Ok(Self::Downward),
            "upward" => Ok(Self::Upward),
            "upwardzero" => Ok(Self::UpwardZero),
            "tonearestaway" => Ok(Self::ToNearestAway),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ParseEnumError::new("FpDynamicRoundingMode", s.to_string())),
        }
    }
}
impl std::fmt::Display for FpDynamicRoundingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::ToNearest => "tonearest",
            Self::Downward => "downward",
            Self::Upward => "upward",
            Self::UpwardZero => "upwardzero",
            Self::ToNearestAway => "tonearestaway",
            Self::Unknown => "unknown",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for FpDynamicRoundingMode {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new(
                        "FpDynamicRoundingMode",
                        value.clone(),
                    ))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => {
                Err(ParseEnumError::new("FpDynamicRoundingMode", format!("{other:?}")))
            }
        }
    }
}
/// floating-point exception mode
///
/// The known floating-point exception mode at the point the instruction is
/// executed. If the actual exception mode differs from this value, the
/// behavior is undefined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum FpExceptionMode {
    Unknown = 0i64,
    Masked = 1i64,
    Unmasked = 2i64,
}
impl TryFrom<i128> for FpExceptionMode {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Unknown),
            1i64 => Ok(Self::Masked),
            2i64 => Ok(Self::Unmasked),
            _ => Err(ParseEnumError::new("FpExceptionMode", value.to_string())),
        }
    }
}
impl From<FpExceptionMode> for i128 {
    fn from(value: FpExceptionMode) -> i128 {
        match value {
            FpExceptionMode::Unknown => i128::from(0i64),
            FpExceptionMode::Masked => i128::from(1i64),
            FpExceptionMode::Unmasked => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for FpExceptionMode {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "masked" => Ok(Self::Masked),
            "unmasked" => Ok(Self::Unmasked),
            _ => Err(ParseEnumError::new("FpExceptionMode", s.to_string())),
        }
    }
}
impl std::fmt::Display for FpExceptionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Unknown => "unknown",
            Self::Masked => "masked",
            Self::Unmasked => "unmasked",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for FpExceptionMode {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("FpExceptionMode", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("FpExceptionMode", format!("{other:?}"))),
        }
    }
}
/// fast-math flags
///
/// Describes fast-math flags for CIR operations. This attribute is shared by
/// operations with floating-point semantics and is not specific to LLVM intrinsic
/// calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FastMathFlags(pub u64);
impl FastMathFlags {
    pub const None: u64 = 0i64 as u64;
    pub const Nnan: u64 = 1i64 as u64;
    pub const Ninf: u64 = 2i64 as u64;
    pub const Nsz: u64 = 4i64 as u64;
    pub const Arcp: u64 = 8i64 as u64;
    pub const Contract: u64 = 16i64 as u64;
    pub const Afn: u64 = 32i64 as u64;
    pub const Reassoc: u64 = 64i64 as u64;
    pub const Fast: u64 = 127i64 as u64;
}
impl TryFrom<&crate::attrs::Attribute> for FastMathFlags {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::FastMathFlags { value } => Ok(*value),
            crate::attrs::Attribute::Int { value, .. } => {
                u64::try_from(*value)
                    .map(Self)
                    .map_err(|_| ParseEnumError::new(
                        stringify!(FastMathFlags),
                        value.to_string(),
                    ))
            }
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<u64>()
                    .map(Self)
                    .map_err(|_| ParseEnumError::new(
                        stringify!(FastMathFlags),
                        value.clone(),
                    ))
            }
            other => {
                Err(ParseEnumError::new(stringify!(FastMathFlags), format!("{other:?}")))
            }
        }
    }
}
/// linkage kind
/// Enum cases:
/// * external (`ExternalLinkage`)
/// * available_externally (`AvailableExternallyLinkage`)
/// * linkonce (`LinkOnceAnyLinkage`)
/// * linkonce_odr (`LinkOnceODRLinkage`)
/// * weak (`WeakAnyLinkage`)
/// * weak_odr (`WeakODRLinkage`)
/// * appending (`AppendingLinkage`)
/// * internal (`InternalLinkage`)
/// * cir_private (`PrivateLinkage`)
/// * extern_weak (`ExternalWeakLinkage`)
/// * common (`CommonLinkage`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum GlobalLinkageKind {
    External = 0i64,
    AvailableExternally = 1i64,
    LinkOnceAny = 2i64,
    LinkOnceOdr = 3i64,
    WeakAny = 4i64,
    WeakOdr = 5i64,
    Appending = 6i64,
    Internal = 7i64,
    Private = 8i64,
    ExternalWeak = 9i64,
    Common = 10i64,
}
impl TryFrom<i128> for GlobalLinkageKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::External),
            1i64 => Ok(Self::AvailableExternally),
            2i64 => Ok(Self::LinkOnceAny),
            3i64 => Ok(Self::LinkOnceOdr),
            4i64 => Ok(Self::WeakAny),
            5i64 => Ok(Self::WeakOdr),
            6i64 => Ok(Self::Appending),
            7i64 => Ok(Self::Internal),
            8i64 => Ok(Self::Private),
            9i64 => Ok(Self::ExternalWeak),
            10i64 => Ok(Self::Common),
            _ => Err(ParseEnumError::new("GlobalLinkageKind", value.to_string())),
        }
    }
}
impl From<GlobalLinkageKind> for i128 {
    fn from(value: GlobalLinkageKind) -> i128 {
        match value {
            GlobalLinkageKind::External => i128::from(0i64),
            GlobalLinkageKind::AvailableExternally => i128::from(1i64),
            GlobalLinkageKind::LinkOnceAny => i128::from(2i64),
            GlobalLinkageKind::LinkOnceOdr => i128::from(3i64),
            GlobalLinkageKind::WeakAny => i128::from(4i64),
            GlobalLinkageKind::WeakOdr => i128::from(5i64),
            GlobalLinkageKind::Appending => i128::from(6i64),
            GlobalLinkageKind::Internal => i128::from(7i64),
            GlobalLinkageKind::Private => i128::from(8i64),
            GlobalLinkageKind::ExternalWeak => i128::from(9i64),
            GlobalLinkageKind::Common => i128::from(10i64),
        }
    }
}
impl std::str::FromStr for GlobalLinkageKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "external" => Ok(Self::External),
            "available_externally" => Ok(Self::AvailableExternally),
            "linkonce" => Ok(Self::LinkOnceAny),
            "linkonce_odr" => Ok(Self::LinkOnceOdr),
            "weak" => Ok(Self::WeakAny),
            "weak_odr" => Ok(Self::WeakOdr),
            "appending" => Ok(Self::Appending),
            "internal" => Ok(Self::Internal),
            "cir_private" => Ok(Self::Private),
            "extern_weak" => Ok(Self::ExternalWeak),
            "common" => Ok(Self::Common),
            _ => Err(ParseEnumError::new("GlobalLinkageKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for GlobalLinkageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::External => "external",
            Self::AvailableExternally => "available_externally",
            Self::LinkOnceAny => "linkonce",
            Self::LinkOnceOdr => "linkonce_odr",
            Self::WeakAny => "weak",
            Self::WeakOdr => "weak_odr",
            Self::Appending => "appending",
            Self::Internal => "internal",
            Self::Private => "cir_private",
            Self::ExternalWeak => "extern_weak",
            Self::Common => "common",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for GlobalLinkageKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("GlobalLinkageKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("GlobalLinkageKind", format!("{other:?}"))),
        }
    }
}
/// allowed i32 cases: 0, 1, 2, 3, 4, 5, 6, 7
/// Enum cases:
/// * reference (`Reference`)
/// * pointer (`Pointer`)
/// * scalar (`Scalar`)
/// * objc (`Objc`)
/// * trivial_copy (`TrivialCopy`)
/// * non_trivial_copy (`NonTrivialCopy`)
/// * reference_to_pointer (`ReferenceToPointer`)
/// * reference_to_record_pointer (`ReferenceToRecordPointer`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum InitCatchKind {
    Reference = 0i64,
    Pointer = 1i64,
    Scalar = 2i64,
    Objc = 3i64,
    TrivialCopy = 4i64,
    NonTrivialCopy = 5i64,
    ReferenceToPointer = 6i64,
    ReferenceToRecordPointer = 7i64,
}
impl TryFrom<i128> for InitCatchKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Reference),
            1i64 => Ok(Self::Pointer),
            2i64 => Ok(Self::Scalar),
            3i64 => Ok(Self::Objc),
            4i64 => Ok(Self::TrivialCopy),
            5i64 => Ok(Self::NonTrivialCopy),
            6i64 => Ok(Self::ReferenceToPointer),
            7i64 => Ok(Self::ReferenceToRecordPointer),
            _ => Err(ParseEnumError::new("InitCatchKind", value.to_string())),
        }
    }
}
impl From<InitCatchKind> for i128 {
    fn from(value: InitCatchKind) -> i128 {
        match value {
            InitCatchKind::Reference => i128::from(0i64),
            InitCatchKind::Pointer => i128::from(1i64),
            InitCatchKind::Scalar => i128::from(2i64),
            InitCatchKind::Objc => i128::from(3i64),
            InitCatchKind::TrivialCopy => i128::from(4i64),
            InitCatchKind::NonTrivialCopy => i128::from(5i64),
            InitCatchKind::ReferenceToPointer => i128::from(6i64),
            InitCatchKind::ReferenceToRecordPointer => i128::from(7i64),
        }
    }
}
impl std::str::FromStr for InitCatchKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reference" => Ok(Self::Reference),
            "pointer" => Ok(Self::Pointer),
            "scalar" => Ok(Self::Scalar),
            "objc" => Ok(Self::Objc),
            "trivial_copy" => Ok(Self::TrivialCopy),
            "non_trivial_copy" => Ok(Self::NonTrivialCopy),
            "reference_to_pointer" => Ok(Self::ReferenceToPointer),
            "reference_to_record_pointer" => Ok(Self::ReferenceToRecordPointer),
            _ => Err(ParseEnumError::new("InitCatchKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for InitCatchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Reference => "reference",
            Self::Pointer => "pointer",
            Self::Scalar => "scalar",
            Self::Objc => "objc",
            Self::TrivialCopy => "trivial_copy",
            Self::NonTrivialCopy => "non_trivial_copy",
            Self::ReferenceToPointer => "reference_to_pointer",
            Self::ReferenceToRecordPointer => "reference_to_record_pointer",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for InitCatchKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("InitCatchKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("InitCatchKind", format!("{other:?}"))),
        }
    }
}
/// inline kind
/// Enum cases:
/// * no_inline (`NoInline`)
/// * always_inline (`AlwaysInline`)
/// * inline_hint (`InlineHint`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum InlineKind {
    NoInline = 1i64,
    AlwaysInline = 2i64,
    InlineHint = 3i64,
}
impl TryFrom<i128> for InlineKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            1i64 => Ok(Self::NoInline),
            2i64 => Ok(Self::AlwaysInline),
            3i64 => Ok(Self::InlineHint),
            _ => Err(ParseEnumError::new("InlineKind", value.to_string())),
        }
    }
}
impl From<InlineKind> for i128 {
    fn from(value: InlineKind) -> i128 {
        match value {
            InlineKind::NoInline => i128::from(1i64),
            InlineKind::AlwaysInline => i128::from(2i64),
            InlineKind::InlineHint => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for InlineKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no_inline" => Ok(Self::NoInline),
            "always_inline" => Ok(Self::AlwaysInline),
            "inline_hint" => Ok(Self::InlineHint),
            _ => Err(ParseEnumError::new("InlineKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for InlineKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::NoInline => "no_inline",
            Self::AlwaysInline => "always_inline",
            Self::InlineHint => "inline_hint",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for InlineKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("InlineKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("InlineKind", format!("{other:?}"))),
        }
    }
}
/// known standard library entity
/// Enum cases:
/// * std::find (`StdFind`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum KnownFuncKind {
    StdFind = 1i64,
}
impl TryFrom<i128> for KnownFuncKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            1i64 => Ok(Self::StdFind),
            _ => Err(ParseEnumError::new("KnownFuncKind", value.to_string())),
        }
    }
}
impl From<KnownFuncKind> for i128 {
    fn from(value: KnownFuncKind) -> i128 {
        match value {
            KnownFuncKind::StdFind => i128::from(1i64),
        }
    }
}
impl std::str::FromStr for KnownFuncKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "std::find" => Ok(Self::StdFind),
            _ => Err(ParseEnumError::new("KnownFuncKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for KnownFuncKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::StdFind => "std::find",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for KnownFuncKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("KnownFuncKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("KnownFuncKind", format!("{other:?}"))),
        }
    }
}
/// language address space kind
///
/// Enumerates language-specific address spaces used by CIR. These represent
/// semantic qualifiers from source languages (e.g., CUDA `__shared__`,
/// OpenCL `__local`) before target lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum LangAddressSpace {
    Default = 0i64,
    OffloadPrivate = 1i64,
    OffloadLocal = 2i64,
    OffloadGlobal = 3i64,
    OffloadConstant = 4i64,
    OffloadGeneric = 5i64,
    OffloadGlobalDevice = 6i64,
    OffloadGlobalHost = 7i64,
}
impl TryFrom<i128> for LangAddressSpace {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Default),
            1i64 => Ok(Self::OffloadPrivate),
            2i64 => Ok(Self::OffloadLocal),
            3i64 => Ok(Self::OffloadGlobal),
            4i64 => Ok(Self::OffloadConstant),
            5i64 => Ok(Self::OffloadGeneric),
            6i64 => Ok(Self::OffloadGlobalDevice),
            7i64 => Ok(Self::OffloadGlobalHost),
            _ => Err(ParseEnumError::new("LangAddressSpace", value.to_string())),
        }
    }
}
impl From<LangAddressSpace> for i128 {
    fn from(value: LangAddressSpace) -> i128 {
        match value {
            LangAddressSpace::Default => i128::from(0i64),
            LangAddressSpace::OffloadPrivate => i128::from(1i64),
            LangAddressSpace::OffloadLocal => i128::from(2i64),
            LangAddressSpace::OffloadGlobal => i128::from(3i64),
            LangAddressSpace::OffloadConstant => i128::from(4i64),
            LangAddressSpace::OffloadGeneric => i128::from(5i64),
            LangAddressSpace::OffloadGlobalDevice => i128::from(6i64),
            LangAddressSpace::OffloadGlobalHost => i128::from(7i64),
        }
    }
}
impl std::str::FromStr for LangAddressSpace {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "offload_private" => Ok(Self::OffloadPrivate),
            "offload_local" => Ok(Self::OffloadLocal),
            "offload_global" => Ok(Self::OffloadGlobal),
            "offload_constant" => Ok(Self::OffloadConstant),
            "offload_generic" => Ok(Self::OffloadGeneric),
            "offload_global_device" => Ok(Self::OffloadGlobalDevice),
            "offload_global_host" => Ok(Self::OffloadGlobalHost),
            _ => Err(ParseEnumError::new("LangAddressSpace", s.to_string())),
        }
    }
}
impl std::fmt::Display for LangAddressSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Default => "default",
            Self::OffloadPrivate => "offload_private",
            Self::OffloadLocal => "offload_local",
            Self::OffloadGlobal => "offload_global",
            Self::OffloadConstant => "offload_constant",
            Self::OffloadGeneric => "offload_generic",
            Self::OffloadGlobalDevice => "offload_global_device",
            Self::OffloadGlobalHost => "offload_global_host",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for LangAddressSpace {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("LangAddressSpace", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("LangAddressSpace", format!("{other:?}"))),
        }
    }
}
/// Memory order according to C++11 memory model
/// Enum cases:
/// * relaxed (`Relaxed`)
/// * consume (`Consume`)
/// * acquire (`Acquire`)
/// * release (`Release`)
/// * acq_rel (`AcquireRelease`)
/// * seq_cst (`SequentiallyConsistent`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum MemOrder {
    Relaxed = 0i64,
    Consume = 1i64,
    Acquire = 2i64,
    Release = 3i64,
    AcquireRelease = 4i64,
    SequentiallyConsistent = 5i64,
}
impl TryFrom<i128> for MemOrder {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Relaxed),
            1i64 => Ok(Self::Consume),
            2i64 => Ok(Self::Acquire),
            3i64 => Ok(Self::Release),
            4i64 => Ok(Self::AcquireRelease),
            5i64 => Ok(Self::SequentiallyConsistent),
            _ => Err(ParseEnumError::new("MemOrder", value.to_string())),
        }
    }
}
impl From<MemOrder> for i128 {
    fn from(value: MemOrder) -> i128 {
        match value {
            MemOrder::Relaxed => i128::from(0i64),
            MemOrder::Consume => i128::from(1i64),
            MemOrder::Acquire => i128::from(2i64),
            MemOrder::Release => i128::from(3i64),
            MemOrder::AcquireRelease => i128::from(4i64),
            MemOrder::SequentiallyConsistent => i128::from(5i64),
        }
    }
}
impl std::str::FromStr for MemOrder {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "relaxed" => Ok(Self::Relaxed),
            "consume" => Ok(Self::Consume),
            "acquire" => Ok(Self::Acquire),
            "release" => Ok(Self::Release),
            "acq_rel" => Ok(Self::AcquireRelease),
            "seq_cst" => Ok(Self::SequentiallyConsistent),
            _ => Err(ParseEnumError::new("MemOrder", s.to_string())),
        }
    }
}
impl std::fmt::Display for MemOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Relaxed => "relaxed",
            Self::Consume => "consume",
            Self::Acquire => "acquire",
            Self::Release => "release",
            Self::AcquireRelease => "acq_rel",
            Self::SequentiallyConsistent => "seq_cst",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for MemOrder {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("MemOrder", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("MemOrder", format!("{other:?}"))),
        }
    }
}
/// whether memory may be read and/or written
///
/// The access a function or callee may perform on a class of memory.  Which
/// class is given by the `#cir.memory_effects` parameter that holds the
/// value.  The possible values are:
///
/// - `none`: the memory is neither read nor written.
/// - `read`: the memory may be read, but it is not written.
/// - `write`: the memory may be written, but it is not read.
/// - `readwrite`: the memory may be both read and written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum ModRefInfo {
    NoModRef = 0i64,
    Ref = 1i64,
    Mod = 2i64,
    ModRef = 3i64,
}
impl TryFrom<i128> for ModRefInfo {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::NoModRef),
            1i64 => Ok(Self::Ref),
            2i64 => Ok(Self::Mod),
            3i64 => Ok(Self::ModRef),
            _ => Err(ParseEnumError::new("ModRefInfo", value.to_string())),
        }
    }
}
impl From<ModRefInfo> for i128 {
    fn from(value: ModRefInfo) -> i128 {
        match value {
            ModRefInfo::NoModRef => i128::from(0i64),
            ModRefInfo::Ref => i128::from(1i64),
            ModRefInfo::Mod => i128::from(2i64),
            ModRefInfo::ModRef => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for ModRefInfo {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::NoModRef),
            "read" => Ok(Self::Ref),
            "write" => Ok(Self::Mod),
            "readwrite" => Ok(Self::ModRef),
            _ => Err(ParseEnumError::new("ModRefInfo", s.to_string())),
        }
    }
}
impl std::fmt::Display for ModRefInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::NoModRef => "none",
            Self::Ref => "read",
            Self::Mod => "write",
            Self::ModRef => "readwrite",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for ModRefInfo {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("ModRefInfo", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("ModRefInfo", format!("{other:?}"))),
        }
    }
}
/// offload kind
/// Enum cases:
/// * host (`Host`)
/// * device (`Device`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum OffloadKind {
    Host = 0i64,
    Device = 1i64,
}
impl TryFrom<i128> for OffloadKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Host),
            1i64 => Ok(Self::Device),
            _ => Err(ParseEnumError::new("OffloadKind", value.to_string())),
        }
    }
}
impl From<OffloadKind> for i128 {
    fn from(value: OffloadKind) -> i128 {
        match value {
            OffloadKind::Host => i128::from(0i64),
            OffloadKind::Device => i128::from(1i64),
        }
    }
}
impl std::str::FromStr for OffloadKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "host" => Ok(Self::Host),
            "device" => Ok(Self::Device),
            _ => Err(ParseEnumError::new("OffloadKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for OffloadKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Host => "host",
            Self::Device => "device",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for OffloadKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("OffloadKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("OffloadKind", format!("{other:?}"))),
        }
    }
}
/// what a record member holds
///
/// `pad` is storage the compiler inserted to place a later member at its
/// required offset, and is reusable tail padding when it trails the record.
///
/// `data` and `empty` both span something whose extent the source fixed: a
/// field, a vtable pointer, or a base subobject.  They differ only on whether
/// it holds data for argument passing.
///
/// `bitfield` is a bit-field access unit the source can read at least one
/// bit-field of, and its member type is `!cir.bitfield`.  The mark is needed
/// because a unit can be narrower than the declared type of a bit-field in it:
/// without it, a 32-bit unit holding a bit-field of `long long` would be
/// indistinguishable from an `unsigned` field, and clang's x86-64 lowering
/// coerces two records that differ only in that to `i64` and `i32`.
///
/// A unit of nothing but unnamed bit-fields is `empty`, since no field of the
/// source reads it, even though it still holds data for argument passing.  A
/// zero-width bit-field is unnamed by definition, so it is always `empty`, and
/// it is the only bit-field member that holds no such data.
///
/// A union's base subobject is the one exception to all of this: a single
/// member stands in for every variant, and it takes a bit-field mark as soon
/// as any variant is one, whatever its own storage type came from.
///
/// A record is empty for the ABI when no member is `data` or `bitfield`, which
/// is trivially true for a record with no members, and when no `empty` member
/// is an access unit of unnamed bit-fields.  The distinction between `pad` and
/// the rest is load-bearing beyond that: only `pad` is reusable, so a record
/// whose trailing member is a unit of unnamed bit-fields keeps that unit in
/// its data size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum RecordMemberKind {
    Data = 0i64,
    Pad = 1i64,
    Empty = 2i64,
    BitField = 3i64,
}
impl TryFrom<i128> for RecordMemberKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Data),
            1i64 => Ok(Self::Pad),
            2i64 => Ok(Self::Empty),
            3i64 => Ok(Self::BitField),
            _ => Err(ParseEnumError::new("RecordMemberKind", value.to_string())),
        }
    }
}
impl From<RecordMemberKind> for i128 {
    fn from(value: RecordMemberKind) -> i128 {
        match value {
            RecordMemberKind::Data => i128::from(0i64),
            RecordMemberKind::Pad => i128::from(1i64),
            RecordMemberKind::Empty => i128::from(2i64),
            RecordMemberKind::BitField => i128::from(3i64),
        }
    }
}
impl std::str::FromStr for RecordMemberKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "data" => Ok(Self::Data),
            "pad" => Ok(Self::Pad),
            "empty" => Ok(Self::Empty),
            "bitfield" => Ok(Self::BitField),
            _ => Err(ParseEnumError::new("RecordMemberKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for RecordMemberKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Data => "data",
            Self::Pad => "pad",
            Self::Empty => "empty",
            Self::BitField => "bitfield",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for RecordMemberKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("RecordMemberKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("RecordMemberKind", format!("{other:?}"))),
        }
    }
}
/// source language
/// Enum cases:
/// * c (`C`)
/// * cxx (`CXX`)
/// * opencl_c (`OpenCLC`)
/// * opencl_cxx (`OpenCLCXX`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum SourceLanguage {
    C = 1i64,
    Cxx = 2i64,
    OpenClc = 3i64,
    OpenClcxx = 4i64,
}
impl TryFrom<i128> for SourceLanguage {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            1i64 => Ok(Self::C),
            2i64 => Ok(Self::Cxx),
            3i64 => Ok(Self::OpenClc),
            4i64 => Ok(Self::OpenClcxx),
            _ => Err(ParseEnumError::new("SourceLanguage", value.to_string())),
        }
    }
}
impl From<SourceLanguage> for i128 {
    fn from(value: SourceLanguage) -> i128 {
        match value {
            SourceLanguage::C => i128::from(1i64),
            SourceLanguage::Cxx => i128::from(2i64),
            SourceLanguage::OpenClc => i128::from(3i64),
            SourceLanguage::OpenClcxx => i128::from(4i64),
        }
    }
}
impl std::str::FromStr for SourceLanguage {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(Self::C),
            "cxx" => Ok(Self::Cxx),
            "opencl_c" => Ok(Self::OpenClc),
            "opencl_cxx" => Ok(Self::OpenClcxx),
            _ => Err(ParseEnumError::new("SourceLanguage", s.to_string())),
        }
    }
}
impl std::fmt::Display for SourceLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::C => "c",
            Self::Cxx => "cxx",
            Self::OpenClc => "opencl_c",
            Self::OpenClcxx => "opencl_cxx",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for SourceLanguage {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("SourceLanguage", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("SourceLanguage", format!("{other:?}"))),
        }
    }
}
/// sync scope kind
/// Enum cases:
/// * single_thread (`SingleThread`)
/// * system (`System`)
/// * device (`Device`)
/// * workgroup (`Workgroup`)
/// * wavefront (`Wavefront`)
/// * cluster (`Cluster`)
/// * hip_single_thread (`HIPSingleThread`)
/// * hip_system (`HIPSystem`)
/// * hip_agent (`HIPAgent`)
/// * hip_workgroup (`HIPWorkgroup`)
/// * hip_wavefront (`HIPWavefront`)
/// * hip_cluster (`HIPCluster`)
/// * opencl_work_group (`OpenCLWorkGroup`)
/// * opencl_device (`OpenCLDevice`)
/// * opencl_all_svm_devices (`OpenCLAllSVMDevices`)
/// * opencl_sub_group (`OpenCLSubGroup`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum SyncScopeKind {
    SingleThread = 0i64,
    System = 1i64,
    Device = 2i64,
    Workgroup = 3i64,
    Wavefront = 4i64,
    Cluster = 5i64,
    HipSingleThread = 6i64,
    HipSystem = 7i64,
    HipAgent = 8i64,
    HipWorkgroup = 9i64,
    HipWavefront = 10i64,
    HipCluster = 11i64,
    OpenClWorkGroup = 12i64,
    OpenClDevice = 13i64,
    OpenClAllSvmDevices = 14i64,
    OpenClSubGroup = 15i64,
}
impl TryFrom<i128> for SyncScopeKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::SingleThread),
            1i64 => Ok(Self::System),
            2i64 => Ok(Self::Device),
            3i64 => Ok(Self::Workgroup),
            4i64 => Ok(Self::Wavefront),
            5i64 => Ok(Self::Cluster),
            6i64 => Ok(Self::HipSingleThread),
            7i64 => Ok(Self::HipSystem),
            8i64 => Ok(Self::HipAgent),
            9i64 => Ok(Self::HipWorkgroup),
            10i64 => Ok(Self::HipWavefront),
            11i64 => Ok(Self::HipCluster),
            12i64 => Ok(Self::OpenClWorkGroup),
            13i64 => Ok(Self::OpenClDevice),
            14i64 => Ok(Self::OpenClAllSvmDevices),
            15i64 => Ok(Self::OpenClSubGroup),
            _ => Err(ParseEnumError::new("SyncScopeKind", value.to_string())),
        }
    }
}
impl From<SyncScopeKind> for i128 {
    fn from(value: SyncScopeKind) -> i128 {
        match value {
            SyncScopeKind::SingleThread => i128::from(0i64),
            SyncScopeKind::System => i128::from(1i64),
            SyncScopeKind::Device => i128::from(2i64),
            SyncScopeKind::Workgroup => i128::from(3i64),
            SyncScopeKind::Wavefront => i128::from(4i64),
            SyncScopeKind::Cluster => i128::from(5i64),
            SyncScopeKind::HipSingleThread => i128::from(6i64),
            SyncScopeKind::HipSystem => i128::from(7i64),
            SyncScopeKind::HipAgent => i128::from(8i64),
            SyncScopeKind::HipWorkgroup => i128::from(9i64),
            SyncScopeKind::HipWavefront => i128::from(10i64),
            SyncScopeKind::HipCluster => i128::from(11i64),
            SyncScopeKind::OpenClWorkGroup => i128::from(12i64),
            SyncScopeKind::OpenClDevice => i128::from(13i64),
            SyncScopeKind::OpenClAllSvmDevices => i128::from(14i64),
            SyncScopeKind::OpenClSubGroup => i128::from(15i64),
        }
    }
}
impl std::str::FromStr for SyncScopeKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single_thread" => Ok(Self::SingleThread),
            "system" => Ok(Self::System),
            "device" => Ok(Self::Device),
            "workgroup" => Ok(Self::Workgroup),
            "wavefront" => Ok(Self::Wavefront),
            "cluster" => Ok(Self::Cluster),
            "hip_single_thread" => Ok(Self::HipSingleThread),
            "hip_system" => Ok(Self::HipSystem),
            "hip_agent" => Ok(Self::HipAgent),
            "hip_workgroup" => Ok(Self::HipWorkgroup),
            "hip_wavefront" => Ok(Self::HipWavefront),
            "hip_cluster" => Ok(Self::HipCluster),
            "opencl_work_group" => Ok(Self::OpenClWorkGroup),
            "opencl_device" => Ok(Self::OpenClDevice),
            "opencl_all_svm_devices" => Ok(Self::OpenClAllSvmDevices),
            "opencl_sub_group" => Ok(Self::OpenClSubGroup),
            _ => Err(ParseEnumError::new("SyncScopeKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for SyncScopeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::SingleThread => "single_thread",
            Self::System => "system",
            Self::Device => "device",
            Self::Workgroup => "workgroup",
            Self::Wavefront => "wavefront",
            Self::Cluster => "cluster",
            Self::HipSingleThread => "hip_single_thread",
            Self::HipSystem => "hip_system",
            Self::HipAgent => "hip_agent",
            Self::HipWorkgroup => "hip_workgroup",
            Self::HipWavefront => "hip_wavefront",
            Self::HipCluster => "hip_cluster",
            Self::OpenClWorkGroup => "opencl_work_group",
            Self::OpenClDevice => "opencl_device",
            Self::OpenClAllSvmDevices => "opencl_all_svm_devices",
            Self::OpenClSubGroup => "opencl_sub_group",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for SyncScopeKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("SyncScopeKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("SyncScopeKind", format!("{other:?}"))),
        }
    }
}
/// thread-local storage kind
/// Enum cases:
/// * none (`None`)
/// * static (`Static`)
/// * dynamic (`Dynamic`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum TlsKind {
    None = 0i64,
    Static = 1i64,
    Dynamic = 2i64,
}
impl TryFrom<i128> for TlsKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::None),
            1i64 => Ok(Self::Static),
            2i64 => Ok(Self::Dynamic),
            _ => Err(ParseEnumError::new("TlsKind", value.to_string())),
        }
    }
}
impl From<TlsKind> for i128 {
    fn from(value: TlsKind) -> i128 {
        match value {
            TlsKind::None => i128::from(0i64),
            TlsKind::Static => i128::from(1i64),
            TlsKind::Dynamic => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for TlsKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "static" => Ok(Self::Static),
            "dynamic" => Ok(Self::Dynamic),
            _ => Err(ParseEnumError::new("TlsKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for TlsKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::None => "none",
            Self::Static => "static",
            Self::Dynamic => "dynamic",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for TlsKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("TlsKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("TlsKind", format!("{other:?}"))),
        }
    }
}
/// TLS model
/// Enum cases:
/// * tls_dyn (`GeneralDynamic`)
/// * tls_local_dyn (`LocalDynamic`)
/// * tls_init_exec (`InitialExec`)
/// * tls_local_exec (`LocalExec`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum TlsModel {
    GeneralDynamic = 1i64,
    LocalDynamic = 2i64,
    InitialExec = 3i64,
    LocalExec = 4i64,
}
impl TryFrom<i128> for TlsModel {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            1i64 => Ok(Self::GeneralDynamic),
            2i64 => Ok(Self::LocalDynamic),
            3i64 => Ok(Self::InitialExec),
            4i64 => Ok(Self::LocalExec),
            _ => Err(ParseEnumError::new("TlsModel", value.to_string())),
        }
    }
}
impl From<TlsModel> for i128 {
    fn from(value: TlsModel) -> i128 {
        match value {
            TlsModel::GeneralDynamic => i128::from(1i64),
            TlsModel::LocalDynamic => i128::from(2i64),
            TlsModel::InitialExec => i128::from(3i64),
            TlsModel::LocalExec => i128::from(4i64),
        }
    }
}
impl std::str::FromStr for TlsModel {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tls_dyn" => Ok(Self::GeneralDynamic),
            "tls_local_dyn" => Ok(Self::LocalDynamic),
            "tls_init_exec" => Ok(Self::InitialExec),
            "tls_local_exec" => Ok(Self::LocalExec),
            _ => Err(ParseEnumError::new("TlsModel", s.to_string())),
        }
    }
}
impl std::fmt::Display for TlsModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::GeneralDynamic => "tls_dyn",
            Self::LocalDynamic => "tls_local_dyn",
            Self::InitialExec => "tls_init_exec",
            Self::LocalExec => "tls_local_exec",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for TlsModel {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("TlsModel", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("TlsModel", format!("{other:?}"))),
        }
    }
}
/// template specialization kind
/// Enum cases:
/// * undeclared (`Undeclared`)
/// * implicit_instantiation (`ImplicitInstantiation`)
/// * explicit_specialization (`ExplicitSpecialization`)
/// * explicit_instantiation_declaration (`ExplicitInstantiationDeclaration`)
/// * explicit_instantiation_definition (`ExplicitInstantiationDefinition`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum TemplateSpecializationKind {
    Undeclared = 0i64,
    ImplicitInstantiation = 1i64,
    ExplicitSpecialization = 2i64,
    ExplicitInstantiationDeclaration = 3i64,
    ExplicitInstantiationDefinition = 4i64,
}
impl TryFrom<i128> for TemplateSpecializationKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Undeclared),
            1i64 => Ok(Self::ImplicitInstantiation),
            2i64 => Ok(Self::ExplicitSpecialization),
            3i64 => Ok(Self::ExplicitInstantiationDeclaration),
            4i64 => Ok(Self::ExplicitInstantiationDefinition),
            _ => {
                Err(ParseEnumError::new("TemplateSpecializationKind", value.to_string()))
            }
        }
    }
}
impl From<TemplateSpecializationKind> for i128 {
    fn from(value: TemplateSpecializationKind) -> i128 {
        match value {
            TemplateSpecializationKind::Undeclared => i128::from(0i64),
            TemplateSpecializationKind::ImplicitInstantiation => i128::from(1i64),
            TemplateSpecializationKind::ExplicitSpecialization => i128::from(2i64),
            TemplateSpecializationKind::ExplicitInstantiationDeclaration => {
                i128::from(3i64)
            }
            TemplateSpecializationKind::ExplicitInstantiationDefinition => {
                i128::from(4i64)
            }
        }
    }
}
impl std::str::FromStr for TemplateSpecializationKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undeclared" => Ok(Self::Undeclared),
            "implicit_instantiation" => Ok(Self::ImplicitInstantiation),
            "explicit_specialization" => Ok(Self::ExplicitSpecialization),
            "explicit_instantiation_declaration" => {
                Ok(Self::ExplicitInstantiationDeclaration)
            }
            "explicit_instantiation_definition" => {
                Ok(Self::ExplicitInstantiationDefinition)
            }
            _ => Err(ParseEnumError::new("TemplateSpecializationKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for TemplateSpecializationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Undeclared => "undeclared",
            Self::ImplicitInstantiation => "implicit_instantiation",
            Self::ExplicitSpecialization => "explicit_specialization",
            Self::ExplicitInstantiationDeclaration => {
                "explicit_instantiation_declaration"
            }
            Self::ExplicitInstantiationDefinition => "explicit_instantiation_definition",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for TemplateSpecializationKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new(
                        "TemplateSpecializationKind",
                        value.clone(),
                    ))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => {
                Err(
                    ParseEnumError::new(
                        "TemplateSpecializationKind",
                        format!("{other:?}"),
                    ),
                )
            }
        }
    }
}
/// Unwind Table Kind
/// Enum cases:
/// * none (`None`)
/// * sync (`Sync`)
/// * async (`Async`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum UnwindTableKind {
    None = 0i64,
    Sync = 1i64,
    Async = 2i64,
}
impl TryFrom<i128> for UnwindTableKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::None),
            1i64 => Ok(Self::Sync),
            2i64 => Ok(Self::Async),
            _ => Err(ParseEnumError::new("UnwindTableKind", value.to_string())),
        }
    }
}
impl From<UnwindTableKind> for i128 {
    fn from(value: UnwindTableKind) -> i128 {
        match value {
            UnwindTableKind::None => i128::from(0i64),
            UnwindTableKind::Sync => i128::from(1i64),
            UnwindTableKind::Async => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for UnwindTableKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "sync" => Ok(Self::Sync),
            "async" => Ok(Self::Async),
            _ => Err(ParseEnumError::new("UnwindTableKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for UnwindTableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::None => "none",
            Self::Sync => "sync",
            Self::Async => "async",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for UnwindTableKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("UnwindTableKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("UnwindTableKind", format!("{other:?}"))),
        }
    }
}
/// C/C++ visibility
/// Enum cases:
/// * default (`Default`)
/// * hidden (`Hidden`)
/// * protected (`Protected`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum VisibilityKind {
    Default = 0i64,
    Hidden = 1i64,
    Protected = 2i64,
}
impl TryFrom<i128> for VisibilityKind {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let value = value as i64;
        match value {
            0i64 => Ok(Self::Default),
            1i64 => Ok(Self::Hidden),
            2i64 => Ok(Self::Protected),
            _ => Err(ParseEnumError::new("VisibilityKind", value.to_string())),
        }
    }
}
impl From<VisibilityKind> for i128 {
    fn from(value: VisibilityKind) -> i128 {
        match value {
            VisibilityKind::Default => i128::from(0i64),
            VisibilityKind::Hidden => i128::from(1i64),
            VisibilityKind::Protected => i128::from(2i64),
        }
    }
}
impl std::str::FromStr for VisibilityKind {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "hidden" => Ok(Self::Hidden),
            "protected" => Ok(Self::Protected),
            _ => Err(ParseEnumError::new("VisibilityKind", s.to_string())),
        }
    }
}
impl std::fmt::Display for VisibilityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            Self::Default => "default",
            Self::Hidden => "hidden",
            Self::Protected => "protected",
        };
        write!(f, "{keyword}")
    }
}
impl TryFrom<&crate::attrs::Attribute> for VisibilityKind {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value as i128),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("VisibilityKind", value.clone()))
                    .and_then(Self::try_from)
            }
            crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
            other => Err(ParseEnumError::new("VisibilityKind", format!("{other:?}"))),
        }
    }
}
/// floating-point class test flags
/// Enum cases:
/// * fcNone (`None`)
/// * fcSNan (`SignalingNaN`)
/// * fcQNan (`QuietNaN`)
/// * fcNegInf (`NegativeInfinity`)
/// * fcNegNormal (`NegativeNormal`)
/// * fcNegSubnormal (`NegativeSubnormal`)
/// * fcNegZero (`NegativeZero`)
/// * fcPosZero (`PositiveZero`)
/// * fcPosSubnormal (`PositiveSubnormal`)
/// * fcPosNormal (`PositiveNormal`)
/// * fcPosInf (`PositiveInfinity`)
/// * fcNan (`Nan`)
/// * fcInf (`Infinity`)
/// * fcNormal (`Normal`)
/// * fcSubnormal (`Subnormal`)
/// * fcZero (`Zero`)
/// * fcPosFinite (`PositiveFinite`)
/// * fcNegFinite (`NegativeFinite`)
/// * fcFinite (`Finite`)
/// * fcPositive (`Positive`)
/// * fcNegative (`Negative`)
/// * fcAllFlags (`All`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FpClassTest(pub u64);
impl FpClassTest {
    pub const None: u64 = 0i64 as u64;
    pub const SignalingNaN: u64 = 1i64 as u64;
    pub const QuietNaN: u64 = 2i64 as u64;
    pub const NegativeInfinity: u64 = 4i64 as u64;
    pub const NegativeNormal: u64 = 8i64 as u64;
    pub const NegativeSubnormal: u64 = 16i64 as u64;
    pub const NegativeZero: u64 = 32i64 as u64;
    pub const PositiveZero: u64 = 64i64 as u64;
    pub const PositiveSubnormal: u64 = 128i64 as u64;
    pub const PositiveNormal: u64 = 256i64 as u64;
    pub const PositiveInfinity: u64 = 512i64 as u64;
    pub const Nan: u64 = 3i64 as u64;
    pub const Infinity: u64 = 516i64 as u64;
    pub const Normal: u64 = 264i64 as u64;
    pub const Subnormal: u64 = 144i64 as u64;
    pub const Zero: u64 = 96i64 as u64;
    pub const PositiveFinite: u64 = 448i64 as u64;
    pub const NegativeFinite: u64 = 56i64 as u64;
    pub const Finite: u64 = 504i64 as u64;
    pub const Positive: u64 = 960i64 as u64;
    pub const Negative: u64 = 60i64 as u64;
    pub const All: u64 = 1023i64 as u64;
}
impl TryFrom<&crate::attrs::Attribute> for FpClassTest {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::FpClassTest { value } => Ok(*value),
            crate::attrs::Attribute::Int { value, .. } => {
                u64::try_from(*value)
                    .map(Self)
                    .map_err(|_| ParseEnumError::new(
                        stringify!(FpClassTest),
                        value.to_string(),
                    ))
            }
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<u64>()
                    .map(Self)
                    .map_err(|_| ParseEnumError::new(
                        stringify!(FpClassTest),
                        value.clone(),
                    ))
            }
            other => {
                Err(ParseEnumError::new(stringify!(FpClassTest), format!("{other:?}")))
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i64)]
pub enum SideEffect {
    All = 0,
    Pure = 1,
    Const = 2,
}
impl TryFrom<i128> for SideEffect {
    type Error = ParseEnumError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::All),
            1 => Ok(Self::Pure),
            2 => Ok(Self::Const),
            _ => Err(ParseEnumError::new("SideEffect", value.to_string())),
        }
    }
}
impl TryFrom<&crate::attrs::Attribute> for SideEffect {
    type Error = ParseEnumError;
    fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
        match attr {
            crate::attrs::Attribute::Int { value, .. } => Self::try_from(*value),
            crate::attrs::Attribute::CirInt { value, .. } => {
                value
                    .parse::<i128>()
                    .map_err(|_| ParseEnumError::new("SideEffect", value.clone()))
                    .and_then(Self::try_from)
            }
            other => Err(ParseEnumError::new("SideEffect", format!("{other:?}"))),
        }
    }
}
impl std::fmt::Display for SideEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::All => "all",
            Self::Pure => "pure",
            Self::Const => "const",
        };
        write!(f, "{name}")
    }
}