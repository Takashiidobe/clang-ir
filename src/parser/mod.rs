//! Recursive-descent parser for the generic MLIR textual syntax produced by
//! `cir-opt --mlir-print-op-generic`, specialized to interpret CIR dialect
//! types/attributes structurally where practical.

mod attrs;
mod types;

use crate::ast::{Attribute, Block, GenericModule, Operation, Region, Type};
use crate::error::{Error, Result};
use crate::lexer::{Lexer, SpannedTok, Tok};

pub struct Parser<'a> {
    src: &'a str,
    toks: Vec<SpannedTok>,
    pos: usize,
}

pub fn parse_generic_module(src: &str) -> Result<GenericModule> {
    let mut p = Parser::new(src)?;
    p.parse_module()
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Result<Self> {
        let toks = Lexer::tokenize(src)?;
        Ok(Parser { src, toks, pos: 0 })
    }

    fn cur(&self) -> &SpannedTok {
        &self.toks[self.pos]
    }

    fn tok(&self) -> &Tok {
        &self.cur().tok
    }

    fn at(&self, t: &Tok) -> bool {
        self.tok() == t
    }

    fn bump(&mut self) -> SpannedTok {
        let t = self.toks[self.pos].clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    fn eat(&mut self, t: &Tok) -> bool {
        if self.at(t) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn err(&self, msg: impl Into<String>) -> Error {
        let c = self.cur();
        Error::Parse {
            line: c.line,
            col: c.col,
            msg: msg.into(),
        }
    }

    fn expect(&mut self, t: Tok) -> Result<SpannedTok> {
        if self.at(&t) {
            Ok(self.bump())
        } else {
            Err(self.err(format!("expected {:?}, found {:?}", t, self.tok())))
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        match self.tok().clone() {
            Tok::Ident(s) => {
                self.bump();
                Ok(s)
            }
            other => Err(self.err(format!("expected identifier, found {other:?}"))),
        }
    }

    fn expect_value(&mut self) -> Result<String> {
        match self.tok().clone() {
            Tok::Value(s) => {
                self.bump();
                // A multi-result op's individual result is referenced at use
                // sites as `%base#N` (declared as `%base:N = ...`).
                if self.eat(&Tok::Hash) {
                    let idx = self.expect_number()?;
                    Ok(format!("{s}#{idx}"))
                } else {
                    Ok(s)
                }
            }
            other => Err(self.err(format!("expected %value, found {other:?}"))),
        }
    }

    fn expect_str(&mut self) -> Result<String> {
        match self.tok().clone() {
            Tok::Str(s) => {
                self.bump();
                Ok(s)
            }
            other => Err(self.err(format!("expected string literal, found {other:?}"))),
        }
    }

    fn expect_symbol_ref(&mut self) -> Result<String> {
        match self.tok().clone() {
            Tok::SymbolRef(s) => {
                self.bump();
                Ok(s)
            }
            other => Err(self.err(format!("expected @symbol, found {other:?}"))),
        }
    }

    fn expect_block_label(&mut self) -> Result<String> {
        match self.tok().clone() {
            Tok::BlockLabel(s) => {
                self.bump();
                Ok(s)
            }
            other => Err(self.err(format!("expected ^block label, found {other:?}"))),
        }
    }

    fn expect_number(&mut self) -> Result<String> {
        match self.tok().clone() {
            Tok::Number(s) => {
                self.bump();
                Ok(s)
            }
            other => Err(self.err(format!("expected number, found {other:?}"))),
        }
    }

    /// Captures the raw source text of a bracketed body, given that the
    /// opening delimiter has already been consumed (its matching depth is 1).
    /// Stops and consumes the matching closer, returning the text strictly
    /// between the two delimiters. Used as the fallback representation for
    /// dialect types/attributes we don't interpret structurally.
    fn capture_raw_body(&mut self) -> String {
        let start = self.cur().pos;
        let mut depth: i32 = 1;
        loop {
            match self.tok() {
                Tok::Less | Tok::LParen | Tok::LBrace | Tok::LBracket => depth += 1,
                Tok::Greater | Tok::RParen | Tok::RBrace | Tok::RBracket => depth -= 1,
                Tok::Eof => {
                    let end = self.cur().pos;
                    return self.src[start..end].to_string();
                }
                _ => {}
            }
            if depth == 0 {
                let end = self.cur().pos;
                self.bump();
                return self.src[start..end].to_string();
            }
            self.bump();
        }
    }

    // ---- top-level module ----

    fn parse_module(&mut self) -> Result<GenericModule> {
        let mut m = GenericModule::default();
        loop {
            match self.tok() {
                Tok::Bang => {
                    self.bump();
                    let name = self.expect_ident()?;
                    self.expect(Tok::Equal)?;
                    let ty = self.parse_type()?;
                    m.type_aliases.insert(name, ty);
                }
                Tok::Hash => {
                    self.bump();
                    let name = self.expect_ident()?;
                    self.expect(Tok::Equal)?;
                    let attr = self.parse_attribute()?;
                    m.attr_aliases.insert(name, attr);
                }
                Tok::Eof => break,
                _ => {
                    let op = self.parse_operation()?;
                    m.ops.push(op);
                }
            }
        }
        Ok(m)
    }

    // ---- operations / regions / blocks ----

    fn parse_operation(&mut self) -> Result<Operation> {
        let mut result_ids = Vec::new();
        if matches!(self.tok(), Tok::Value(_)) {
            loop {
                let base = match self.tok().clone() {
                    Tok::Value(s) => {
                        self.bump();
                        s
                    }
                    _ => unreachable!(),
                };
                // A grouped multi-result declaration `%base:N` expands to
                // `N` individual results `%base#0 .. %base#(N-1)`.
                if self.eat(&Tok::Colon) {
                    let n: usize = self
                        .expect_number()?
                        .parse()
                        .map_err(|_| self.err("invalid result group count"))?;
                    for i in 0..n {
                        result_ids.push(format!("{base}#{i}"));
                    }
                } else {
                    result_ids.push(base);
                }
                if !self.eat(&Tok::Comma) {
                    break;
                }
            }
            self.expect(Tok::Equal)?;
        }

        let name = self.expect_str()?;

        self.expect(Tok::LParen)?;
        let mut operands = Vec::new();
        if !self.at(&Tok::RParen) {
            loop {
                operands.push(self.expect_value()?);
                if !self.eat(&Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(Tok::RParen)?;

        let mut successors = Vec::new();
        if self.eat(&Tok::LBracket) {
            if !self.at(&Tok::RBracket) {
                loop {
                    successors.push(self.expect_block_label()?);
                    if !self.eat(&Tok::Comma) {
                        break;
                    }
                }
            }
            self.expect(Tok::RBracket)?;
        }

        let properties = if self.eat(&Tok::Less) {
            let entries = self.parse_dict_entries()?;
            self.expect(Tok::Greater)?;
            entries
        } else {
            Vec::new()
        };

        let mut regions = Vec::new();
        if self.eat(&Tok::LParen) {
            if !self.at(&Tok::RParen) {
                loop {
                    regions.push(self.parse_region()?);
                    if !self.eat(&Tok::Comma) {
                        break;
                    }
                }
            }
            self.expect(Tok::RParen)?;
        }

        let attributes = if self.at(&Tok::LBrace) {
            self.parse_dict_entries()?
        } else {
            Vec::new()
        };

        self.expect(Tok::Colon)?;
        let (_input_tys, result_tys) = self.parse_function_type()?;
        if result_tys.len() != result_ids.len() {
            return Err(self.err(format!(
                "operation '{name}' has {} result value(s) but function type lists {}",
                result_ids.len(),
                result_tys.len()
            )));
        }
        let results = result_ids.into_iter().zip(result_tys).collect();

        Ok(Operation {
            name,
            results,
            operands,
            successors,
            properties,
            regions,
            attributes,
        })
    }

    fn parse_region(&mut self) -> Result<Region> {
        self.expect(Tok::LBrace)?;
        let mut blocks = Vec::new();
        if !self.at(&Tok::RBrace) {
            blocks.push(self.parse_block(true)?);
            while matches!(self.tok(), Tok::BlockLabel(_)) {
                blocks.push(self.parse_block(false)?);
            }
        }
        self.expect(Tok::RBrace)?;
        Ok(Region { blocks })
    }

    fn parse_block(&mut self, is_first: bool) -> Result<Block> {
        let mut label = None;
        let mut args = Vec::new();
        if matches!(self.tok(), Tok::BlockLabel(_)) {
            label = Some(self.expect_block_label()?);
            if self.eat(&Tok::LParen) {
                if !self.at(&Tok::RParen) {
                    loop {
                        let id = self.expect_value()?;
                        self.expect(Tok::Colon)?;
                        let ty = self.parse_type()?;
                        args.push((id, ty));
                        if !self.eat(&Tok::Comma) {
                            break;
                        }
                    }
                }
                self.expect(Tok::RParen)?;
            }
            self.expect(Tok::Colon)?;
        } else if !is_first {
            return Err(self.err(format!("expected ^block label, found {:?}", self.tok())));
        }

        let mut ops = Vec::new();
        while !self.at(&Tok::RBrace) && !matches!(self.tok(), Tok::BlockLabel(_)) {
            ops.push(self.parse_operation()?);
        }
        Ok(Block { label, args, ops })
    }

    // ---- shared: dict / function-type ----

    fn parse_dict_entries(&mut self) -> Result<Vec<(String, Attribute)>> {
        self.expect(Tok::LBrace)?;
        let mut out = Vec::new();
        if !self.at(&Tok::RBrace) {
            loop {
                let key = match self.tok().clone() {
                    Tok::Ident(s) => {
                        self.bump();
                        s
                    }
                    Tok::Str(s) => {
                        self.bump();
                        s
                    }
                    other => {
                        return Err(self.err(format!("expected attribute key, found {other:?}")));
                    }
                };
                let val = if self.eat(&Tok::Equal) {
                    self.parse_attribute()?
                } else {
                    Attribute::Unit
                };
                out.push((key, val));
                if !self.eat(&Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(Tok::RBrace)?;
        Ok(out)
    }

    fn parse_function_type(&mut self) -> Result<(Vec<Type>, Vec<Type>)> {
        let inputs = self.parse_type_list_parens()?;
        self.expect(Tok::Arrow)?;
        let results = if self.at(&Tok::LParen) {
            self.parse_type_list_parens()?
        } else {
            vec![self.parse_type()?]
        };
        Ok((inputs, results))
    }

    fn parse_type_list_parens(&mut self) -> Result<Vec<Type>> {
        self.expect(Tok::LParen)?;
        let mut out = Vec::new();
        if !self.at(&Tok::RParen) {
            loop {
                out.push(self.parse_type()?);
                if !self.eat(&Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(Tok::RParen)?;
        Ok(out)
    }
}
