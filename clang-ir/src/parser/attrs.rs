use super::Parser;
use crate::ast::Attribute;
use crate::error::Result;
use crate::lexer::Tok;

impl<'a> Parser<'a> {
    pub(super) fn parse_attribute(&mut self) -> Result<Attribute> {
        match self.tok().clone() {
            Tok::Hash => {
                self.bump();
                let full = self.expect_ident()?;
                match full.split_once('.') {
                    Some((dialect, mnemonic)) => self.parse_dialect_attr(dialect, mnemonic),
                    // `#dialect<tag ...>` (no `.mnemonic`): some dialect attrs
                    // (e.g. enum attrs with no registered print/parse
                    // mnemonic) fall back to this bare form, where the first
                    // token inside `<>` self-identifies the attribute kind.
                    // Only an alias use (`#name`) when NOT followed by `<`.
                    None if self.at(&Tok::Less) => {
                        self.bump();
                        let tag = self.expect_ident()?;
                        let raw = self.capture_raw_body();
                        let ty = self.maybe_colon_type()?;
                        Ok(Attribute::Dialect {
                            dialect: full,
                            mnemonic: tag,
                            raw: Some(raw),
                            ty,
                        })
                    }
                    None => Ok(Attribute::Named(full)),
                }
            }
            Tok::Str(s) => {
                self.bump();
                Ok(Attribute::Str(s))
            }
            Tok::SymbolRef(s) => {
                self.bump();
                Ok(Attribute::SymbolRef(s))
            }
            Tok::LBracket => {
                self.bump();
                let mut out = Vec::new();
                if !self.at(&Tok::RBracket) {
                    loop {
                        out.push(self.parse_attribute()?);
                        if !self.eat(&Tok::Comma) {
                            break;
                        }
                    }
                }
                self.expect(Tok::RBracket)?;
                Ok(Attribute::Array(out))
            }
            Tok::LBrace => {
                let entries = self.parse_dict_entries()?;
                Ok(Attribute::Dict(entries))
            }
            Tok::Ident(id) => {
                self.bump();
                match id.as_str() {
                    "true" => Ok(Attribute::Bool(true)),
                    "false" => Ok(Attribute::Bool(false)),
                    "unit" => Ok(Attribute::Unit),
                    // Builtin `array<elemty: v, v, ...>` (DenseArrayAttr) and
                    // `dense<...> : type` (DenseElementsAttr): not CIR-specific
                    // (they show up e.g. in `dlti`/LLVM function attributes),
                    // so we don't interpret their contents structurally.
                    "array" | "dense" if self.at(&Tok::Less) => {
                        self.bump();
                        let raw = self.capture_raw_body();
                        let ty = self.maybe_colon_type()?;
                        Ok(Attribute::Dialect {
                            dialect: "builtin".to_string(),
                            mnemonic: id,
                            raw: Some(raw),
                            ty,
                        })
                    }
                    _ => Err(self.err(format!(
                        "unexpected identifier `{id}` in attribute position"
                    ))),
                }
            }
            Tok::Number(_) => {
                let n = self.expect_number()?;
                let ty = self.maybe_colon_type()?;
                if n.contains('.') || n.contains('e') || n.contains('E') {
                    Ok(Attribute::Float { text: n, ty })
                } else {
                    let value: i128 = n.parse().map_err(|_| self.err("invalid integer literal"))?;
                    Ok(Attribute::Int { value, ty })
                }
            }
            Tok::Bang => {
                let ty = self.parse_type()?;
                Ok(Attribute::Type(ty))
            }
            other => Err(self.err(format!("expected attribute, found {other:?}"))),
        }
    }

