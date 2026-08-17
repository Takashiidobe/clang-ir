//! Hand-written lexer for the generic MLIR textual syntax used by `cir-opt
//! --mlir-print-op-generic` output (operations, regions, CIR types/attributes).

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Ident(String),
    /// `%name` or `%42` (the text after `%`)
    Value(String),
    /// `@name` (the text after `@`, quotes already stripped)
    SymbolRef(String),
    /// `^bb0` (the text after `^`)
    BlockLabel(String),
    /// Raw text of a numeric literal, exactly as written (sign, digits, `.`, exponent, hex).
    Number(String),
    /// Decoded contents of a `"..."` string literal.
    Str(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Less,
    Greater,
    Colon,
    Comma,
    Equal,
    Bang,
    Hash,
    Arrow,
    Star,
    Pipe,
    Dot,
    Question,
    Ellipsis,
    Eof,
}

#[derive(Debug, Clone)]
pub struct SpannedTok {
    pub tok: Tok,
    pub pos: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: usize,
    col: usize,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("lex error at {line}:{col}: {msg}")]
pub struct LexError {
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(src: &'a str) -> Result<Vec<SpannedTok>, LexError> {
        let mut lex = Lexer::new(src);
        let mut out = Vec::new();
        loop {
            let t = lex.next_token()?;
            let eof = matches!(t.tok, Tok::Eof);
            out.push(t);
            if eof {
                break;
            }
        }
        Ok(out)
    }

    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_byte_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.peek_byte()?;
        self.pos += 1;
        if b == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(b)
    }

    fn err(&self, msg: impl Into<String>) -> LexError {
        LexError {
            line: self.line,
            col: self.col,
            msg: msg.into(),
        }
    }

    fn skip_trivia(&mut self) {
        loop {
            match self.peek_byte() {
                Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => {
                    self.advance();
                }
                Some(b'/') if self.peek_byte_at(1) == Some(b'/') => {
                    while let Some(b) = self.peek_byte() {
                        if b == b'\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn is_ident_start(b: u8) -> bool {
        b.is_ascii_alphabetic() || b == b'_' || b == b'$' || b == b'.'
    }

    fn is_ident_continue(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'-' || b == b'.'
    }

    /// Reads a bare identifier (letters/digits/`_`/`$`/`-`), used for the text
    /// following sigils like `%`, `@`, `^`.
    fn read_bare_id(&mut self) -> String {
        let start = self.pos;
        while let Some(b) = self.peek_byte() {
            if Self::is_ident_continue(b) {
                self.advance();
            } else {
                break;
            }
        }
        self.src[start..self.pos].to_string()
    }

    fn read_quoted_string(&mut self) -> Result<String, LexError> {
        // opening quote already consumed by caller
        let mut out = Vec::new();
        loop {
            let b = self
                .advance()
                .ok_or_else(|| self.err("unterminated string literal"))?;
            match b {
                b'"' => break,
                b'\\' => {
                    let esc = self
                        .advance()
                        .ok_or_else(|| self.err("unterminated escape"))?;
                    match esc {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        hex if hex.is_ascii_hexdigit() => {
                            let hex2 = self
                                .advance()
                                .ok_or_else(|| self.err("truncated hex escape"))?;
                            if !hex2.is_ascii_hexdigit() {
                                return Err(self.err("invalid hex escape"));
                            }
                            let hi = (hex as char).to_digit(16).unwrap();
                            let lo = (hex2 as char).to_digit(16).unwrap();
                            out.push((hi * 16 + lo) as u8);
                        }
                        other => out.push(other),
                    }
                }
                other => out.push(other),
            }
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    fn read_number(&mut self) -> String {
        let start = self.pos;
        if self.peek_byte() == Some(b'-') {
            self.advance();
        }
        // hex literal
        if self.peek_byte() == Some(b'0') && matches!(self.peek_byte_at(1), Some(b'x') | Some(b'X'))
        {
            self.advance();
            self.advance();
            while let Some(b) = self.peek_byte() {
                if b.is_ascii_hexdigit() {
                    self.advance();
                } else {
                    break;
                }
            }
            return self.src[start..self.pos].to_string();
        }
        while let Some(b) = self.peek_byte() {
            if b.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek_byte() == Some(b'.')
            && self.peek_byte_at(1).is_some_and(|b| b.is_ascii_digit())
        {
            self.advance();
            while let Some(b) = self.peek_byte() {
                if b.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        if matches!(self.peek_byte(), Some(b'e') | Some(b'E')) {
            let save = self.pos;
            self.advance();
            if matches!(self.peek_byte(), Some(b'+') | Some(b'-')) {
                self.advance();
            }
            if self.peek_byte().is_some_and(|b| b.is_ascii_digit()) {
                while let Some(b) = self.peek_byte() {
                    if b.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            } else {
                self.pos = save;
            }
        }
        self.src[start..self.pos].to_string()
    }

    pub fn next_token(&mut self) -> Result<SpannedTok, LexError> {
        self.skip_trivia();
        let (line, col, pos) = (self.line, self.col, self.pos);
        macro_rules! mk {
            ($tok:expr) => {
                SpannedTok {
                    tok: $tok,
                    pos,
                    end: self.pos,
                    line,
                    col,
                }
            };
        }

        let b = match self.peek_byte() {
            None => return Ok(mk!(Tok::Eof)),
            Some(b) => b,
        };

        match b {
            b'(' => {
                self.advance();
                Ok(mk!(Tok::LParen))
            }
            b')' => {
                self.advance();
                Ok(mk!(Tok::RParen))
            }
            b'{' => {
                self.advance();
                Ok(mk!(Tok::LBrace))
            }
            b'}' => {
                self.advance();
                Ok(mk!(Tok::RBrace))
            }
            b'[' => {
                self.advance();
                Ok(mk!(Tok::LBracket))
            }
            b']' => {
                self.advance();
                Ok(mk!(Tok::RBracket))
            }
            b'<' => {
                self.advance();
                Ok(mk!(Tok::Less))
            }
            b'>' => {
                self.advance();
                Ok(mk!(Tok::Greater))
            }
            b':' => {
                self.advance();
                Ok(mk!(Tok::Colon))
            }
            b',' => {
                self.advance();
                Ok(mk!(Tok::Comma))
            }
            b'=' => {
                self.advance();
                Ok(mk!(Tok::Equal))
            }
            b'!' => {
                self.advance();
                Ok(mk!(Tok::Bang))
            }
            b'#' => {
                self.advance();
                Ok(mk!(Tok::Hash))
            }
            b'*' => {
                self.advance();
                Ok(mk!(Tok::Star))
            }
            b'|' => {
                self.advance();
                Ok(mk!(Tok::Pipe))
            }
            b'?' => {
                self.advance();
                Ok(mk!(Tok::Question))
            }
            b'.' => {
                self.advance();
                if self.peek_byte() == Some(b'.') && self.peek_byte_at(1) == Some(b'.') {
                    self.advance();
                    self.advance();
                    Ok(mk!(Tok::Ellipsis))
                } else {
                    Ok(mk!(Tok::Dot))
                }
            }
            b'-' if self.peek_byte_at(1) == Some(b'>') => {
                self.advance();
                self.advance();
                Ok(mk!(Tok::Arrow))
            }
            b'-' if self.peek_byte_at(1).is_some_and(|b| b.is_ascii_digit()) => {
                let n = self.read_number();
                Ok(mk!(Tok::Number(n)))
            }
            b'"' => {
                self.advance();
                let s = self.read_quoted_string()?;
                Ok(mk!(Tok::Str(s)))
            }
            b'%' => {
                self.advance();
                if self.peek_byte() == Some(b'"') {
                    self.advance();
                    let s = self.read_quoted_string()?;
                    Ok(mk!(Tok::Value(s)))
                } else {
                    let id = self.read_bare_id();
                    Ok(mk!(Tok::Value(id)))
                }
            }
            b'@' => {
                self.advance();
                if self.peek_byte() == Some(b'"') {
                    self.advance();
                    let s = self.read_quoted_string()?;
                    Ok(mk!(Tok::SymbolRef(s)))
                } else {
                    let id = self.read_bare_id();
                    Ok(mk!(Tok::SymbolRef(id)))
                }
            }
            b'^' => {
                self.advance();
                let id = self.read_bare_id();
                Ok(mk!(Tok::BlockLabel(id)))
            }
            b'0'..=b'9' => {
                let n = self.read_number();
                Ok(mk!(Tok::Number(n)))
            }
            b if Self::is_ident_start(b) && b != b'.' => {
                let id = self.read_bare_id();
                Ok(mk!(Tok::Ident(id)))
            }
            other => Err(self.err(format!("unexpected byte {:?} ({:?})", other as char, other))),
        }
    }
}

/// Decodes a quoted string literal's escapes (`\"`, `\\`, `\n`, `\t`,
/// `\XX` hex byte) to exact bytes, given the raw source text strictly
/// between the opening and closing `"`. Unlike [`Lexer::read_quoted_string`],
/// this preserves non-UTF-8 byte sequences instead of lossily replacing them:
/// CIR const-array string bodies encode arbitrary byte data (not necessarily
/// text), so callers that need byte-exact contents (e.g. `#cir.const_array<"...">`)
/// re-decode from the token's source span via this function rather than using
/// the lexer's own (lossy, `String`-typed) [`Tok::Str`] value.
pub(crate) fn decode_escaped_bytes(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut i = 0;
    while i < body.len() {
        let b = body[i];
        i += 1;
        if b != b'\\' {
            out.push(b);
            continue;
        }
        let Some(&esc) = body.get(i) else {
            out.push(b);
            break;
        };
        i += 1;
        match esc {
            b'"' => out.push(b'"'),
            b'\\' => out.push(b'\\'),
            b'n' => out.push(b'\n'),
            b't' => out.push(b'\t'),
            hex if hex.is_ascii_hexdigit() => {
                if let Some(&hex2) = body.get(i)
                    && hex2.is_ascii_hexdigit()
                {
                    let hi = (hex as char).to_digit(16).unwrap();
                    let lo = (hex2 as char).to_digit(16).unwrap();
                    out.push((hi * 16 + lo) as u8);
                    i += 1;
                } else {
                    out.push(hex);
                }
            }
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_simple_op() {
        let toks = Lexer::tokenize(r#""cir.const"() <{value = 0 : i32}> : () -> !s32i"#).unwrap();
        assert!(matches!(toks[0].tok, Tok::Str(ref s) if s == "cir.const"));
    }

    #[test]
    fn decodes_hex_escape() {
        let toks = Lexer::tokenize(r#""%d\0A""#).unwrap();
        match &toks[0].tok {
            Tok::Str(s) => assert_eq!(s, "%d\n"),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn lexes_negative_number() {
        let toks = Lexer::tokenize("-10").unwrap();
        assert!(matches!(&toks[0].tok, Tok::Number(n) if n == "-10"));
    }
}
