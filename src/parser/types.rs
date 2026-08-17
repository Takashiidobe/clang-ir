use super::Parser;
use crate::ast::Type;
use crate::ast::ty::{FloatKind, RecordKind, RecordMemberKind, StructType};
use crate::error::Result;
use crate::lexer::Tok;

impl<'a> Parser<'a> {
    pub(super) fn parse_type(&mut self) -> Result<Type> {
        match self.tok().clone() {
            Tok::Bang => {
                self.bump();
                let full = self.expect_ident()?;
                match full.split_once('.') {
                    Some((dialect, mnemonic)) => self.parse_dialect_type(dialect, mnemonic),
                    // `!dialect<tag ...>` (no `.mnemonic`), symmetric with the
                    // analogous bare attribute form; see parser/attrs.rs.
                    None if self.at(&Tok::Less) => {
                        self.bump();
                        let tag = self.expect_ident()?;
                        let raw = self.capture_raw_body();
                        Ok(Type::Dialect {
                            dialect: full,
                            mnemonic: tag,
                            raw: Some(raw),
                        })
                    }
                    None => Ok(Type::Named(full)),
                }
            }
            Tok::LParen => {
                let inputs = self.parse_type_list_parens()?;
                self.expect(Tok::Arrow)?;
                let results = if self.at(&Tok::LParen) {
                    self.parse_type_list_parens()?
                } else {
                    vec![self.parse_type()?]
                };
                Ok(Type::FunctionType { inputs, results })
            }
            Tok::Ident(id) => {
                self.bump();
                parse_builtin_ident_type(&id)
                    .ok_or_else(|| self.err(format!("unknown builtin type `{id}`")))
            }
            other => Err(self.err(format!("expected type, found {other:?}"))),
        }
    }

    fn parse_dialect_type(&mut self, dialect: &str, mnemonic: &str) -> Result<Type> {
        if dialect != "cir" {
            let raw = self.maybe_capture_body();
            return Ok(Type::Dialect {
                dialect: dialect.to_string(),
                mnemonic: mnemonic.to_string(),
                raw,
            });
        }
        match mnemonic {
            "int" => self.parse_cir_int(),
            "bool" => Ok(Type::Bool),
            "void" => Ok(Type::Void),
            "float" => Ok(Type::Float(FloatKind::F32)),
            "double" => Ok(Type::Float(FloatKind::F64)),
            "f16" => Ok(Type::Float(FloatKind::F16)),
            "f80" => Ok(Type::Float(FloatKind::F80)),
            "f128" => Ok(Type::Float(FloatKind::F128)),
            "long_double" => {
                self.expect(Tok::Less)?;
                let inner = self.parse_type()?;
                self.expect(Tok::Greater)?;
                Ok(Type::LongDouble(Box::new(inner)))
            }
            "ptr" => self.parse_cir_ptr(),
            "array" => self.parse_cir_array(),
            "vector" => self.parse_cir_vector(),
            "func" => self.parse_cir_func(),
            "struct" => self.parse_cir_record(RecordKind::Struct),
            "union" => self.parse_cir_record(RecordKind::Union),
            "complex" => {
                self.expect(Tok::Less)?;
                let inner = self.parse_type()?;
                self.expect(Tok::Greater)?;
                Ok(Type::Complex(Box::new(inner)))
            }
            _ => {
                let raw = self.maybe_capture_body();
                Ok(Type::Dialect {
                    dialect: "cir".to_string(),
                    mnemonic: mnemonic.to_string(),
                    raw,
                })
            }
        }
    }

    /// If the next token opens a `<...>` body, consumes it and returns the
    /// raw text; otherwise there was no body at all.
    fn maybe_capture_body(&mut self) -> Option<String> {
        if self.eat(&Tok::Less) {
            Some(self.capture_raw_body())
        } else {
            None
        }
    }

    fn parse_cir_int(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let signed = match self.expect_ident()?.as_str() {
            "s" => true,
            "u" => false,
            other => return Err(self.err(format!("expected `s` or `u`, found `{other}`"))),
        };
        self.expect(Tok::Comma)?;
        let width: u32 = self
            .expect_number()?
            .parse()
            .map_err(|_| self.err("invalid integer width"))?;
        let bit_precise = if self.eat(&Tok::Comma) {
            self.expect_ident_matching("bitint")?;
            true
        } else {
            false
        };
        self.expect(Tok::Greater)?;
        Ok(Type::CirInt {
            signed,
            width,
            bit_precise,
        })
    }

