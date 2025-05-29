use super::parser::{Diagnostic, Span};
use codespan_reporting::diagnostic::Label;
use herring::{Herring, Lexer};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexerError {
    #[default]
    Invalid,
    UnterminatedString,
    Indent,
    InvalidFormatSpec,
    UnterminatedFormatSpec,
}

impl LexerError {
    pub fn into_diagnostic(self, span: Span) -> Diagnostic {
        match self {
            Self::Invalid => Diagnostic::error()
                .with_message("invalid token")
                .with_labels(vec![Label::primary((), span)]),
            Self::UnterminatedString => Diagnostic::error()
                .with_message("unterminated string")
                .with_labels(vec![Label::primary((), span)]),
            Self::Indent => Diagnostic::error()
                .with_message("unindent does not match any outer indentation level")
                .with_labels(vec![Label::primary((), span)]),
            Self::InvalidFormatSpec => Diagnostic::error()
                .with_message("invalid format specifier")
                .with_labels(vec![Label::primary((), span)]),
            Self::UnterminatedFormatSpec => Diagnostic::error()
                .with_message("unterminated format specifier")
                .with_labels(vec![Label::primary((), span)]),
        }
    }
}

fn lpar(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    lexer.extras.paren += 1;
    Ok(Token::LPar)
}
fn rpar(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    lexer.extras.paren = lexer.extras.paren.saturating_sub(1);
    Ok(Token::RPar)
}
fn lbrak(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    lexer.extras.paren += 1;
    Ok(Token::LBrak)
}
fn rbrak(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    lexer.extras.paren = lexer.extras.paren.saturating_sub(1);
    Ok(Token::RBrak)
}
fn lbrace(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    if !lexer.extras.strings_stack.is_empty() {
        return Ok(Token::LBrace);
        // inside format after that
    }
    lexer.extras.paren += 1;
    Ok(Token::LBrace)
}
fn rbrace(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    if let Some(kind) = lexer.extras.strings_stack.last() {
        // lexer.bump(1);
        parse_string_middle(lexer, *kind)?;
        return Ok(Token::RBrace);
        // out of format after that
    }
    lexer.extras.paren = lexer.extras.paren.saturating_sub(1);
    Ok(Token::RBrace)
}

fn parse_string_middle(
    lexer: &mut Lexer<'_, Token>,
    kind: StringKind,
) -> Result<Token, LexerError> {
    match kind {
        StringKind::ShortDouble => parse_short_double_fstring(lexer),
        StringKind::ShortSingle => parse_short_single_fstring(lexer),
        StringKind::LongDouble => parse_long_double_fstring(lexer),
        StringKind::LongSingle => parse_long_single_fstring(lexer),
    }
}

