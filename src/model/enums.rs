//! Typed decodings of CIR's TableGen-defined enum attributes.
//!
//! MLIR's generic op printer serializes a plain (non-wrapped) enum attribute
//! as a raw `N : i32`, so without this table `kind = 11 : i32` on a
//! `cir.cast` is meaningless. Values and gaps below are taken verbatim from
//! `clang/include/clang/CIR/Dialect/IR/{CIROps,CIRAttrs}.td` in the
//! ClangIR/LLVM tree; several enums have non-contiguous discriminants (e.g.
//! `CastKind` mirrors Clang's `OperationKinds.def` ordinals) so they can't be
//! derived from declaration order alone.
//!
//! A handful of enums (`InlineKind`, `TLSModel`, ...) are wrapped in a real
//! dialect `Attribute` subclass with its own custom printer instead, so they
//! always show up as a bare keyword (`#cir.inline_kind<no_inline>`) even
//! under the generic op printer, never as an integer. Those are decoded from
//! a `&str` (via `FromStr`) instead of an int.
//!
//! Every enum here implements the standard conversion traits rather than
//! bespoke methods: `Display`/`FromStr` for the keyword form, and for the
//! integer-encoded ones, `From<Enum> for i128`/`TryFrom<i128>` for the raw
//! form and `TryFrom<&Attribute>` to decode straight from a parsed op
//! attribute (e.g. `CastKind::try_from(op.attr("kind")?)`).

use std::fmt;
use std::str::FromStr;

use crate::ast::Attribute;

/// Error returned by a CIR enum's `FromStr`/`TryFrom` impls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnumError {
    type_name: &'static str,
    input: String,
}

impl ParseEnumError {
    pub fn new(type_name: &'static str, input: impl Into<String>) -> Self {
        ParseEnumError {
            type_name,
            input: input.into(),
        }
    }
}

impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid {}", self.input, self.type_name)
    }
}

impl std::error::Error for ParseEnumError {}

macro_rules! int_enum {
    (
        $(#[$meta:meta])*
        $name:ident from $td:literal {
            $($(#[$vmeta:meta])* $variant:ident = $value:literal => $keyword:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[doc = concat!("See `", $td, "` in the ClangIR TableGen sources.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name {
            $($(#[$vmeta])* $variant),+
        }

        impl TryFrom<i128> for $name {
            type Error = ParseEnumError;
            fn try_from(value: i128) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant)),+,
                    _ => Err(ParseEnumError { type_name: stringify!($name), input: value.to_string() }),
                }
            }
        }

        impl From<$name> for i128 {
            fn from(v: $name) -> i128 {
                match v {
                    $(<$name>::$variant => $value),+
                }
            }
        }

        impl FromStr for $name {
            type Err = ParseEnumError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($keyword => Ok(Self::$variant)),+,
                    _ => Err(ParseEnumError { type_name: stringify!($name), input: s.to_string() }),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let kw = match self {
                    $(Self::$variant => $keyword),+
                };
                write!(f, "{kw}")
            }
        }

        /// Decodes from an op/attribute's plain integer-valued enum field,
        /// e.g. `CastKind::try_from(op.attr("kind")?)` on a `cir.cast`.
        impl TryFrom<&Attribute> for $name {
            type Error = ParseEnumError;
            fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
                let value = attr.as_int().ok_or_else(|| ParseEnumError {
                    type_name: stringify!($name),
                    input: format!("{attr:?}"),
                })?;
                Self::try_from(value)
            }
        }
    };
}

