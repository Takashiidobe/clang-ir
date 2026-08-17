use super::ty::Type;

/// An attribute as it appears in generic MLIR/CIR text.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attribute {
    /// Bare dict key with no `=`, e.g. `init` inside `<{alignment = 4, init}>`.
    Unit,
    Bool(bool),
    Int {
        value: i128,
        ty: Option<Type>,
    },
    /// Raw text preserved verbatim (float literals can exceed f64 precision,
    /// e.g. `!cir.f128`/`!cir.long_double` constants).
    Float {
        text: String,
        ty: Option<Type>,
    },
    Str(String),
    Array(Vec<Attribute>),
    /// Ordered `{ key = value, ... }`, including unit-sugar entries.
    Dict(Vec<(String, Attribute)>),
    SymbolRef(String),
    /// A bare type used directly as an attribute value (`TypeAttr`).
    Type(Type),
    /// Reference to a `#name` alias defined at the top of the file (e.g.
    /// clang IR hoists common `#cir.bool<true>`/`<false>` into `#true`/`#false`).
    Named(String),

    /// `#cir.int<N> : ty`. `text` is the verbatim decimal literal: CIR
    /// constants can be arbitrary-width `_BitInt`s, so the value doesn't
    /// always fit in an `i128`/`u128` (see [`Attribute::as_i128`]).
    CirInt {
        text: String,
        ty: Type,
    },
    /// `#cir.fp<literal> : ty`
    CirFloat {
        text: String,
        ty: Type,
    },
    /// `#cir.bool<true|false> : !cir.bool`
    CirBool {
        value: bool,
        ty: Type,
    },
    /// `#cir.const_array<"..."|[...], trailing_zeros?> : ty`
    ConstArray {
        data: ConstArrayData,
        trailing_zeros: bool,
        ty: Type,
    },
    /// `#cir.const_vector<[...]> : ty`
    ConstVector {
        elements: Vec<Attribute>,
        ty: Type,
    },
    /// `#cir.const_record<{...}> : ty`
    ConstRecord {
        elements: Vec<Attribute>,
        ty: Type,
    },
    /// `#cir.const_complex<real, imag> : ty`
    ConstComplex {
        real: Box<Attribute>,
        imag: Box<Attribute>,
        ty: Type,
    },
    /// `#cir.global_view<@symbol[, [idx : ty, ...]]> : ty`. `indices` is the
    /// optional GEP-like index chain used to take the address of a nested
    /// member/element within `symbol` rather than `symbol` itself.
    GlobalView {
        symbol: String,
        indices: Vec<i128>,
        ty: Type,
    },
    /// `#cir.bitfield_info<name = "...", storage_type = ty, size = N, offset
    /// = N, is_signed = bool>` (no trailing `: ty` - unlike most other `#cir.*`
    /// attrs, the CIR printer doesn't attach one to this one).
    BitfieldInfo {
        name: String,
        storage_type: Type,
        size: u32,
        offset: u32,
        is_signed: bool,
    },
    /// `#cir.zero : ty`
    Zero {
        ty: Type,
    },
    /// `#cir.poison : ty`
    Poison {
        ty: Type,
    },

    /// Any attribute we don't interpret structurally: unrecognized dialect,
    /// or a known CIR mnemonic whose body didn't match the expected shape.
    /// `raw` is the verbatim source text of the `<...>` body (excluding the
    /// angle brackets), or `None` if there was no body at all.
    Dialect {
        dialect: String,
        mnemonic: String,
        raw: Option<String>,
        ty: Option<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstArrayData {
    /// Exact byte contents (not necessarily valid UTF-8: `#cir.const_array`
    /// string bodies encode arbitrary byte data, e.g. non-printable padding
    /// or non-ASCII wide-char data).
    Str(Vec<u8>),
    Elements(Vec<Attribute>),
}

impl Attribute {
    pub fn as_dict(&self) -> Option<&[(String, Attribute)]> {
        match self {
            Attribute::Dict(d) => Some(d),
            _ => None,
        }
    }

    pub fn dict_get(&self, key: &str) -> Option<&Attribute> {
        self.as_dict()?
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn as_int(&self) -> Option<i128> {
        match self {
            Attribute::Int { value, .. } => Some(*value),
            Attribute::CirInt { text, .. } => text.parse().ok(),
            _ => None,
        }
    }

    /// Like [`Attribute::as_int`], but also covers values in `u128`'s extra
    /// range (CIR constants can be unsigned 128-bit, which doesn't fit
    /// `i128`). Returns `None` for `_BitInt` literals wider than 128 bits.
    pub fn as_u128(&self) -> Option<u128> {
        match self {
            Attribute::Int { value, .. } => u128::try_from(*value).ok(),
            Attribute::CirInt { text, .. } => text.parse().ok(),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Attribute::Bool(b) | Attribute::CirBool { value: b, .. } => Some(*b),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Attribute::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_symbol_ref(&self) -> Option<&str> {
        match self {
            Attribute::SymbolRef(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Attribute]> {
        match self {
            Attribute::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_type(&self) -> Option<&Type> {
        match self {
            Attribute::Type(t) => Some(t),
            _ => None,
        }
    }
}

impl From<bool> for Attribute {
    fn from(b: bool) -> Attribute {
        Attribute::Bool(b)
    }
}

impl From<i128> for Attribute {
    fn from(value: i128) -> Attribute {
        Attribute::Int { value, ty: None }
    }
}

impl From<String> for Attribute {
    fn from(s: String) -> Attribute {
        Attribute::Str(s)
    }
}

impl From<&str> for Attribute {
    fn from(s: &str) -> Attribute {
        Attribute::Str(s.to_string())
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Unit => write!(f, "unit"),
            Attribute::Bool(b) => write!(f, "{b}"),
            Attribute::Int { value, .. } => write!(f, "{value}"),
            Attribute::Float { text, .. } => write!(f, "{text}"),
            Attribute::Str(s) => write!(f, "{s:?}"),
            Attribute::Array(items) => write_bracketed(f, '[', ']', items),
            Attribute::Dict(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k} = {v}")?;
                }
                write!(f, "}}")
            }
            Attribute::SymbolRef(s) => write!(f, "@{s}"),
            Attribute::Type(t) => write!(f, "{t}"),
            Attribute::Named(n) => write!(f, "#{n}"),
            Attribute::CirInt { text, .. } => write!(f, "{text}"),
            Attribute::CirFloat { text, .. } => write!(f, "{text}"),
            Attribute::CirBool { value, .. } => write!(f, "{value}"),
            Attribute::ConstArray { data, .. } => match data {
                ConstArrayData::Str(bytes) => write!(f, "{:?}", String::from_utf8_lossy(bytes)),
                ConstArrayData::Elements(items) => write_bracketed(f, '[', ']', items),
            },
            Attribute::ConstVector { elements, .. } => write_bracketed(f, '[', ']', elements),
            Attribute::ConstRecord { elements, .. } => write_bracketed(f, '{', '}', elements),
            Attribute::ConstComplex { real, imag, .. } => write!(f, "({real}, {imag})"),
            Attribute::GlobalView {
                symbol, indices, ..
            } => {
                write!(f, "@{symbol}")?;
                if !indices.is_empty() {
                    write!(f, "[")?;
                    for (i, index) in indices.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{index}")?;
                    }
                    write!(f, "]")?;
                }
                Ok(())
            }
            Attribute::BitfieldInfo {
                name,
                storage_type,
                size,
                offset,
                is_signed,
            } => write!(
                f,
                "bitfield_info<{name:?}, {storage_type}, size={size}, offset={offset}, signed={is_signed}>"
            ),
            Attribute::Zero { .. } => write!(f, "zero"),
            Attribute::Poison { .. } => write!(f, "poison"),
            Attribute::Dialect {
                dialect,
                mnemonic,
                raw,
                ..
            } => {
                write!(f, "#{dialect}.{mnemonic}")?;
                match raw {
                    Some(raw) => write!(f, "<{raw}>"),
                    None => Ok(()),
                }
            }
        }
    }
}

fn write_bracketed(
    f: &mut std::fmt::Formatter<'_>,
    open: char,
    close: char,
    items: &[Attribute],
) -> std::fmt::Result {
    write!(f, "{open}")?;
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{item}")?;
    }
    write!(f, "{close}")
}