fn parse_short_double_fstring(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    while let Some(c) = it.next() {
        match c {
            '"' => {
                lexer.bump(1);
                lexer.extras.strings_stack.pop();
                return Ok(Token::FString);
            }
            '\\' => {
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            '\n' => {
                break;
            }
            '{' if it.next() == Some('{') => {
                lexer.bump('{'.len_utf8() * 2);
            }
            '{' => {
                return Ok(Token::FString);
            }
            c => {
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_short_single_fstring(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    while let Some(c) = it.next() {
        match c {
            '\'' => {
                lexer.bump(1);
                lexer.extras.strings_stack.pop();
                return Ok(Token::FString);
            }
            '\\' => {
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            '\n' => {
                break;
            }
            '{' if it.next() == Some('{') => {
                lexer.bump('{'.len_utf8() * 2);
            }
            '{' => {
                return Ok(Token::FString);
            }
            c => {
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_long_double_fstring(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    let mut closing = 0;
    while let Some(c) = it.next() {
        match c {
            '"' if closing == 2 => {
                lexer.bump(1);
                lexer.extras.strings_stack.pop();
                return Ok(Token::FString);
            }
            '"' => {
                lexer.bump(1);
                closing += 1;
            }
            '\\' => {
                closing = 0;
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            '{' if it.next() == Some('{') => {
                lexer.bump('{'.len_utf8() * 2);
            }
            '{' => {
                return Ok(Token::FString);
            }
            c => {
                closing = 0;
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_long_single_fstring(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    let mut closing = 0;
    while let Some(c) = it.next() {
        match c {
            '\'' if closing == 2 => {
                lexer.bump(1);
                lexer.extras.strings_stack.pop();
                return Ok(Token::FString);
            }
            '\'' => {
                lexer.bump(1);
                closing += 1;
            }
            '\\' => {
                closing = 0;
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            '{' if it.next() == Some('{') => {
                lexer.bump('{'.len_utf8() * 2);
            }
            '{' => {
                return Ok(Token::FString);
            }
            c => {
                closing = 0;
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_short_double_string(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    while let Some(c) = it.next() {
        match c {
            '"' => {
                lexer.bump(1);
                return Ok(Token::String);
            }
            '\\' => {
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            '\n' => {
                break;
            }
            c => {
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_short_single_string(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    while let Some(c) = it.next() {
        match c {
            '\'' => {
                lexer.bump(1);
                return Ok(Token::String);
            }
            '\\' => {
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            '\n' => {
                break;
            }
            c => {
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_long_double_string(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    let mut closing = 0;
    while let Some(c) = it.next() {
        match c {
            '"' if closing == 2 => {
                lexer.bump(1);
                return Ok(Token::String);
            }
            '"' => {
                lexer.bump(1);
                closing += 1;
            }
            '\\' => {
                closing = 0;
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            c => {
                closing = 0;
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}
fn parse_long_single_string(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    let mut it = lexer.remainder().chars();
    let mut closing = 0;
    while let Some(c) = it.next() {
        match c {
            '\'' if closing == 2 => {
                lexer.bump(1);
                return Ok(Token::String);
            }
            '\'' => {
                lexer.bump(1);
                closing += 1;
            }
            '\\' => {
                closing = 0;
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            c => {
                closing = 0;
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}

fn check_indent(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    if lexer.remainder().starts_with('\n')
        || lexer.remainder().starts_with('#')
        || lexer.remainder().starts_with("\r\n")
    {
        return Ok(Token::Whitespace);
    }
    if lexer.extras.paren == 0 {
        let mut error = false;
        let mut indent = 0;
        let nl_index = lexer.slice().find('\n').unwrap();
        for c in lexer.slice()[nl_index + 1..].chars() {
            if c == '\t' {
                indent += 8 - (indent % 8);
            } else {
                indent += 1;
            }
        }
        if indent > *lexer.extras.indent.last().unwrap() {
            lexer.extras.pending_indent += 1;
            lexer.extras.indent.push(indent);
        } else {
            let mut count = 0;
            for level in lexer.extras.indent.iter().rev() {
                if *level == indent {
                    break;
                }
                if *level < indent {
                    error = true;
                    break;
                }
                count += 1;
            }
            lexer.extras.pending_dedent += count;
            lexer
                .extras
                .indent
                .truncate(lexer.extras.indent.len() - count);
        }
        if error {
            Err(LexerError::Indent)
        } else {
            Ok(Token::Newline)
        }
    } else {
        Ok(Token::Whitespace)
    }
}
fn check_first_line_indent(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    if lexer.span().start == 0 {
        lexer.extras.pending_indent += 1;
        lexer.extras.indent.push(lexer.span().end);
    }
    Ok(Token::Whitespace)
}
fn emit_indent_dedent(lexer: &mut Lexer<'_, Token>) -> Option<Result<Token, LexerError>> {
    if lexer.extras.pending_indent > 0 {
        lexer.extras.pending_indent -= 1;
        return Some(Ok(Token::Indent));
    }
    if lexer.extras.pending_dedent > 0 {
        lexer.extras.pending_dedent -= 1;
        return Some(Ok(Token::Dedent));
    }
    None
}

#[derive(Debug, Clone, Copy)]
enum StringKind {
    ShortDouble,
    ShortSingle,
    LongDouble,
    LongSingle,
}

pub struct Context {
    paren: usize,
    indent: Vec<usize>,
    pending_indent: usize,
    pending_dedent: usize,
    strings_stack: Vec<StringKind>,
}
impl Default for Context {
    fn default() -> Self {
        Context {
            paren: 0,
            indent: vec![0],
            pending_indent: 0,
            pending_dedent: 0,
            strings_stack: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FormatSpecState {
    Start,        // Beginning of format spec
    Fill,         // Fill character seen
    Align,        // Alignment seen
    Sign,         // Sign seen
    Z,            // 'z' option seen
    Hash,         // '#' option seen
    Zero,         // '0' padding seen
    Width,        // Width digits seen
    Grouping,     // Grouping (',' or '_') seen
    PrecisionDot, // '.' for precision seen
    Precision,    // Precision digits seen
    Type,         // Type character seen
}

impl FormatSpecState {
    #[inline]
    fn allows_fill_at_start(self) -> bool {
        self == FormatSpecState::Start
    }

    #[inline]
    fn allows_precision_digits(self) -> bool {
        self == FormatSpecState::PrecisionDot
    }
}

// as we can't properly parse the colon in f-strings, we shall ensure the format is correct at least
// according to docs
// format_spec ::= [options][width][grouping]["." precision][type]
// TODO! need to return an error for this edge case: f"{'x':}>5}"
fn parse_colon(lexer: &mut Lexer<'_, Token>) -> Result<Token, LexerError> {
    if lexer.extras.strings_stack.is_empty() {
        return Ok(Token::Colon);
    }

    let remainder = lexer.remainder();
    let bytes = remainder.as_bytes();
    let mut pos = 0;
    let len = bytes.len();
    let mut state = FormatSpecState::Start;

    if len >= 2 {
        let first = bytes[0];
        let second = bytes[1];
        if is_align(second) && is_valid_fill(first) {
            pos = 2;
            lexer.bump(2);
            state = FormatSpecState::Align;
        }
    }

    while pos < len {
        let b = bytes[pos];

        match b {
            b'}' => {
                if state == FormatSpecState::PrecisionDot {
                    return Err(LexerError::InvalidFormatSpec);
                }
                return Ok(Token::FormatSpecifier);
            }

            b'{' => return parse_nested_format_spec_fast(lexer, bytes, pos),

            b'<' | b'>' | b'=' | b'^' => {
                if state > FormatSpecState::Fill {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::Align;
                pos += 1;
                lexer.bump(1);
            }

            b'+' | b'-' | b' ' => {
                if state > FormatSpecState::Align {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::Sign;
                pos += 1;
                lexer.bump(1);
            }

            b'z' => {
                if state > FormatSpecState::Sign {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::Z;
                pos += 1;
                lexer.bump(1);
            }

            b'#' => {
                if state > FormatSpecState::Z {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::Hash;
                pos += 1;
                lexer.bump(1);
            }

            b'0' => {
                if state.allows_precision_digits() {
                    let (new_pos, consumed) = consume_digits(bytes, pos, len);
                    state = FormatSpecState::Precision;
                    pos = new_pos;
                    lexer.bump(consumed);
                } else if state <= FormatSpecState::Hash {
                    state = FormatSpecState::Zero;
                    pos += 1;
                    lexer.bump(1);
                } else if state <= FormatSpecState::Zero {
                    let (new_pos, consumed) = consume_digits(bytes, pos, len);
                    state = FormatSpecState::Width;
                    pos = new_pos;
                    lexer.bump(consumed);
                } else {
                    return Err(LexerError::InvalidFormatSpec);
                }
            }

            b'1'..=b'9' => {
                if state.allows_precision_digits() {
                    let (new_pos, consumed) = consume_digits(bytes, pos, len);
                    state = FormatSpecState::Precision;
                    pos = new_pos;
                    lexer.bump(consumed);
                } else if state <= FormatSpecState::Zero {
                    let (new_pos, consumed) = consume_digits(bytes, pos, len);
                    state = FormatSpecState::Width;
                    pos = new_pos;
                    lexer.bump(consumed);
                } else {
                    return Err(LexerError::InvalidFormatSpec);
                }
            }

            b',' | b'_' => {
                if state > FormatSpecState::Width {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::Grouping;
                pos += 1;
                lexer.bump(1);
            }

            b'.' => {
                if state > FormatSpecState::Grouping {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::PrecisionDot;
                pos += 1;
                lexer.bump(1);
            }

            b'b' | b'c' | b'd' | b'e' | b'E' | b'f' | b'F' | b'g' | b'G' | b'n' | b'o' | b's'
            | b'x' | b'X' | b'%' => {
                if state == FormatSpecState::PrecisionDot || state > FormatSpecState::Precision {
                    return Err(LexerError::InvalidFormatSpec);
                }
                state = FormatSpecState::Type;
                pos += 1;
                lexer.bump(1);
            }

            _ => {
                if state.allows_fill_at_start() {
                    // Should have been caught by fill+align check
                    return Err(LexerError::InvalidFormatSpec);
                }
                return Err(LexerError::InvalidFormatSpec);
            }
        }
    }

    Err(LexerError::UnterminatedFormatSpec)
}

#[inline]
const fn is_valid_fill(b: u8) -> bool {
    !matches!(b, b'{' | b'}')
}

#[inline]
const fn is_align(b: u8) -> bool {
    matches!(b, b'<' | b'>' | b'=' | b'^')
}

#[inline]
fn consume_digits(bytes: &[u8], start: usize, len: usize) -> (usize, usize) {
    let mut pos = start;
    while pos < len && bytes[pos].is_ascii_digit() {
        pos += 1;
    }
    (pos, pos - start)
}

fn parse_nested_format_spec_fast(
    lexer: &mut Lexer<'_, Token>,
    bytes: &[u8],
    start_pos: usize,
) -> Result<Token, LexerError> {
    let len = bytes.len();
    let mut pos = start_pos + 1; // Skip opening '{'
    let mut brace_count = 1;

    while pos < len {
        match bytes[pos] {
            b'{' => brace_count += 1,
            b'}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    lexer.bump(pos - start_pos + 1);
                    return Ok(Token::FormatSpecifier);
                }
            }
            _ => {}
        }
        pos += 1;
    }

    Err(LexerError::InvalidFormatSpec)
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Herring, Debug, PartialEq, Copy, Clone)]
#[herring(initial = emit_indent_dedent)]
#[herring(extras = Context)]
#[herring(error = LexerError)]
#[herring(subpattern stringprefix = "r|u|ur|R|U|UR|Ur|uR|b|B|br|Br|bR|BR")]
#[herring(subpattern fstringprefix = "f|F|fr|Fr|fR|FR|rF|Rf")]
#[herring(subpattern exponent = "[eE][+-]?[0-9]+")]
#[herring(subpattern pointfloat = r"[0-9]*\.[0-9]+|[0-9]+\.")]
#[herring(subpattern floatnumber = "(?&pointfloat)(?&exponent)?|[0-9]+(?&exponent)")]
pub enum Token {
    EOF,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("=")]
    Eq,
    #[token("@")]
    At,
    #[token("&")]
    And,
    #[token(".")]
    Dot,
    #[token("(", lpar)]
    LPar,
    #[token(")", rpar)]
    RPar,
    #[token("*")]
    Star,
    #[token("+")]
    Plus,
    #[token(";")]
    Semi,
    #[token("<<")]
    Lt2,
    #[token("==")]
    Eq2,
    #[token(">>")]
    Gt2,
    #[token("|")]
    Pipe,
    #[token(",")]
    Comma,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,
    #[token(":", parse_colon)]
    Colon,
    #[token("<=")]
    LtEq,
    #[token("<>")]
    LtGt,
    #[token(">=")]
    GtEq,
    #[token("[", lbrak)]
    LBrak,
    #[token("]", rbrak)]
    RBrak,
    #[token("^")]
    Caret,
    #[token("`")]
    BTick,
    #[token("~")]
    Tilde,
    #[token("&=")]
    AndEq,
    #[token("**")]
    Star2,
    #[token("{", lbrace)]
    LBrace,
    #[token("}", rbrace)]
    RBrace,
    #[token("!=")]
    ExclEq,
    #[token("%")]
    Percent,
    #[token("*=")]
    StarEq,
    #[token("+=")]
    PlusEq,
    #[token("//")]
    Slash2,
    #[token("<<=")]
    Lt2Eq,
    #[token(">>=")]
    Gt2Eq,
    #[token("|=")]
    PipeEq,
    #[token("-=")]
    MinusEq,
    #[token("/=")]
    SlashEq,
    #[token("^=")]
    CaretEq,
    #[token("**=")]
    Star2Eq,
    #[token("%=")]
    PercentEq,
    #[token("//=")]
    Slash2Eq,
    #[token("as")]
    As,
    #[token("if")]
    If,
    #[token("in")]
    In,
    #[token("is")]
    Is,
    #[token("or")]
    Or,
    #[token("and")]
    AndKw,
    #[token("def")]
    Def,
    #[token("del")]
    Del,
    #[token("for")]
    For,
    #[token("not")]
    Not,
    #[token("try")]
    Try,
    #[token("else")]
    Else,
    #[token("exec")]
    Exec,
    #[token("from")]
    From,
    #[token("pass")]
    Pass,
    #[token("elif")]
    Elif,
    #[token("with")]
    With,
    #[token("break")]
    Break,
    #[token("class")]
    Class,
    #[token("print")]
    Print,
    #[token("raise")]
    Raise,
    #[token("while")]
    While,
    #[token("assert")]
    Assert,
    #[token("except")]
    Except,
    #[token("global")]
    Global,
    #[token("import")]
    Import,
    #[token("lambda")]
    Lambda,
    #[token("return")]
    Return,
    #[token("finally")]
    Finally,
    #[token("continue")]
    Continue,
    #[token("yield")]
    Yield,
    #[regex("\r?\n[\t ]*", check_indent)]
    Newline,
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*")]
    Name,
    Dedent,
    Indent,
    #[regex("([1-9][0-9]*|0)[lL]?")]
    #[regex("(0[oO][0-7]+|0[0-7]+)[lL]?")]
    #[regex("0[xX][0-9a-fA-F]+[lL]?")]
    #[regex("0[bB][0-1]+[lL]?")]
    Int,
    #[regex(r"(?&floatnumber)")]
    Float,
    #[regex(r"(?&floatnumber)[jJ]")]
    #[regex(r"[0-9]+[jJ]")]
    Imaginary,
    #[regex("(?&stringprefix)?\"", parse_short_double_string)]
    #[regex("(?&stringprefix)?'", parse_short_single_string)]
    #[regex("(?&stringprefix)?\"\"\"", parse_long_double_string)]
    #[regex("(?&stringprefix)?'''", parse_long_single_string)]
    String,

    // exclusively for python3
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[regex("![ars]")]
    ConversionSpecifier,
    FormatSpecifier,
    #[regex("(?&fstringprefix)\"", |lexer| {
        lexer.extras.strings_stack.push(StringKind::ShortDouble);
        parse_short_double_fstring(lexer)
    })]
    #[regex("(?&fstringprefix)'", |lexer| {
        lexer.extras.strings_stack.push(StringKind::ShortSingle);
        parse_short_single_fstring(lexer)
    })]
    #[regex("(?&fstringprefix)\"\"\"", |lexer| {
        lexer.extras.strings_stack.push(StringKind::LongDouble);
        parse_long_double_fstring(lexer)
    })]
    #[regex("(?&fstringprefix)'''", |lexer| {
        lexer.extras.strings_stack.push(StringKind::LongSingle);
        parse_long_single_fstring(lexer)
    })]
    FString,

    #[regex(r"([\t ]|\\\r?\n)+", check_first_line_indent)]
    Whitespace,
    #[regex(r"#[^\n]*", check_first_line_indent)]
    Comment,
    Error,
}

pub fn tokenize(source: &str, diags: &mut Vec<Diagnostic>) -> (Vec<Token>, Vec<Span>) {
    let lexer = Token::lexer(source);
    let mut tokens = vec![];
    let mut spans = vec![];

    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => {
                tokens.push(token);
            }
            Err(err) => {
                tokens.push(match err {
                    LexerError::Invalid => Token::Error,
                    LexerError::UnterminatedString => Token::String,
                    LexerError::Indent => Token::Newline,
                    LexerError::InvalidFormatSpec => Token::FormatSpecifier,
                    LexerError::UnterminatedFormatSpec => Token::FormatSpecifier,
                });
                diags.push(err.into_diagnostic(span.clone()));
            }
        }
        spans.push(span);
    }
    (tokens, spans)
}