    fn parse_cir_ptr(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let pointee = self.parse_type()?;
        // Tolerate a trailing address-space/etc payload we don't model yet.
        if !self.at(&Tok::Greater) {
            let _ = self.capture_raw_body();
            return Ok(Type::Ptr(Box::new(pointee)));
        }
        self.expect(Tok::Greater)?;
        Ok(Type::Ptr(Box::new(pointee)))
    }

    fn parse_cir_array(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let element = self.parse_type()?;
        self.expect_ident_matching("x")?;
        let size: u64 = self
            .expect_number()?
            .parse()
            .map_err(|_| self.err("invalid array size"))?;
        self.expect(Tok::Greater)?;
        Ok(Type::Array {
            element: Box::new(element),
            size,
        })
    }

    fn parse_cir_vector(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let size: u64 = self
            .expect_number()?
            .parse()
            .map_err(|_| self.err("invalid vector size"))?;
        self.expect_ident_matching("x")?;
        let element = self.parse_type()?;
        self.expect(Tok::Greater)?;
        Ok(Type::Vector {
            element: Box::new(element),
            size,
        })
    }

    fn parse_cir_func(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        self.expect(Tok::LParen)?;
        let mut inputs = Vec::new();
        let mut varargs = false;
        if !self.at(&Tok::RParen) {
            loop {
                if self.eat(&Tok::Ellipsis) {
                    varargs = true;
                    break;
                }
                inputs.push(self.parse_type()?);
                if !self.eat(&Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(Tok::RParen)?;
        let output = if self.eat(&Tok::Arrow) {
            self.parse_type()?
        } else {
            Type::Void
        };
        self.expect(Tok::Greater)?;
        Ok(Type::CirFunc {
            inputs,
            output: Box::new(output),
            varargs,
        })
    }

    fn parse_cir_record(&mut self, kind: RecordKind) -> Result<Type> {
        self.expect(Tok::Less)?;
        let name = match self.tok().clone() {
            Tok::Str(s) => {
                self.bump();
                Some(s)
            }
            _ => None,
        };

        let mut incomplete = false;
        let mut packed = false;
        let mut padded = false;
        loop {
            match self.tok().clone() {
                Tok::Ident(ref id) if id == "incomplete" => {
                    self.bump();
                    incomplete = true;
                }
                Tok::Ident(ref id) if id == "packed" => {
                    self.bump();
                    packed = true;
                }
                Tok::Ident(ref id) if id == "padded" => {
                    self.bump();
                    padded = true;
                }
                _ => break,
            }
        }

        let mut members = Vec::new();
        if self.eat(&Tok::LBrace) {
            if !self.at(&Tok::RBrace) {
                loop {
                    let member_kind = match self.expect_ident()?.as_str() {
                        "data" => RecordMemberKind::Data,
                        "pad" => RecordMemberKind::Pad,
                        "empty" => RecordMemberKind::Empty,
                        other => {
                            return Err(self.err(format!("unknown record member kind `{other}`")));
                        }
                    };
                    let ty = self.parse_type()?;
                    members.push((member_kind, ty));
                    if !self.eat(&Tok::Comma) {
                        break;
                    }
                }
            }
            self.expect(Tok::RBrace)?;
        }

        let trailing = if !self.at(&Tok::Greater) {
            Some(self.capture_raw_body())
        } else {
            None
        };
        if trailing.is_none() {
            self.expect(Tok::Greater)?;
        }

        Ok(Type::Struct(StructType {
            name,
            kind,
            incomplete,
            packed,
            padded,
            members,
            trailing,
        }))
    }

    fn expect_ident_matching(&mut self, want: &str) -> Result<()> {
        let id = self.expect_ident()?;
        if id == want {
            Ok(())
        } else {
            Err(self.err(format!("expected `{want}`, found `{id}`")))
        }
    }
}

fn parse_builtin_ident_type(id: &str) -> Option<Type> {
    if id == "index" {
        return Some(Type::Index);
    }
    if let Some(rest) = id.strip_prefix('i')
        && !rest.is_empty()
        && rest.bytes().all(|b| b.is_ascii_digit())
    {
        return rest.parse().ok().map(Type::Integer);
    }
    if let Some(rest) = id.strip_prefix('f')
        && !rest.is_empty()
        && rest.bytes().all(|b| b.is_ascii_digit())
    {
        let width: u32 = rest.parse().ok()?;
        let kind = match width {
            16 => FloatKind::F16,
            32 => FloatKind::F32,
            64 => FloatKind::F64,
            80 => FloatKind::F80,
            128 => FloatKind::F128,
            _ => return None,
        };
        return Some(Type::Float(kind));
    }
    None
}
