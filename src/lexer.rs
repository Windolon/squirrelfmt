const NULL: u8 = 0;
const UPPER_A: u8 = 65;
const UPPER_Z: u8 = 90;
const UNDERSCORE: u8 = 95;
const LOWER_A: u8 = 97;
const LOWER_Z: u8 = 122;
const ZERO: u8 = 48;
const NINE: u8 = 57;
const EXCLAMATION: u8 = 33;
const EQUAL: u8 = 61;
const PERCENT: u8 = 37;
const AMPERSAND: u8 = 38;
const ASTERISK: u8 = 42;
const PLUS: u8 = 43;
const MINUS: u8 = 45;
const SLASH: u8 = 47;
const LESS_THAN: u8 = 60;
const GREATER_THAN: u8 = 62;
const CARET: u8 = 94;
const BAR: u8 = 124;
const TILDE: u8 = 126;

#[derive(Debug, PartialEq)]
struct Position {
    start_line: u32,
    end_line: u32,
    start_column: u32,
    end_column: u32,
}

impl Position {
    fn new(lines: (u32, u32), columns: (u32, u32)) -> Position {
        Self {
            start_line: lines.0,
            end_line: lines.1,
            start_column: columns.0,
            end_column: columns.1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    If,
    In,
    For,
    Try,
    Base,
    Case,
    Else,
    Enum,
    Null,
    This,
    True,
    Break,
    Catch,
    Class,
    Clone,
    Const,
    False,
    Local,
    Throw,
    While,
    Yield,
    Delete,
    Resume,
    Return,
    Static,
    Switch,
    Typeof,
    Default,
    Extends,
    Foreach,
    Rawcall,
    File,
    Line,
    Continue,
    Function,
    InstanceOf,
    Constructor,
    Identifier,
    Not,
    Neq,
    Mod,
    ModAssign,
    BitAnd,
    And,
    Mult,
    MultAssign,
    Add,
    Increment,
    AddAssign,
    Sub,
    Decrement,
    SubAssign,
    Div,
    DivAssign,
    Lt,
    Ins,
    BitLeft,
    Le,
    Spaceship,
    Assign,
    Eq,
    Gt,
    Ge,
    BitRight,
    UnsignedRight,
    BitXor,
    BitOr,
    Or,
    BitNot,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: String,
    position: Position,
}

impl Token {
    fn new(kind: TokenKind, value: String, position: Position) -> Self {
        Self {
            kind,
            value,
            position,
        }
    }
}

pub struct Lexer {
    source: Vec<u8>,
    index: usize,
    line: u32,
    column: u32,
    did_send_eof: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.bytes().collect(),
            index: 0,
            line: 1,
            column: 1,
            did_send_eof: false,
        }
    }

    pub fn next_token(&mut self) -> Option<Result<Token, LexerError>> {
        match self.current_byte() {
            NULL => self.eof(),
            UPPER_A..=UPPER_Z | LOWER_A..=LOWER_Z | UNDERSCORE => self.identifier_or_keyword(),
            EXCLAMATION => self.exclamation(),
            PERCENT => self.percent(),
            AMPERSAND => self.ampersand(),
            ASTERISK => self.asterisk(),
            PLUS => self.plus(),
            MINUS => self.minus(),
            SLASH => self.slash(),
            LESS_THAN => self.less_than(),
            EQUAL => self.equal(),
            GREATER_THAN => self.greater_than(),
            CARET => self.caret(),
            BAR => self.bar(),
            TILDE => self.tilde(),
            _ => todo!(),
        }
    }

    /// Returns a token of `kind`, having an empty `String` as its `value` and
    /// spanning only on the current line of the lexer.
    /// Column-wise, it spans from `start` to one column before the lexer, inclusive.
    fn token_on_line(&self, kind: TokenKind, start: u32) -> Token {
        self.token_on_line_with_value(kind, "", start)
    }

    /// Returns a token of `kind` and `value`, that spans only on the current line of the lexer.
    /// Column-wise, it spans from `start` to one column before the lexer, inclusive.
    fn token_on_line_with_value(&self, kind: TokenKind, value: &str, start: u32) -> Token {
        let position = Position::new((self.line, self.line), (start, self.column - 1));
        Token::new(kind, value.to_string(), position)
    }

