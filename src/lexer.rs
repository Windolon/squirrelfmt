const NULL: u8 = 0;
const EXCLAMATION: u8 = 33;
const PERCENT: u8 = 37;
const AMPERSAND: u8 = 38;
const APOSTROPHE: u8 = 39;
const PAREN_OPEN: u8 = 40;
const PAREN_CLOSE: u8 = 41;
const ASTERISK: u8 = 42;
const PLUS: u8 = 43;
const COMMA: u8 = 44;
const MINUS: u8 = 45;
const DOT: u8 = 46;
const SLASH: u8 = 47;
const ZERO: u8 = 48;
const NINE: u8 = 57;
const COLON: u8 = 58;
const SEMICOLON: u8 = 59;
const LESS_THAN: u8 = 60;
const EQUAL: u8 = 61;
const GREATER_THAN: u8 = 62;
const UPPER_A: u8 = 65;
const UPPER_Z: u8 = 90;
const SQUARE_OPEN: u8 = 91;
const BACKSLASH: u8 = 92;
const SQUARE_CLOSE: u8 = 93;
const CARET: u8 = 94;
const UNDERSCORE: u8 = 95;
const LOWER_A: u8 = 97;
const LOWER_Z: u8 = 122;
const BRACE_OPEN: u8 = 123;
const BAR: u8 = 124;
const BRACE_CLOSE: u8 = 125;
const TILDE: u8 = 126;

/// Represents a symbol's exact location in source code.
// TODO: How does unicode chars affect this counter?
#[derive(Debug, PartialEq)]
pub struct Position {
    line: u32,
    column: u32,
}

impl Position {
    /// Returns the line number of this position.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column number of this position.
    pub fn column(&self) -> u32 {
        self.column
    }

    fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// The kind of [`Token`].
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// The `+` operator.
    Add,
    /// The `+=` operator.
    AddAssign,
    /// The `&&` operator.
    And,
    /// The `=` operator.
    Assign,
    /// The `base` keyword.
    Base,
    /// The `&` operator.
    BitAnd,
    /// The `<<` operator.
    BitLeft,
    /// The `~` operator.
    BitNot,
    /// The `|` operator.
    BitOr,
    /// The `>>` operator.
    BitRight,
    /// The `^` operator.
    BitXor,
    /// A closing brace / curly bracket `}`.
    BraceClose,
    /// An opening brace / curly bracket `{`.
    BraceOpen,
    /// The `break` keyword.
    Break,
    /// The `case` keyword.
    Case,
    /// The `catch` keyword.
    Catch,
    /// A `char`-like literal, e.g. `'a'`.
    Char,
    /// The `class` keyword.
    Class,
    /// The `clone` keyword.
    Clone,
    /// A colon `:`.
    Colon,
    /// The `,` operator, or the separator used in function argument lists, tables and arrays.
    Comma,
    /// The `const` keyword.
    Const,
    /// The `constructor` keyword.
    Constructor,
    /// The `continue` keyword.
    Continue,
    /// The `--` operator.
    Decrement,
    /// The `default` keyword.
    Default,
    /// The `delete` keyword.
    Delete,
    /// The `/` operator.
    Div,
    /// The `/=` operator.
    DivAssign,
    /// A dot `.`.
    Dot,
    /// An ellipsis `...`, seen in function argument lists.
    Ellipsis,
    /// The `else` keyword.
    Else,
    /// The `enum` keyword.
    Enum,
    /// Signifies the end of file.
    Eof,
    /// The `==` operator.
    Eq,
    /// The `extends` keyword.
    Extends,
    /// The `false` keyword.
    False,
    /// The `__FILE__` keyword.
    File,
    /// The `for` keyword.
    For,
    /// The `foreach` keyword.
    Foreach,
    /// The `function` keyword.
    Function,
    /// The `>=` operator.
    Ge,
    /// The `>` operator.
    Gt,
    /// An identifier.
    Identifier,
    /// The `if` keyword.
    If,
    /// The `in` keyword.
    In,
    /// The `++` operator.
    Increment,
    /// The `<-` operator.
    Ins,
    /// The `instanceof` keyword.
    InstanceOf,
    /// The `<=` operator.
    Le,
    /// The `__LINE__` keyword.
    Line,
    /// The `local` keyword.
    Local,
    /// The `<` operator.
    Lt,
    /// The `%` operator.
    Mod,
    /// The `%=` operator.
    ModAssign,
    /// The `*` operator.
    Mult,
    /// The `*=` operator.
    MultAssign,
    /// The `!=` operator.
    Neq,
    /// The `!` operator.
    Not,
    /// The `null` keyword.
    Null,
    /// The `||` operator.
    Or,
    /// A closing parenthesis `)`.
    ParenClose,
    /// An opening parenthesis `(`.
    ParenOpen,
    /// The `rawcall` keyword.
    Rawcall,
    /// The `resume` keyword.
    Resume,
    /// The `return` keyword.
    Return,
    /// A scope resolution symbol `::`.
    // TODO: Is this an operator?
    ScopeRes,
    /// A semicolon `;`.
    Semicolon,
    /// The `<=>` operator. Also known as the three-way comparison operator.
    Spaceship,
    /// A closing square bracket `]`.
    SquareClose,
    /// An opening square bracket `[`.
    SquareOpen,
    /// The `static` keyword.
    Static,
    /// The `-` operator.
    Sub,
    /// The `-=` operator.
    SubAssign,
    /// The `switch` keyword.
    Switch,
    /// The `this` keyword.
    This,
    /// The `throw` keyword.
    Throw,
    /// The `true` keyword.
    True,
    /// The `try` keyword.
    Try,
    /// The `typeof` keyword.
    Typeof,
    /// The `>>>` operator.
    UnsignedRight,
    /// The `while` keyword.
    While,
    /// The `yield` keyword.
    Yield,
}