    /// Consumes `: type` if present, otherwise returns `None`.
    fn maybe_colon_type(&mut self) -> Result<Option<crate::ast::Type>> {
        if self.eat(&Tok::Colon) {
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }

    fn parse_dialect_attr(&mut self, dialect: &str, mnemonic: &str) -> Result<Attribute> {
        if dialect != "cir" {
            let raw = self.maybe_capture_attr_body();
            let ty = self.maybe_colon_type()?;
            return Ok(Attribute::Dialect {
                dialect: dialect.to_string(),
                mnemonic: mnemonic.to_string(),
                raw,
                ty,
            });
        }
        match mnemonic {
            "int" => self.parse_cir_int_attr(),
            "fp" => self.parse_cir_fp_attr(),
            "bool" => self.parse_cir_bool_attr(),
            "const_array" => self.parse_cir_const_array(),
            "const_vector" => self.parse_cir_const_list("vector"),
            "const_record" => self.parse_cir_const_list("record"),
            "const_complex" => self.parse_cir_const_complex(),
            "global_view" => self.parse_cir_global_view(),
            "bitfield_info" => self.parse_cir_bitfield_info(),
            "ptr" => self.parse_cir_ptr_attr(),
            "block_addr_info" => self.parse_cir_block_addr_info_attr(),
            "zero" => self.parse_cir_typed_marker(|ty| Attribute::Zero { ty }),
            "poison" => self.parse_cir_typed_marker(|ty| Attribute::Poison { ty }),
            _ => {
                let raw = self.maybe_capture_attr_body();
                let ty = self.maybe_colon_type()?;
                Ok(Attribute::Dialect {
                    dialect: "cir".to_string(),
                    mnemonic: mnemonic.to_string(),
                    raw,
                    ty,
                })
            }
        }
    }

    fn maybe_capture_attr_body(&mut self) -> Option<String> {
        if self.eat(&Tok::Less) {
            Some(self.capture_raw_body())
        } else {
            None
        }
    }

    fn parse_cir_int_attr(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let text = self.expect_number()?;
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::CirInt { value: text, ty })
    }