    fn eof(&mut self) -> Option<Result<Token, LexerError>> {
        let line = self.line;
        let column = self.column;

        if self.did_send_eof {
            return None;
        }

        self.did_send_eof = true;

        if column == 1 {
            // If the Eof is on a new line on its own, its position should be at [<line>:1].
            let pos = Position::new((line, line), (column, column));
            Some(Ok(Token::new(TokenKind::Eof, "".to_string(), pos)))
        } else {
            // Otherwise, its position should be wherever the last character is at.
            Some(Ok(self.token_on_line(TokenKind::Eof, column - 1)))
        }
    }

    fn identifier_or_keyword(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        let index_start = self.index;

        while let UPPER_A..=UPPER_Z | LOWER_A..=LOWER_Z | ZERO..=NINE | UNDERSCORE =
            self.advance_char()
        {
            continue;
        }

        // NOTE: The loop above should have made sure that the range is in bounds and consists of
        // valid bytes only, making the Err arm of this match practically unreachable. However, I
        // don't think this code is really "waterproof" per se, so TODO: some form of error
        // handling should be done in the future.
        let value = str::from_utf8(&self.source[index_start..self.index]).unwrap();

        // This logic follows Inko's implementation.
        //
        // If we did a simple match against all keywords, each identifier
        // would need to be run through every match arm before exiting.
        let kind = match value.len() {
            2 => match value {
                "if" => TokenKind::If,
                "in" => TokenKind::In,
                _ => TokenKind::Identifier,
            },
            3 => match value {
                "for" => TokenKind::For,
                "try" => TokenKind::Try,
                _ => TokenKind::Identifier,
            },
            4 => match value {
                "base" => TokenKind::Base,
                "case" => TokenKind::Case,
                "else" => TokenKind::Else,
                "enum" => TokenKind::Enum,
                "null" => TokenKind::Null,
                "this" => TokenKind::This,
                "true" => TokenKind::True,
                _ => TokenKind::Identifier,
            },
            5 => match value {
                "break" => TokenKind::Break,
                "catch" => TokenKind::Catch,
                "class" => TokenKind::Class,
                "clone" => TokenKind::Clone,
                "const" => TokenKind::Const,
                "false" => TokenKind::False,
                "local" => TokenKind::Local,
                "throw" => TokenKind::Throw,
                "while" => TokenKind::While,
                "yield" => TokenKind::Yield,
                _ => TokenKind::Identifier,
            },
            6 => match value {
                "delete" => TokenKind::Delete,
                "resume" => TokenKind::Resume,
                "return" => TokenKind::Return,
                "static" => TokenKind::Static,
                "switch" => TokenKind::Switch,
                "typeof" => TokenKind::Typeof,
                _ => TokenKind::Identifier,
            },
            7 => match value {
                "default" => TokenKind::Default,
                "extends" => TokenKind::Extends,
                "foreach" => TokenKind::Foreach,
                "rawcall" => TokenKind::Rawcall,
                _ => TokenKind::Identifier,
            },
            8 => match value {
                "__FILE__" => TokenKind::File,
                "__LINE__" => TokenKind::Line,
                "continue" => TokenKind::Continue,
                "function" => TokenKind::Function,
                _ => TokenKind::Identifier,
            },
            10 => match value {
                "instanceof" => TokenKind::InstanceOf,
                _ => TokenKind::Identifier,
            },
            11 => match value {
                "constructor" => TokenKind::Constructor,
                _ => TokenKind::Identifier,
            },
            _ => TokenKind::Identifier,
        };

        if kind == TokenKind::Identifier {
            Some(Ok(self.token_on_line_with_value(kind, value, column_start)))
        } else {
            Some(Ok(self.token_on_line(kind, column_start)))
        }
    }

    fn exclamation(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        if self.advance_char() == EQUAL {
            self.advance_char();
            Some(Ok(self.token_on_line(TokenKind::Neq, column_start)))
        } else {
            Some(Ok(self.token_on_line(TokenKind::Not, column_start)))
        }
    }

    fn percent(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        if self.advance_char() == EQUAL {
            self.advance_char();
            Some(Ok(self.token_on_line(TokenKind::ModAssign, column_start)))
        } else {
            Some(Ok(self.token_on_line(TokenKind::Mod, column_start)))
        }
    }

    fn ampersand(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        if self.advance_char() == AMPERSAND {
            self.advance_char();
            Some(Ok(self.token_on_line(TokenKind::And, column_start)))
        } else {
            Some(Ok(self.token_on_line(TokenKind::BitAnd, column_start)))
        }
    }