macro_rules! keyword_enum {
    (
        $(#[$meta:meta])*
        $name:ident from $td:literal {
            $($variant:ident => $keyword:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[doc = concat!("See `", $td, "` in the ClangIR TableGen sources.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name {
            $($variant),+
        }

        impl FromStr for $name {
            type Err = ParseEnumError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($keyword => Ok(Self::$variant)),+,
                    _ => Err(ParseEnumError { type_name: stringify!($name), input: s.to_string() }),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let kw = match self {
                    $(Self::$variant => $keyword),+
                };
                write!(f, "{kw}")
            }
        }

        /// Decodes a CIR dialect attribute whose body is just this enum's
        /// bare keyword (e.g. `#cir.inline_kind<no_inline>`, represented
        /// generically as `Attribute::Dialect { raw: Some("no_inline"), .. }`).
        impl TryFrom<&Attribute> for $name {
            type Error = ParseEnumError;
            fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
                match attr {
                    Attribute::Dialect { raw: Some(raw), .. } => raw.trim().parse(),
                    other => Err(ParseEnumError { type_name: stringify!($name), input: format!("{other:?}") }),
                }
            }
        }
    };
}

int_enum! {
    /// `cir.cast`'s `kind` operand. Discriminants mirror Clang's
    /// `OperationKinds.def` ordinals, hence the large gaps.
    CastKind from "CIROps.td:172-245" {
        Bitcast = 1 => "bitcast",
        ArrayToPtrDecay = 11 => "array_to_ptrdecay",
        MemberPtrToBool = 17 => "member_ptr_to_bool",
        IntToPtr = 21 => "int_to_ptr",
        PtrToInt = 22 => "ptr_to_int",
        PtrToBool = 23 => "ptr_to_bool",
        Integral = 27 => "integral",
        IntToBool = 28 => "int_to_bool",
        IntToFloat = 29 => "int_to_float",
        FloatToInt = 36 => "float_to_int",
        FloatToBool = 37 => "float_to_bool",
        BoolToInt = 38 => "bool_to_int",
        Floating = 39 => "floating",
        FloatToComplex = 44 => "float_to_complex",
        FloatComplexToReal = 45 => "float_complex_to_real",
        FloatComplexToBool = 46 => "float_complex_to_bool",
        FloatComplex = 47 => "float_complex",
        FloatComplexToIntComplex = 48 => "float_complex_to_int_complex",
        IntToComplex = 49 => "int_to_complex",
        IntComplexToReal = 50 => "int_complex_to_real",
        IntComplexToBool = 51 => "int_complex_to_bool",
        IntComplex = 52 => "int_complex",
        IntComplexToFloatComplex = 53 => "int_complex_to_float_complex",
        AddressSpace = 63 => "address_space",
        BoolToFloat = 1000 => "bool_to_float",
    }
}

int_enum! {
    /// `cir.cmp`'s `kind` operand.
    CmpOpKind from "CIROps.td:2504-2513" {
        Lt = 0 => "lt",
        Le = 1 => "le",
        Gt = 2 => "gt",
        Ge = 3 => "ge",
        Eq = 4 => "eq",
        Ne = 5 => "ne",
        /// Ordered not-equal (float only).
        One = 6 => "one",
        /// Unordered (float only).
        Uno = 7 => "uno",
    }
}

int_enum! {
    /// `cir.global`/`cir.func`'s `linkage` field.
    GlobalLinkageKind from "CIROps.td:3272-3298" {
        External = 0 => "external",
        AvailableExternally = 1 => "available_externally",
        LinkOnceAny = 2 => "linkonce",
        LinkOnceOdr = 3 => "linkonce_odr",
        WeakAny = 4 => "weak",
        WeakOdr = 5 => "weak_odr",
        Appending = 6 => "appending",
        Internal = 7 => "internal",
        Private = 8 => "cir_private",
        ExternalWeak = 9 => "extern_weak",
        Common = 10 => "common",
    }
}

int_enum! {
    /// `global_visibility` field. Normally printed as a bare keyword via a
    /// custom MLIR `Property` (not a plain attribute), so it typically won't
    /// appear as `N : i32` even under the generic op printer — this exists
    /// for completeness/robustness in case a caller decodes it manually.
    VisibilityKind from "CIRAttrs.td:1291-1296" {
        Default = 0 => "default",
        Hidden = 1 => "hidden",
        Protected = 2 => "protected",
    }
}

int_enum! {
    /// `cir.func`'s `calling_conv` field. CIR-specific numbering, not kept
    /// in sync with `llvm::CallingConv`/`clang::CallingConv`.
    CallingConv from "CIROps.td:4140-4146" {
        C = 0 => "c",
        PtxKernel = 1 => "ptx_kernel",
        SpirFunction = 2 => "spir_function",
        SpirKernel = 3 => "spir_kernel",
        AmdGpuKernel = 4 => "amdgpu_kernel",
    }
}

int_enum! {
    /// `side_effect` field on call-like ops / `cir.func`.
    SideEffect from "CIRAttrs.td:1839-1844" {
        All = 0 => "all",
        Pure = 1 => "pure",
        Const = 2 => "const",
    }
}

int_enum! {
    /// `cir.case`'s `kind` field.
    CaseOpKind from "CIROps.td:1515-1520" {
        Default = 0 => "default",
        Equal = 1 => "equal",
        Anyof = 2 => "anyof",
        Range = 3 => "range",
    }
}

int_enum! {
    /// Atomic ops' `mem_order`/`succ_order`/`fail_order` fields.
    MemOrder from "CIROps.td:628-635" {
        Relaxed = 0 => "relaxed",
        Consume = 1 => "consume",
        Acquire = 2 => "acquire",
        Release = 3 => "release",
        AcquireRelease = 4 => "acq_rel",
        SequentiallyConsistent = 5 => "seq_cst",
    }
}

int_enum! {
    /// Atomic ops' `sync_scope` field.
    SyncScopeKind from "CIROps.td:642-662" {
        SingleThread = 0 => "single_thread",
        System = 1 => "system",
        Device = 2 => "device",
        Workgroup = 3 => "workgroup",
        Wavefront = 4 => "wavefront",
        Cluster = 5 => "cluster",
        HipSingleThread = 6 => "hip_single_thread",
        HipSystem = 7 => "hip_system",
        HipAgent = 8 => "hip_agent",
        HipWorkgroup = 9 => "hip_workgroup",
        HipWavefront = 10 => "hip_wavefront",
        HipCluster = 11 => "hip_cluster",
        OpenClWorkGroup = 12 => "opencl_work_group",
        OpenClDevice = 13 => "opencl_device",
        OpenClAllSvmDevices = 14 => "opencl_all_svm_devices",
        OpenClSubGroup = 15 => "opencl_sub_group",
    }
}

keyword_enum! {
    /// `inline_kind` field, printed via a custom attribute
    /// (`#cir.inline_kind<no_inline>`), never as a raw integer.
    InlineKind from "CIRAttrs.td:1723-1729" {
        NoInline => "no_inline",
        AlwaysInline => "always_inline",
        InlineHint => "inline_hint",
    }
}

keyword_enum! {
    /// `cir.default_tls_model` / a global's TLS model, printed via a custom
    /// attribute (`#cir.tls_model<tls_dyn>`), never as a raw integer.
    TlsModel from "CIROps.td:3304-3308" {
        GeneralDynamic => "tls_dyn",
        LocalDynamic => "tls_local_dyn",
        InitialExec => "tls_init_exec",
        LocalExec => "tls_local_exec",
    }
}

keyword_enum! {
    /// `cir.lang` module attribute, printed via a custom attribute
    /// (`#cir.lang<c>`), never as a raw integer.
    SourceLanguage from "CIRAttrs.td:80-82" {
        C => "c",
        Cxx => "cxx",
    }
}

/// `cir.is_fp_class`'s `flags` field: a bitmask of individual FP class bits
/// (composite groups like `fcNan` are just multiple bits OR'd together, not
/// separate encodings). Bit positions mirror LLVM's `FPClassTest` layout, per
/// `FPClassTestEnum` in `CIROps.td:6841-6889`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FpClassFlags(pub u32);

impl FpClassFlags {
    pub const SIGNALING_NAN: u32 = 1 << 0;
    pub const QUIET_NAN: u32 = 1 << 1;
    pub const NEGATIVE_INFINITY: u32 = 1 << 2;
    pub const NEGATIVE_NORMAL: u32 = 1 << 3;
    pub const NEGATIVE_SUBNORMAL: u32 = 1 << 4;
    pub const NEGATIVE_ZERO: u32 = 1 << 5;
    pub const POSITIVE_ZERO: u32 = 1 << 6;
    pub const POSITIVE_SUBNORMAL: u32 = 1 << 7;
    pub const POSITIVE_NORMAL: u32 = 1 << 8;
    pub const POSITIVE_INFINITY: u32 = 1 << 9;

    const NAMED_BITS: [(u32, &'static str); 10] = [
        (Self::SIGNALING_NAN, "fcSNan"),
        (Self::QUIET_NAN, "fcQNan"),
        (Self::NEGATIVE_INFINITY, "fcNegInf"),
        (Self::NEGATIVE_NORMAL, "fcNegNormal"),
        (Self::NEGATIVE_SUBNORMAL, "fcNegSubnormal"),
        (Self::NEGATIVE_ZERO, "fcNegZero"),
        (Self::POSITIVE_ZERO, "fcPosZero"),
        (Self::POSITIVE_SUBNORMAL, "fcPosSubnormal"),
        (Self::POSITIVE_NORMAL, "fcPosNormal"),
        (Self::POSITIVE_INFINITY, "fcPosInf"),
    ];

    pub fn contains(self, bit: u32) -> bool {
        self.0 & bit != 0
    }
}

impl From<u32> for FpClassFlags {
    fn from(value: u32) -> Self {
        FpClassFlags(value)
    }
}

/// Decodes from `cir.is_fp_class`'s plain integer-valued `flags` field.
impl TryFrom<&Attribute> for FpClassFlags {
    type Error = ParseEnumError;
    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        let value = attr.as_int().ok_or_else(|| ParseEnumError {
            type_name: "FpClassFlags",
            input: format!("{attr:?}"),
        })?;
        Ok(FpClassFlags(value as u32))
    }
}

impl fmt::Display for FpClassFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            return write!(f, "fcNone");
        }
        let names: Vec<&str> = Self::NAMED_BITS
            .iter()
            .filter(|(bit, _)| self.0 & bit != 0)
            .map(|(_, name)| *name)
            .collect();
        write!(f, "{}", names.join("|"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_kind_matches_sample_output() {
        // `cir.cast ... <{kind = 11 : i32}>` from `add.c`'s array-decay cast.
        assert_eq!(CastKind::try_from(11), Ok(CastKind::ArrayToPtrDecay));
        assert_eq!(CastKind::try_from(28), Ok(CastKind::IntToBool));
        assert!(CastKind::try_from(9999).is_err());
    }

    #[test]
    fn linkage_matches_sample_output() {
        assert_eq!(
            GlobalLinkageKind::try_from(8),
            Ok(GlobalLinkageKind::Private)
        );
        assert_eq!(
            GlobalLinkageKind::try_from(7),
            Ok(GlobalLinkageKind::Internal)
        );
    }

    #[test]
    fn cmp_order_is_not_eq_ne_lt_gt_le_ge() {
        assert_eq!(CmpOpKind::try_from(0), Ok(CmpOpKind::Lt));
        assert_eq!(CmpOpKind::try_from(4), Ok(CmpOpKind::Eq));
    }

    #[test]
    fn inline_kind_decodes_from_keyword_attr() {
        let attr = Attribute::Dialect {
            dialect: "cir".into(),
            mnemonic: "inline_kind".into(),
            raw: Some("no_inline".into()),
            ty: None,
        };
        assert_eq!(InlineKind::try_from(&attr), Ok(InlineKind::NoInline));
    }

    #[test]
    fn round_trips_keyword() {
        for v in [
            CastKind::Bitcast,
            CastKind::BoolToFloat,
            CastKind::AddressSpace,
        ] {
            assert_eq!(CastKind::try_from(i128::from(v)), Ok(v));
        }
    }

    #[test]
    fn from_str_round_trips_display() {
        assert_eq!("array_to_ptrdecay".parse(), Ok(CastKind::ArrayToPtrDecay));
        assert_eq!(CastKind::ArrayToPtrDecay.to_string(), "array_to_ptrdecay");
        assert_eq!("no_inline".parse(), Ok(InlineKind::NoInline));
        assert_eq!(InlineKind::NoInline.to_string(), "no_inline");
    }

    #[test]
    fn fp_class_flags_decodes_composite_group() {
        // `fcNan` is `fcSNan | fcQNan` (bits 0 and 1).
        let attr = Attribute::Int { value: 3, ty: None };
        let flags = FpClassFlags::try_from(&attr).unwrap();
        assert_eq!(flags.0, 3);
        assert_eq!(flags.to_string(), "fcSNan|fcQNan");
    }

    #[test]
    fn fp_class_flags_none_displays_as_fc_none() {
        assert_eq!(FpClassFlags(0).to_string(), "fcNone");
    }
}
