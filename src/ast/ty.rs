/// A type as it appears in generic MLIR/CIR text.
///
/// `Named` is deliberately not eagerly resolved: CIR record types can be
/// self- or mutually-recursive (`!rec_node = !cir.struct<"node" {data !cir.ptr<!rec_node>}>`),
/// so alias resolution has to stay lazy (via [`crate::ast::op::Module::resolve_type`])
/// rather than inlining bodies at parse time.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// Reference to a `!name` alias defined at the top of the file.
    Named(String),
    /// Builtin `iN` (signless integer of width N).
    Integer(u32),
    /// Builtin `index`.
    Index,
    /// Builtin function type `(ins) -> outs`, distinct from `!cir.func`.
    FunctionType {
        inputs: Vec<Type>,
        results: Vec<Type>,
    },
    /// `!cir.int<s|u, N>` (optionally `, bitint` for a C23 bit-precise `_BitInt(N)`).
    CirInt {
        signed: bool,
        width: u32,
        bit_precise: bool,
    },
    /// `!cir.bool`
    Bool,
    /// `!cir.void`
    Void,
    /// `!cir.float` / `!cir.double` / `!cir.f16` / `!cir.f80` / `!cir.f128`
    Float(FloatKind),
    /// `!cir.long_double<underlying>`
    LongDouble(Box<Type>),
    /// `!cir.ptr<T>`
    Ptr(Box<Type>),
    /// `!cir.array<T x N>`
    Array { element: Box<Type>, size: u64 },
    /// `!cir.vector<T x N>`
    Vector { element: Box<Type>, size: u64 },
    /// `!cir.func<(ins) -> out>` or `!cir.func<(ins, ...) -> out>`
    CirFunc {
        inputs: Vec<Type>,
        output: Box<Type>,
        varargs: bool,
    },
    /// `!cir.struct<...>` / `!cir.union<...>`
    Struct(StructType),
    /// `!cir.complex<T>`
    Complex(Box<Type>),
    /// Any type we don't interpret structurally (unknown dialect, or a known
    /// CIR mnemonic whose body didn't match the expected shape). `raw` is
    /// the verbatim source text of the `<...>` body (excluding the angle
    /// brackets), or `None` if there was no body at all.
    Dialect {
        dialect: String,
        mnemonic: String,
        raw: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FloatKind {
    F16,
    F32,
    F64,
    F80,
    F128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RecordKind {
    Struct,
    Union,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RecordMemberKind {
    Data,
    Pad,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructType {
    pub name: Option<String>,
    pub kind: RecordKind,
    pub incomplete: bool,
    pub packed: bool,
    pub members: Vec<(RecordMemberKind, Type)>,
    /// Anything after the member list we didn't specifically parse (e.g.
    /// AST-decl attributes), kept as raw source text.
    pub trailing: Option<String>,
}

impl Type {
    /// A signed `!cir.int<s, N>`, for hand-building IR (parsed code almost
    /// always sees these as a `Named` alias like `s32i` instead, since
    /// that's how CIR's printer emits them — see [`Type::Named`]).
    pub fn signed(width: u32) -> Type {
        Type::CirInt {
            signed: true,
            width,
            bit_precise: false,
        }
    }

    /// An unsigned `!cir.int<u, N>`.
    pub fn unsigned(width: u32) -> Type {
        Type::CirInt {
            signed: false,
            width,
            bit_precise: false,
        }
    }

    pub fn i8() -> Type {
        Type::signed(8)
    }

    pub fn i16() -> Type {
        Type::signed(16)
    }

    pub fn i32() -> Type {
        Type::signed(32)
    }

    pub fn i64() -> Type {
        Type::signed(64)
    }

    pub fn u8() -> Type {
        Type::unsigned(8)
    }

    pub fn u16() -> Type {
        Type::unsigned(16)
    }

    pub fn u32() -> Type {
        Type::unsigned(32)
    }

    pub fn u64() -> Type {
        Type::unsigned(64)
    }

    pub fn bool_() -> Type {
        Type::Bool
    }

    pub fn void() -> Type {
        Type::Void
    }

    pub fn float() -> Type {
        Type::Float(FloatKind::F32)
    }

    pub fn double() -> Type {
        Type::Float(FloatKind::F64)
    }

    pub fn ptr(self) -> Type {
        Type::Ptr(Box::new(self))
    }

    pub fn array(self, size: u64) -> Type {
        Type::Array {
            element: Box::new(self),
            size,
        }
    }

    pub fn vector(self, size: u64) -> Type {
        Type::Vector {
            element: Box::new(self),
            size,
        }
    }

    pub fn complex(self) -> Type {
        Type::Complex(Box::new(self))
    }

    /// A named reference, for hand-building IR that reuses an alias (e.g.
    /// `Type::named("s32i")` instead of the fully expanded `Type::i32()`).
    pub fn named(name: impl Into<String>) -> Type {
        Type::Named(name.into())
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // CIR always defines an alias for scalar int/float types and
            // refers to them by name everywhere, so this is what shows up
            // in practice (e.g. `s32i`), not the `CirInt`/`Float` variants.
            Type::Named(name) => write!(f, "{name}"),
            Type::Integer(w) => write!(f, "i{w}"),
            Type::Index => write!(f, "index"),
            Type::FunctionType { inputs, results } => {
                write!(f, "(")?;
                write_comma_list(f, inputs)?;
                write!(f, ") -> ")?;
                match results.as_slice() {
                    [one] => write!(f, "{one}"),
                    many => {
                        write!(f, "(")?;
                        write_comma_list(f, many)?;
                        write!(f, ")")
                    }
                }
            }
            Type::CirInt {
                signed,
                width,
                bit_precise,
            } => {
                write!(f, "{}{width}", if *signed { "s" } else { "u" })?;
                if *bit_precise {
                    write!(f, "_bitint")?;
                }
                Ok(())
            }
            Type::Bool => write!(f, "bool"),
            Type::Void => write!(f, "void"),
            Type::Float(kind) => write!(f, "{kind}"),
            Type::LongDouble(inner) => write!(f, "long_double<{inner}>"),
            Type::Ptr(inner) => match &**inner {
                // Parenthesize so the trailing `*` clearly binds to the
                // whole pointee rather than reading as part of its tail
                // (e.g. a function pointer's return type).
                Type::CirFunc { .. } | Type::FunctionType { .. } => write!(f, "({inner})*"),
                _ => write!(f, "{inner}*"),
            },
            Type::Array { element, size } => write!(f, "{element}[{size}]"),
            Type::Vector { element, size } => write!(f, "vector<{size} x {element}>"),
            Type::CirFunc {
                inputs,
                output,
                varargs,
            } => {
                write!(f, "(")?;
                write_comma_list(f, inputs)?;
                if *varargs {
                    if !inputs.is_empty() {
                        write!(f, ", ")?;
                    }
                    write!(f, "...")?;
                }
                write!(f, ") -> {output}")
            }
            Type::Struct(s) => {
                write!(
                    f,
                    "{} ",
                    if s.kind == RecordKind::Union {
                        "union"
                    } else {
                        "struct"
                    }
                )?;
                match &s.name {
                    Some(name) => write!(f, "{name}")?,
                    None => write!(f, "<anon>")?,
                }
                if s.incomplete {
                    return write!(f, " {{ incomplete }}");
                }
                write!(f, " {{ ")?;
                for (i, (_, ty)) in s.members.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{ty}")?;
                }
                write!(f, " }}")
            }
            Type::Complex(inner) => write!(f, "complex<{inner}>"),
            Type::Dialect {
                dialect,
                mnemonic,
                raw,
            } => {
                write!(f, "{dialect}.{mnemonic}")?;
                match raw {
                    Some(raw) => write!(f, "<{raw}>"),
                    None => Ok(()),
                }
            }
        }
    }
}

impl std::str::FromStr for FloatKind {
    type Err = crate::model::enums::ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "f16" => Ok(FloatKind::F16),
            "float" => Ok(FloatKind::F32),
            "double" => Ok(FloatKind::F64),
            "f80" => Ok(FloatKind::F80),
            "f128" => Ok(FloatKind::F128),
            other => Err(crate::model::enums::ParseEnumError::new("FloatKind", other)),
        }
    }
}

impl std::fmt::Display for FloatKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self {
            FloatKind::F16 => "f16",
            FloatKind::F32 => "float",
            FloatKind::F64 => "double",
            FloatKind::F80 => "f80",
            FloatKind::F128 => "f128",
        };
        write!(f, "{kw}")
    }
}

fn write_comma_list(f: &mut std::fmt::Formatter<'_>, items: &[Type]) -> std::fmt::Result {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{item}")?;
    }
    Ok(())
}