    fn asterisk(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        if self.advance_char() == EQUAL {
            self.advance_char();
            Some(Ok(self.token_on_line(TokenKind::MultAssign, column_start)))
        } else {
            Some(Ok(self.token_on_line(TokenKind::Mult, column_start)))
        }
    }

    fn plus(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            PLUS => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::Increment, column_start)))
            }
            EQUAL => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::AddAssign, column_start)))
            }
            _ => Some(Ok(self.token_on_line(TokenKind::Add, column_start))),
        }
    }

    fn minus(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            MINUS => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::Decrement, column_start)))
            }
            EQUAL => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::SubAssign, column_start)))
            }
            _ => Some(Ok(self.token_on_line(TokenKind::Sub, column_start))),
        }
    }

    fn slash(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            EQUAL => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::DivAssign, column_start)))
            }
            // Comment.
            SLASH => todo!(),
            _ => Some(Ok(self.token_on_line(TokenKind::Div, column_start))),
        }
    }

    fn less_than(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            MINUS => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::Ins, column_start)))
            }
            LESS_THAN => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::BitLeft, column_start)))
            }
            EQUAL => match self.advance_char() {
                GREATER_THAN => {
                    self.advance_char();
                    Some(Ok(self.token_on_line(TokenKind::Spaceship, column_start)))
                }
                _ => Some(Ok(self.token_on_line(TokenKind::Le, column_start))),
            },
            _ => Some(Ok(self.token_on_line(TokenKind::Lt, column_start))),
        }
    }

    fn equal(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        if self.advance_char() == EQUAL {
            self.advance_char();
            Some(Ok(self.token_on_line(TokenKind::Eq, column_start)))
        } else {
            Some(Ok(self.token_on_line(TokenKind::Assign, column_start)))
        }
    }

    fn greater_than(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            EQUAL => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::Ge, column_start)))
            }
            GREATER_THAN => match self.advance_char() {
                GREATER_THAN => {
                    self.advance_char();
                    Some(Ok(
                        self.token_on_line(TokenKind::UnsignedRight, column_start)
                    ))
                }
                _ => Some(Ok(self.token_on_line(TokenKind::BitRight, column_start))),
            },
            _ => Some(Ok(self.token_on_line(TokenKind::Gt, column_start))),
        }
    }

    fn caret(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        self.advance_char();
        Some(Ok(self.token_on_line(TokenKind::BitXor, column_start)))
    }

    fn bar(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        if self.advance_char() == BAR {
            self.advance_char();
            Some(Ok(self.token_on_line(TokenKind::Or, column_start)))
        } else {
            Some(Ok(self.token_on_line(TokenKind::BitOr, column_start)))
        }
    }

    fn tilde(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        self.advance_char();
        Some(Ok(self.token_on_line(TokenKind::BitNot, column_start)))
    }

    fn current_byte(&self) -> u8 {
        match self.source.get(self.index) {
            Some(&n) => n,
            None => NULL,
        }
    }

    fn peek_byte(&self) -> u8 {
        match self.source.get(self.index + 1) {
            Some(&n) => n,
            None => NULL,
        }
    }

    // Only call this when you are sure that the current "string environment"
    // doesn't contain any unicode symbols, otherwise the column logic will break.
    // If working in such an environment, you should handle the logic manually.
    fn advance_char(&mut self) -> u8 {
        self.index += 1;
        self.column += 1;
        self.current_byte()
    }

    fn advance_line(&mut self) {
        self.index += 1;
        self.line += 1;
        self.column = 1;
    }
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedSymbol,
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    fn tok_wrapped(
        kind: TokenKind,
        value: &str,
        lines: (u32, u32),
        columns: (u32, u32),
    ) -> Option<Result<Token, LexerError>> {
        let pos = Position::new(lines, columns);
        Some(Ok(Token::new(kind, value.to_string(), pos)))
    }

    fn tok_wrapped_with_next(
        kind: TokenKind,
        value: &str,
        lines: (u32, u32),
        columns: (u32, u32),
    ) -> Vec<Option<Result<Token, LexerError>>> {
        vec![
            tok_wrapped(kind, value, lines, columns),
            tok_wrapped(Eof, "", lines, (columns.1, columns.1)),
        ]
    }

    fn tok_from_with_next(source: &str) -> Vec<Option<Result<Token, LexerError>>> {
        let mut lexer = Lexer::new(source);
        let first_tok = lexer.next_token();
        vec![first_tok, lexer.next_token()]
    }

    #[test]
    fn eof_empty_none() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), tok_wrapped(Eof, "", (1, 1), (1, 1)));
        assert_eq!(lexer.next_token(), None);
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn eof_non_empty_line() {
        let mut lexer = Lexer::new("if");
        lexer.next_token();
        assert_eq!(lexer.next_token(), tok_wrapped(Eof, "", (1, 1), (2, 2)));
    }

    #[test]
    #[rustfmt::skip]
    fn keywords() {
        assert_eq!(tok_from_with_next("if"), tok_wrapped_with_next(If, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("in"), tok_wrapped_with_next(In, "", (1, 1), (1, 2)));

        assert_eq!(tok_from_with_next("for"), tok_wrapped_with_next(For, "", (1, 1), (1, 3)));
        assert_eq!(tok_from_with_next("try"), tok_wrapped_with_next(Try, "", (1, 1), (1, 3)));

        assert_eq!(tok_from_with_next("base"), tok_wrapped_with_next(Base, "", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("case"), tok_wrapped_with_next(Case, "", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("else"), tok_wrapped_with_next(Else, "", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("enum"), tok_wrapped_with_next(Enum, "", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("null"), tok_wrapped_with_next(Null, "", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("this"), tok_wrapped_with_next(This, "", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("true"), tok_wrapped_with_next(True, "", (1, 1), (1, 4)));

        assert_eq!(tok_from_with_next("break"), tok_wrapped_with_next(Break, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("catch"), tok_wrapped_with_next(Catch, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("class"), tok_wrapped_with_next(Class, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("clone"), tok_wrapped_with_next(Clone, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("const"), tok_wrapped_with_next(Const, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("false"), tok_wrapped_with_next(False, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("local"), tok_wrapped_with_next(Local, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("throw"), tok_wrapped_with_next(Throw, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("while"), tok_wrapped_with_next(While, "", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("yield"), tok_wrapped_with_next(Yield, "", (1, 1), (1, 5)));

        assert_eq!(tok_from_with_next("delete"), tok_wrapped_with_next(Delete, "", (1, 1), (1, 6)));
        assert_eq!(tok_from_with_next("resume"), tok_wrapped_with_next(Resume, "", (1, 1), (1, 6)));
        assert_eq!(tok_from_with_next("return"), tok_wrapped_with_next(Return, "", (1, 1), (1, 6)));
        assert_eq!(tok_from_with_next("static"), tok_wrapped_with_next(Static, "", (1, 1), (1, 6)));
        assert_eq!(tok_from_with_next("switch"), tok_wrapped_with_next(Switch, "", (1, 1), (1, 6)));
        assert_eq!(tok_from_with_next("typeof"), tok_wrapped_with_next(Typeof, "", (1, 1), (1, 6)));

        assert_eq!(tok_from_with_next("default"), tok_wrapped_with_next(Default, "", (1, 1), (1, 7)));
        assert_eq!(tok_from_with_next("extends"), tok_wrapped_with_next(Extends, "", (1, 1), (1, 7)));
        assert_eq!(tok_from_with_next("foreach"), tok_wrapped_with_next(Foreach, "", (1, 1), (1, 7)));
        assert_eq!(tok_from_with_next("rawcall"), tok_wrapped_with_next(Rawcall, "", (1, 1), (1, 7)));

        assert_eq!(tok_from_with_next("__FILE__"), tok_wrapped_with_next(File, "", (1, 1), (1, 8)));
        assert_eq!(tok_from_with_next("__LINE__"), tok_wrapped_with_next(Line, "", (1, 1), (1, 8)));
        assert_eq!(tok_from_with_next("continue"), tok_wrapped_with_next(Continue, "", (1, 1), (1, 8)));
        assert_eq!(tok_from_with_next("function"), tok_wrapped_with_next(Function, "", (1, 1), (1, 8)));

        assert_eq!(tok_from_with_next("instanceof"), tok_wrapped_with_next(InstanceOf, "", (1, 1), (1, 10)));
        assert_eq!(tok_from_with_next("constructor"), tok_wrapped_with_next(Constructor, "", (1, 1), (1, 11)));
    }

    #[test]
    #[rustfmt::skip]
    fn identifiers() {
        assert_eq!(tok_from_with_next("_"), tok_wrapped_with_next(Identifier, "_", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("f"), tok_wrapped_with_next(Identifier, "f", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("F"), tok_wrapped_with_next(Identifier, "F", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("f1"), tok_wrapped_with_next(Identifier, "f1", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("_1"), tok_wrapped_with_next(Identifier, "_1", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("__"), tok_wrapped_with_next(Identifier, "__", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("foo"), tok_wrapped_with_next(Identifier, "foo", (1, 1), (1, 3)));
        assert_eq!(tok_from_with_next("__fo"), tok_wrapped_with_next(Identifier, "__fo", (1, 1), (1, 4)));
        assert_eq!(tok_from_with_next("__2fo"), tok_wrapped_with_next(Identifier, "__2fo", (1, 1), (1, 5)));
        assert_eq!(tok_from_with_next("FooBar"), tok_wrapped_with_next(Identifier, "FooBar", (1, 1), (1, 6)));
        assert_eq!(tok_from_with_next("fOo2BaR"), tok_wrapped_with_next(Identifier, "fOo2BaR", (1, 1), (1, 7)));
        assert_eq!(tok_from_with_next("HALFLIFE"), tok_wrapped_with_next(Identifier, "HALFLIFE", (1, 1), (1, 8)));
        assert_eq!(tok_from_with_next("fooBarBaz"), tok_wrapped_with_next(Identifier, "fooBarBaz", (1, 1), (1, 9)));
        assert_eq!(tok_from_with_next("portal_two"), tok_wrapped_with_next(Identifier, "portal_two", (1, 1), (1, 10)));
        assert_eq!(tok_from_with_next("__DumpScope"), tok_wrapped_with_next(Identifier, "__DumpScope", (1, 1), (1, 11)));
        assert_eq!(tok_from_with_next("__0foobarbaz"), tok_wrapped_with_next(Identifier, "__0foobarbaz", (1, 1), (1, 12)));
        assert_eq!(tok_from_with_next("___0123456789"), tok_wrapped_with_next(Identifier, "___0123456789", (1, 1), (1, 13)));
    }

    #[test]
    #[rustfmt::skip]
    fn operators() {
        assert_eq!(tok_from_with_next("!"), tok_wrapped_with_next(Not, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("!="), tok_wrapped_with_next(Neq, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("%"), tok_wrapped_with_next(Mod, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("%="), tok_wrapped_with_next(ModAssign, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("&"), tok_wrapped_with_next(BitAnd, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("&&"), tok_wrapped_with_next(And, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("*"), tok_wrapped_with_next(Mult, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("*="), tok_wrapped_with_next(MultAssign, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("+"), tok_wrapped_with_next(Add, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("++"), tok_wrapped_with_next(Increment, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("+="), tok_wrapped_with_next(AddAssign, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("-"), tok_wrapped_with_next(Sub, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("--"), tok_wrapped_with_next(Decrement, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("-="), tok_wrapped_with_next(SubAssign, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("/"), tok_wrapped_with_next(Div, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("/="), tok_wrapped_with_next(DivAssign, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("<"), tok_wrapped_with_next(Lt, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("<-"), tok_wrapped_with_next(Ins, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("<<"), tok_wrapped_with_next(BitLeft, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("<="), tok_wrapped_with_next(Le, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("<=>"), tok_wrapped_with_next(Spaceship, "", (1, 1), (1, 3)));
        assert_eq!(tok_from_with_next("="), tok_wrapped_with_next(Assign, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("=="), tok_wrapped_with_next(Eq, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next(">"), tok_wrapped_with_next(Gt, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next(">="), tok_wrapped_with_next(Ge, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next(">>"), tok_wrapped_with_next(BitRight, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next(">>>"), tok_wrapped_with_next(UnsignedRight, "", (1, 1), (1, 3)));
        assert_eq!(tok_from_with_next("^"), tok_wrapped_with_next(BitXor, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("|"), tok_wrapped_with_next(BitOr, "", (1, 1), (1, 1)));
        assert_eq!(tok_from_with_next("||"), tok_wrapped_with_next(Or, "", (1, 1), (1, 2)));
        assert_eq!(tok_from_with_next("~"), tok_wrapped_with_next(BitNot, "", (1, 1), (1, 1)));
    }
}