/// A token consisting of its [`TokenKind`], value if any, and its starting and ending position in
/// source code.
#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: String,
    start_position: Position,
    end_position: Position,
}

impl Token {
    fn new(kind: TokenKind, value: String, start: (u32, u32), end: (u32, u32)) -> Self {
        Self {
            kind,
            value,
            start_position: Position::new(start.0, start.1),
            end_position: Position::new(end.0, end.1),
        }
    }
}

/// The main lexing object that takes in a source string and returns a stream of tokens.
pub struct Lexer {
    source: Vec<u8>,
    index: usize,
    line: u32,
    column: u32,
    did_send_eof: bool,
}

impl Lexer {
    /// Creates a new Lexer from the input source string.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.bytes().collect(),
            index: 0,
            line: 1,
            column: 1,
            did_send_eof: false,
        }
    }

    /// Returns the next token.
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
            COMMA => self.comma(),
            PAREN_OPEN | PAREN_CLOSE => self.paren(),
            SQUARE_OPEN | SQUARE_CLOSE => self.square(),
            BRACE_OPEN | BRACE_CLOSE => self.brace(),
            DOT => self.dot(),
            COLON => self.colon(),
            SEMICOLON => self.semicolon(),
            APOSTROPHE => self.char(),
            _ => {
                self.terminate();
                Some(Err(LexerError::new(
                    LexerErrorKind::UnexpectedSymbol,
                    self.line,
                    self.column,
                )))
            }
        }
    }

    // Call this when new token ends precisely at one column before the lexer
    fn token_on_line(&self, kind: TokenKind, start: u32) -> Token {
        self.token_on_line_with_value(kind, "", start)
    }

    // Call this when new token ends precisely at one column before the lexer
    fn token_on_line_with_value(&self, kind: TokenKind, value: &str, start: u32) -> Token {
        Token::new(
            kind,
            value.to_string(),
            (self.line, start),
            (self.line, self.column - 1),
        )
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
            Some(Ok(Token::new(
                TokenKind::Eof,
                "".to_string(),
                (line, column),
                (line, column),
            )))
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

    fn comma(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        self.advance_char();
        Some(Ok(self.token_on_line(TokenKind::Comma, column_start)))
    }

    fn paren(&mut self) -> Option<Result<Token, LexerError>> {
        // TODO: This is really bad code and doesn't look right, improve this and
        // other derived methods?
        let column_start = self.column;
        let current_byte = self.current_byte();
        self.advance_char();
        let token = match current_byte {
            PAREN_OPEN => self.token_on_line(TokenKind::ParenOpen, column_start),
            PAREN_CLOSE => self.token_on_line(TokenKind::ParenClose, column_start),
            _ => unreachable!(),
        };
        Some(Ok(token))
    }

    fn square(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        let current_byte = self.current_byte();
        self.advance_char();
        let token = match current_byte {
            SQUARE_OPEN => self.token_on_line(TokenKind::SquareOpen, column_start),
            SQUARE_CLOSE => self.token_on_line(TokenKind::SquareClose, column_start),
            _ => unreachable!(),
        };
        Some(Ok(token))
    }

    fn brace(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        let current_byte = self.current_byte();
        self.advance_char();
        let token = match current_byte {
            BRACE_OPEN => self.token_on_line(TokenKind::BraceOpen, column_start),
            BRACE_CLOSE => self.token_on_line(TokenKind::BraceClose, column_start),
            _ => unreachable!(),
        };
        Some(Ok(token))
    }

    fn dot(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            DOT => match self.advance_char() {
                DOT => {
                    self.advance_char();
                    Some(Ok(self.token_on_line(TokenKind::Ellipsis, column_start)))
                }
                // ".." is invalid and should return an error
                _ => todo!(),
            },
            _ => Some(Ok(self.token_on_line(TokenKind::Dot, column_start))),
        }
    }

    fn colon(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            COLON => {
                self.advance_char();
                Some(Ok(self.token_on_line(TokenKind::ScopeRes, column_start)))
            }
            _ => Some(Ok(self.token_on_line(TokenKind::Colon, column_start))),
        }
    }

    fn semicolon(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        self.advance_char();
        Some(Ok(self.token_on_line(TokenKind::Semicolon, column_start)))
    }

    fn char(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            // ''
            APOSTROPHE => {
                self.terminate();
                Some(Err(LexerError::new(
                    LexerErrorKind::EmptyChar,
                    self.line,
                    self.column,
                )))
            }
            // '\<escape>
            BACKSLASH => todo!(),
            // '<ascii>
            0..=127 => match self.advance_char() {
                // '<ascii>': correct char
                APOSTROPHE => {
                    let index_start = self.index - 1;
                    self.advance_char();
                    let value = str::from_utf8(&self.source[index_start..self.index - 1]).unwrap();
                    Some(Ok(self.token_on_line_with_value(
                        TokenKind::Char,
                        value,
                        column_start,
                    )))
                }
                // '<ascii><other>: char is too long
                _ => {
                    self.terminate();
                    Some(Err(LexerError::new(
                        LexerErrorKind::CharTooLong,
                        self.line,
                        self.column,
                    )))
                }
            },
            // '<non-ascii>
            _ => {
                self.terminate();
                Some(Err(LexerError::new(
                    LexerErrorKind::CharOutOfBounds,
                    self.line,
                    self.column,
                )))
            }
        }
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

    fn terminate(&mut self) {
        self.did_send_eof = true;
        self.index = self.source.len();
    }
}