    fn parse_cir_fp_attr(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let text = self.expect_number()?;
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::CirFloat { value: text, ty })
    }

    fn parse_cir_bool_attr(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let value = match self.expect_ident()?.as_str() {
            "true" => true,
            "false" => false,
            other => return Err(self.err(format!("expected `true`/`false`, found `{other}`"))),
        };
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::CirBool { value, ty })
    }

    fn parse_cir_const_array(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let elts = match self.tok().clone() {
            Tok::Str(_) => {
                let tok = self.bump();
                // re-decode from the raw source span (not the lexer's lossy
                // `String`-typed `Tok::Str`): these bytes are arbitrary data,
                // not necessarily valid UTF-8.
                let body = &self.src.as_bytes()[tok.pos + 1..tok.end - 1];
                self.expect(Tok::Colon)?;
                let _elem_ty = self.parse_type()?;
                Attribute::Str(
                    crate::lexer::decode_escaped_bytes(body)
                        .into_iter()
                        .map(char::from)
                        .collect(),
                )
            }
            Tok::LBracket => {
                self.bump();
                let mut elems = Vec::new();
                if !self.at(&Tok::RBracket) {
                    loop {
                        elems.push(self.parse_attribute()?);
                        if !self.eat(&Tok::Comma) {
                            break;
                        }
                    }
                }
                self.expect(Tok::RBracket)?;
                Attribute::Array(elems)
            }
            other => return Err(self.err(format!("expected string or `[...]`, found {other:?}"))),
        };
        let mut trailing_zeros = false;
        if self.eat(&Tok::Comma) {
            self.expect_ident_matching_attr("trailing_zeros")?;
            trailing_zeros = true;
        }
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::ConstArray {
            ty,
            elts: Box::new(elts),
            trailing_zeros_num: i32::from(trailing_zeros),
        })
    }

    /// `#cir.const_vector<[...]>` / `#cir.const_record<{...}>` both hold a
    /// bracketed list of element attributes.
    fn parse_cir_const_list(&mut self, shape: &str) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let (open, close) = if shape == "record" {
            (Tok::LBrace, Tok::RBrace)
        } else {
            (Tok::LBracket, Tok::RBracket)
        };
        self.expect(open)?;
        let mut elems = Vec::new();
        if !self.at(&close) {
            loop {
                elems.push(self.parse_attribute()?);
                if !self.eat(&Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(close)?;
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        if shape == "record" {
            Ok(Attribute::ConstRecord {
                ty,
                members: Box::new(Attribute::Array(elems)),
            })
        } else {
            Ok(Attribute::ConstVector {
                ty,
                elts: Box::new(Attribute::Array(elems)),
            })
        }
    }

    fn parse_cir_const_complex(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let real = self.parse_attribute()?;
        self.expect(Tok::Comma)?;
        let imag = self.parse_attribute()?;
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::ConstComplex {
            real: Box::new(real),
            imag: Box::new(imag),
            ty,
        })
    }

    fn parse_cir_global_view(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let symbol = self.expect_symbol_ref()?;
        let mut indices = Vec::new();
        if self.eat(&Tok::Comma) {
            self.expect(Tok::LBracket)?;
            if !self.at(&Tok::RBracket) {
                loop {
                    let n = self.expect_number()?;
                    let value: i128 = n
                        .parse()
                        .map_err(|_| self.err("invalid global_view index"))?;
                    indices.push(value);
                    let _ = self.maybe_colon_type()?;
                    if !self.eat(&Tok::Comma) {
                        break;
                    }
                }
            }
            self.expect(Tok::RBracket)?;
        }
        self.expect(Tok::Greater)?;
        let ty = self.maybe_colon_type()?.unwrap_or(crate::ast::Type::Void);
        let indices = if indices.is_empty() {
            None
        } else {
            Some(Box::new(Attribute::Array(
                indices
                    .into_iter()
                    .map(|value| Attribute::Int { value, ty: None })
                    .collect(),
            )))
        };
        Ok(Attribute::GlobalView {
            ty,
            symbol,
            indices,
        })
    }

    fn parse_cir_bitfield_info(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let mut name = None;
        let mut storage_type = None;
        let mut size = None;
        let mut offset = None;
        let mut is_signed = None;
        loop {
            let key = self.expect_ident()?;
            self.expect(Tok::Equal)?;
            match key.as_str() {
                "name" => name = Some(self.expect_str()?),
                "storage_type" => storage_type = Some(self.parse_type()?),
                "size" => size = Some(self.parse_attribute()?),
                "offset" => offset = Some(self.parse_attribute()?),
                "is_signed" => is_signed = Some(self.parse_attribute()?),
                other => {
                    return Err(self.err(format!("unexpected bitfield_info field `{other}`")));
                }
            }
            if !self.eat(&Tok::Comma) {
                break;
            }
        }
        self.expect(Tok::Greater)?;
        let name = name.ok_or_else(|| self.err("bitfield_info missing `name`"))?;
        let storage_type =
            storage_type.ok_or_else(|| self.err("bitfield_info missing `storage_type`"))?;
        let size = size
            .and_then(|a| a.as_int())
            .ok_or_else(|| self.err("bitfield_info missing/invalid `size`"))?
            as u64;
        let offset = offset
            .and_then(|a| a.as_int())
            .ok_or_else(|| self.err("bitfield_info missing/invalid `offset`"))?
            as u64;
        let is_signed = is_signed
            .and_then(|a| a.as_bool())
            .ok_or_else(|| self.err("bitfield_info missing/invalid `is_signed`"))?;
        Ok(Attribute::BitfieldInfo {
            name,
            storage_type,
            size,
            offset,
            is_signed,
        })
    }

    fn parse_cir_ptr_attr(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let value = if self.eat(&Tok::Ident("null".to_string())) {
            Attribute::Int { value: 0, ty: None }
        } else {
            let text = self.expect_number()?;
            let ty = self.maybe_colon_type()?;
            Attribute::Int {
                value: text
                    .parse()
                    .map_err(|_| self.err("invalid integer literal"))?,
                ty,
            }
        };
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::ConstPtr {
            ty,
            value: Box::new(value),
        })
    }

    fn parse_cir_block_addr_info_attr(&mut self) -> Result<Attribute> {
        self.expect(Tok::Less)?;
        let func = self.expect_symbol_ref()?;
        self.expect(Tok::Comma)?;
        let label = self.expect_str()?;
        self.expect(Tok::Greater)?;
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(Attribute::BlockAddrInfo { ty, func, label })
    }

    fn parse_cir_typed_marker(
        &mut self,
        make: impl FnOnce(crate::ast::Type) -> Attribute,
    ) -> Result<Attribute> {
        self.expect(Tok::Colon)?;
        let ty = self.parse_type()?;
        Ok(make(ty))
    }

    fn expect_ident_matching_attr(&mut self, want: &str) -> Result<()> {
        let id = self.expect_ident()?;
        if id == want {
            Ok(())
        } else {
            Err(self.err(format!("expected `{want}`, found `{id}`")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_const_array_bytes_losslessly() {
        let mut parser =
            Parser::new(r#"<"\7F\FF" : !s8i, trailing_zeros> : !cir.array<!s8i x 3>"#).unwrap();
        let Attribute::ConstArray { elts, .. } = parser.parse_cir_const_array().unwrap() else {
            panic!("expected a constant array")
        };
        let Attribute::Str(value) = *elts else {
            panic!("expected a string constant array")
        };
        assert_eq!(
            value.chars().map(u32::from).collect::<Vec<_>>(),
            [0x7f, 0xff]
        );
    }
}
