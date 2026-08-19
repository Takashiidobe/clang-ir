use super::Parser;
use crate::ast::Type;
use crate::enums::RecordMemberKind;
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
            "float" => Ok(Type::Single),
            "double" => Ok(Type::Double),
            "f16" => Ok(Type::Fp16),
            "bf16" => Ok(Type::Bf16),
            "f80" => Ok(Type::Fp80),
            "f128" => Ok(Type::Fp128),
            "long_double" => self.parse_cir_long_double(),
            "ptr" => self.parse_cir_ptr(),
            "array" => self.parse_cir_array(),
            "vector" => self.parse_cir_vector(),
            "func" => self.parse_cir_func(),
            "struct" => self.parse_cir_struct(),
            "union" => self.parse_cir_union(),
            "complex" => self.parse_cir_complex(),
            "data_member" => self.parse_cir_member_type(true),
            "method" => self.parse_cir_member_type(false),
            "vptr" => Ok(Type::VPtr),
            "eh_token" => Ok(Type::EhToken),
            "cleanup_token" => Ok(Type::CleanupToken),
            "catch_token" => Ok(Type::CatchToken),
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
        Ok(Type::Int {
            width,
            is_signed: signed,
            is_bit_int: bit_precise,
        })
    }

    fn parse_cir_long_double(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let underlying = self.parse_type()?;
        self.expect(Tok::Greater)?;
        Ok(Type::LongDouble {
            underlying: Box::new(underlying),
        })
    }

    fn parse_cir_ptr(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let pointee = self.parse_type()?;
        let addr_space = if !self.at(&Tok::Greater) {
            Some(self.capture_raw_body())
        } else {
            None
        };
        if addr_space.is_none() {
            self.expect(Tok::Greater)?;
        }
        Ok(Type::Pointer {
            pointee: Box::new(pointee),
            addr_space,
        })
    }

    fn parse_cir_array(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let element_type = self.parse_type()?;
        self.expect_ident_matching("x")?;
        let size: u64 = self
            .expect_number()?
            .parse()
            .map_err(|_| self.err("invalid array size"))?;
        self.expect(Tok::Greater)?;
        Ok(Type::Array {
            element_type: Box::new(element_type),
            size,
        })
    }

    fn parse_cir_vector(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let scalable = self.eat(&Tok::LBracket);
        let size: u64 = self
            .expect_number()?
            .parse()
            .map_err(|_| self.err("invalid vector size"))?;
        if scalable {
            self.expect(Tok::RBracket)?;
        }
        self.expect_ident_matching("x")?;
        let element_type = self.parse_type()?;
        self.expect(Tok::Greater)?;
        Ok(Type::Vector {
            element_type: Box::new(element_type),
            size,
            is_scalable: scalable.then_some(true),
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
        Ok(Type::Func {
            inputs,
            optional_return_type: (output != Type::Void).then(|| Box::new(output)),
            var_arg: varargs,
        })
    }

    fn parse_cir_complex(&mut self) -> Result<Type> {
        self.expect(Tok::Less)?;
        let element_type = self.parse_type()?;
        self.expect(Tok::Greater)?;
        Ok(Type::Complex {
            element_type: Box::new(element_type),
        })
    }

    fn parse_cir_member_type(&mut self, data_member: bool) -> Result<Type> {
        self.expect(Tok::Less)?;
        let first = self.parse_type()?;
        self.expect_ident_matching("in")?;
        let second = self.parse_type()?;
        self.expect(Tok::Greater)?;
        if data_member {
            Ok(Type::DataMember {
                member_ty: Box::new(first),
                class_ty: Box::new(second),
            })
        } else {
            Ok(Type::Method {
                member_func_ty: Box::new(first),
                class_ty: Box::new(second),
            })
        }
    }

    fn parse_cir_struct(&mut self) -> Result<Type> {
        self.parse_cir_record(false)
    }

    fn parse_cir_union(&mut self) -> Result<Type> {
        self.parse_cir_record(true)
    }

    fn parse_cir_record(&mut self, union: bool) -> Result<Type> {
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
                _ => break,
            }
        }

        let mut members = Vec::new();
        let mut member_kinds = Vec::new();
        if self.eat(&Tok::LBrace) {
            if !self.at(&Tok::RBrace) {
                loop {
                    let kind = match self.expect_ident()?.as_str() {
                        "data" => RecordMemberKind::Data,
                        "pad" => RecordMemberKind::Pad,
                        "empty" => RecordMemberKind::Empty,
                        other => {
                            return Err(self.err(format!("unknown record member kind `{other}`")));
                        }
                    };
                    let ty = self.parse_type()?;
                    member_kinds.push(kind);
                    members.push(ty);
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

        let members = (!members.is_empty()).then_some(members);
        if union {
            Ok(Type::Union {
                members,
                name,
                incomplete,
                packed,
                padding: None,
                member_kinds,
            })
        } else {
            Ok(Type::Struct {
                members,
                name,
                incomplete,
                packed,
                member_kinds,
                is_class: false,
            })
        }
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
        return match width {
            16 => Some(Type::Fp16),
            32 => Some(Type::Single),
            64 => Some(Type::Double),
            80 => Some(Type::Fp80),
            128 => Some(Type::Fp128),
            _ => None,
        };
    }
    None
}