/// The kind of [`LexerError`].
#[derive(Debug, PartialEq)]
pub enum LexerErrorKind {
    /// A symbol outside of the ASCII range (0 to 127 inclusive) was encountered in a `char`-like
    /// literal.
    CharOutOfBounds,
    /// More than one symbol were encountered in a `char`-like literal.
    CharTooLong,
    /// An empty `char`-like literal was encountered, i.e. `''`.
    EmptyChar,
    /// An unexpected symbol was encountered outside of comments or strings.
    UnexpectedSymbol,
}

/// An object returned by the [`Lexer`] when it encounters an error.
#[derive(Debug, PartialEq)]
pub struct LexerError {
    /// The kind of error encountered.
    pub kind: LexerErrorKind,
    /// The position of this error in source code.
    pub position: Position,
}

impl LexerError {
    fn new(kind: LexerErrorKind, line: u32, column: u32) -> Self {
        Self {
            kind,
            position: Position::new(line, column),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LexerErrorKind::*;
    use TokenKind::*;

    fn token(
        kind: TokenKind,
        value: &str,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Option<Result<Token, LexerError>> {
        Some(Ok(Token::new(kind, value.to_string(), start, end)))
    }

    fn token_withnext(
        kind: TokenKind,
        value: &str,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Vec<Option<Result<Token, LexerError>>> {
        vec![token(kind, value, start, end), token(Eof, "", end, end)]
    }

    fn error_withnext(
        kind: LexerErrorKind,
        line: u32,
        column: u32,
    ) -> Vec<Option<Result<Token, LexerError>>> {
        vec![Some(Err(LexerError::new(kind, line, column))), None]
    }

    fn token_from_withnext(source: &str) -> Vec<Option<Result<Token, LexerError>>> {
        let mut lexer = Lexer::new(source);
        let first_tok = lexer.next_token();
        vec![first_tok, lexer.next_token()]
    }

    #[test]
    fn eof_empty_none() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), token(Eof, "", (1, 1), (1, 1)));
        assert_eq!(lexer.next_token(), None);
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn eof_non_empty_line() {
        let mut lexer = Lexer::new("if");
        lexer.next_token();
        assert_eq!(lexer.next_token(), token(Eof, "", (1, 2), (1, 2)));
    }

    #[test]
    #[rustfmt::skip]
    fn keywords() {
        assert_eq!(token_from_withnext("if"), token_withnext(If, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("in"), token_withnext(In, "", (1, 1), (1, 2)));

        assert_eq!(token_from_withnext("for"), token_withnext(For, "", (1, 1), (1, 3)));
        assert_eq!(token_from_withnext("try"), token_withnext(Try, "", (1, 1), (1, 3)));

        assert_eq!(token_from_withnext("base"), token_withnext(Base, "", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("case"), token_withnext(Case, "", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("else"), token_withnext(Else, "", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("enum"), token_withnext(Enum, "", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("null"), token_withnext(Null, "", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("this"), token_withnext(This, "", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("true"), token_withnext(True, "", (1, 1), (1, 4)));

        assert_eq!(token_from_withnext("break"), token_withnext(Break, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("catch"), token_withnext(Catch, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("class"), token_withnext(Class, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("clone"), token_withnext(Clone, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("const"), token_withnext(Const, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("false"), token_withnext(False, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("local"), token_withnext(Local, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("throw"), token_withnext(Throw, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("while"), token_withnext(While, "", (1, 1), (1, 5)));
        assert_eq!(token_from_withnext("yield"), token_withnext(Yield, "", (1, 1), (1, 5)));

        assert_eq!(token_from_withnext("delete"), token_withnext(Delete, "", (1, 1), (1, 6)));
        assert_eq!(token_from_withnext("resume"), token_withnext(Resume, "", (1, 1), (1, 6)));
        assert_eq!(token_from_withnext("return"), token_withnext(Return, "", (1, 1), (1, 6)));
        assert_eq!(token_from_withnext("static"), token_withnext(Static, "", (1, 1), (1, 6)));
        assert_eq!(token_from_withnext("switch"), token_withnext(Switch, "", (1, 1), (1, 6)));
        assert_eq!(token_from_withnext("typeof"), token_withnext(Typeof, "", (1, 1), (1, 6)));

        assert_eq!(token_from_withnext("default"), token_withnext(Default, "", (1, 1), (1, 7)));
        assert_eq!(token_from_withnext("extends"), token_withnext(Extends, "", (1, 1), (1, 7)));
        assert_eq!(token_from_withnext("foreach"), token_withnext(Foreach, "", (1, 1), (1, 7)));
        assert_eq!(token_from_withnext("rawcall"), token_withnext(Rawcall, "", (1, 1), (1, 7)));

        assert_eq!(token_from_withnext("__FILE__"), token_withnext(File, "", (1, 1), (1, 8)));
        assert_eq!(token_from_withnext("__LINE__"), token_withnext(Line, "", (1, 1), (1, 8)));
        assert_eq!(token_from_withnext("continue"), token_withnext(Continue, "", (1, 1), (1, 8)));
        assert_eq!(token_from_withnext("function"), token_withnext(Function, "", (1, 1), (1, 8)));

        assert_eq!(token_from_withnext("instanceof"), token_withnext(InstanceOf, "", (1, 1), (1, 10)));
        assert_eq!(token_from_withnext("constructor"), token_withnext(Constructor, "", (1, 1), (1, 11)));
    }

    #[test]
    #[rustfmt::skip]
    fn identifiers() {
        // unused variable
        assert_eq!(token_from_withnext("_"), token_withnext(Identifier, "_", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("f"), token_withnext(Identifier, "f", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("F"), token_withnext(Identifier, "F", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("f1"), token_withnext(Identifier, "f1", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("_1"), token_withnext(Identifier, "_1", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("__"), token_withnext(Identifier, "__", (1, 1), (1, 2)));
        // general variable
        assert_eq!(token_from_withnext("foo"), token_withnext(Identifier, "foo", (1, 1), (1, 3)));
        assert_eq!(token_from_withnext("__fo"), token_withnext(Identifier, "__fo", (1, 1), (1, 4)));
        assert_eq!(token_from_withnext("__2fo"), token_withnext(Identifier, "__2fo", (1, 1), (1, 5)));
        // PascalCase
        assert_eq!(token_from_withnext("FooBar"), token_withnext(Identifier, "FooBar", (1, 1), (1, 6)));
        assert_eq!(token_from_withnext("fOo2BaR"), token_withnext(Identifier, "fOo2BaR", (1, 1), (1, 7)));
        // camelCase
        assert_eq!(token_from_withnext("fooBarBa"), token_withnext(Identifier, "fooBarBa", (1, 1), (1, 8)));
        // SCREAMING_SNAKE_CASE
        assert_eq!(token_from_withnext("HALF_LIFE"), token_withnext(Identifier, "HALF_LIFE", (1, 1), (1, 9)));
        // snake_case
        assert_eq!(token_from_withnext("portal_two"), token_withnext(Identifier, "portal_two", (1, 1), (1, 10)));
        // a general script function beginning with "_"
        assert_eq!(token_from_withnext("__DumpScope"), token_withnext(Identifier, "__DumpScope", (1, 1), (1, 11)));
        assert_eq!(token_from_withnext("__0foobarbaz"), token_withnext(Identifier, "__0foobarbaz", (1, 1), (1, 12)));
        assert_eq!(token_from_withnext("___0123456789"), token_withnext(Identifier, "___0123456789", (1, 1), (1, 13)));
    }

    #[test]
    #[rustfmt::skip]
    fn operators() {
        assert_eq!(token_from_withnext("!"), token_withnext(Not, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("!="), token_withnext(Neq, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("%"), token_withnext(Mod, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("%="), token_withnext(ModAssign, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("&"), token_withnext(BitAnd, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("&&"), token_withnext(And, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("*"), token_withnext(Mult, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("*="), token_withnext(MultAssign, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("+"), token_withnext(Add, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("++"), token_withnext(Increment, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("+="), token_withnext(AddAssign, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("-"), token_withnext(Sub, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("--"), token_withnext(Decrement, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("-="), token_withnext(SubAssign, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("/"), token_withnext(Div, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("/="), token_withnext(DivAssign, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("<"), token_withnext(Lt, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("<-"), token_withnext(Ins, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("<<"), token_withnext(BitLeft, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("<="), token_withnext(Le, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("<=>"), token_withnext(Spaceship, "", (1, 1), (1, 3)));
        assert_eq!(token_from_withnext("="), token_withnext(Assign, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("=="), token_withnext(Eq, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext(">"), token_withnext(Gt, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext(">="), token_withnext(Ge, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext(">>"), token_withnext(BitRight, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext(">>>"), token_withnext(UnsignedRight, "", (1, 1), (1, 3)));
        assert_eq!(token_from_withnext("^"), token_withnext(BitXor, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("|"), token_withnext(BitOr, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("||"), token_withnext(Or, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext("~"), token_withnext(BitNot, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext(","), token_withnext(Comma, "", (1, 1), (1, 1)));
    }

    #[test]
    #[rustfmt::skip]
    fn misc_tokens() {
        assert_eq!(token_from_withnext("("), token_withnext(ParenOpen, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext(")"), token_withnext(ParenClose, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("["), token_withnext(SquareOpen, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("]"), token_withnext(SquareClose, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("{"), token_withnext(BraceOpen, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("}"), token_withnext(BraceClose, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("."), token_withnext(Dot, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("..."), token_withnext(Ellipsis, "", (1, 1), (1, 3)));
        assert_eq!(token_from_withnext(":"), token_withnext(Colon, "", (1, 1), (1, 1)));
        assert_eq!(token_from_withnext("::"), token_withnext(ScopeRes, "", (1, 1), (1, 2)));
        assert_eq!(token_from_withnext(";"), token_withnext(Semicolon, "", (1, 1), (1, 1)));
    }

    #[test]
    fn unexpected_symbol() {
        let mut lexer = Lexer::new("ändern");
        assert_eq!(
            lexer.next_token(),
            Some(Err(LexerError::new(UnexpectedSymbol, 1, 1)))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn char() {
        assert_eq!(
            token_from_withnext("'f'"),
            token_withnext(Char, "f", (1, 1), (1, 3))
        );
    }

    #[test]
    fn char_oob() {
        assert_eq!(
            token_from_withnext("'Ü'"),
            error_withnext(CharOutOfBounds, 1, 2)
        );
    }

    #[test]
    fn char_too_long() {
        assert_eq!(
            token_from_withnext("'xd'"),
            error_withnext(CharTooLong, 1, 3)
        );
    }

    #[test]
    fn char_empty() {
        assert_eq!(token_from_withnext("''"), error_withnext(EmptyChar, 1, 2));
    }
}
