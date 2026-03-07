use unicode_segmentation::UnicodeSegmentation;

const AMPERSAND: u8 = b'&';
const APOSTROPHE: u8 = b'\'';
const ASTERISK: u8 = b'*';
const AT: u8 = b'@';
const BACKSLASH: u8 = b'\\';
const BAR: u8 = b'|';
const BRACE_CLOSE: u8 = b'}';
const BRACE_OPEN: u8 = b'{';
const CARET: u8 = b'^';
const COLON: u8 = b':';
const COMMA: u8 = b',';
const DOT: u8 = b'.';
const EIGHT: u8 = b'8';
const EQUAL: u8 = b'=';
const EXCLAMATION: u8 = b'!';
const GREATER_THAN: u8 = b'>';
const HASH: u8 = b'#';
const LESS_THAN: u8 = b'<';
const LOWER_A: u8 = b'a';
const LOWER_E: u8 = b'e';
const LOWER_F: u8 = b'f';
const LOWER_X: u8 = b'x';
const LOWER_Z: u8 = b'z';
const MINUS: u8 = b'-';
const NEWLINE: u8 = b'\n';
const NINE: u8 = b'9';
const NULL: u8 = b'\0';
const PAREN_CLOSE: u8 = b')';
const PAREN_OPEN: u8 = b'(';
const PERCENT: u8 = b'%';
const PLUS: u8 = b'+';
const QUOTATION: u8 = b'"';
const SEMICOLON: u8 = b';';
const SEVEN: u8 = b'7';
const SLASH: u8 = b'/';
const SQUARE_CLOSE: u8 = b']';
const SQUARE_OPEN: u8 = b'[';
const TILDE: u8 = b'~';
const UNDERSCORE: u8 = b'_';
const UPPER_A: u8 = b'A';
const UPPER_E: u8 = b'E';
const UPPER_F: u8 = b'F';
const UPPER_X: u8 = b'X';
const UPPER_Z: u8 = b'Z';
const ZERO: u8 = b'0';

/// Represents a symbol's exact location in source code.
// TODO: How does unicode chars affect this counter?
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    line: u32,
    column: u32,
}

impl Position {
    /// Returns the line number of the position.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column number of the position.
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
    /// A C-styled comment, e.g. `// comment`.
    CComment,
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
    /// A floating point number.
    ///
    /// Due to the implementations of the Squirrel language, some scary-looking expressions are
    /// valid floats - they parse and compile successfully.
    ///
    /// # Examples
    ///
    /// - `1e5.125..e8...e-1..E+12.0e+10` is one token and resolves to `1e5`
    /// - `3.08.1599.0811e2e-15e-200.0.0.01` is one token and resolves to `3.08`
    ///
    /// However:
    ///
    /// - `4.5e+10-2` is three tokens and resolves to `44998`, because it is lexed as `4.5e+10`, `-`
    ///   and `2`.
    Float,
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
    /// A shell script-styled comment, e.g. `# comment`.
    ShellComment,
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
    /// An integer. Could be in base 8, 10 or 16.
    Integer,
    /// The `@` symbol signaling a lambda expression.
    Lambda,
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
    /// A multi-line comment.
    MultiLineComment,
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
    /// A string.
    String,
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
    /// A verbatim string.
    VerbatimString,
    /// The `while` keyword.
    While,
    /// The `yield` keyword.
    Yield,
}

/// A token consisting of its [`TokenKind`], value if any, and its starting and ending [`Position`]
/// in source code.
#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: String,
    start_position: Position,
    end_position: Position,
}

impl Token {
    /// Returns the kind of the token.
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    /// Returns the value of the token. Tokens may not always contain a value. In that case, an
    /// empty string slice is returned.
    ///
    /// A token tries to represent the source code as closely as possible. If parsing is done early,
    /// precision may be lost. Therefore, all tokens' values are stored as a string.
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /// Returns the starting position of the token.
    pub fn start(&self) -> Position {
        self.start_position.clone()
    }

    /// Returns the ending position of the token.
    pub fn end(&self) -> Position {
        self.end_position.clone()
    }

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
            source: source.as_bytes().to_owned(),
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
            QUOTATION => self.string(),
            AT => self.at(),
            HASH => self.comment(self.column),
            ZERO..=NINE => self.number(),
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

    fn error(&mut self, kind: LexerErrorKind) -> Option<Result<Token, LexerError>> {
        self.did_send_eof = true;
        self.index = self.source.len();
        Some(Err(LexerError::new(kind, self.line, self.column - 1)))
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
            SLASH => self.comment(column_start),
            ASTERISK => self.multi_line_comment(self.line, column_start),
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
            // '': empty
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
            // '<\n> or '<end>: unclosed
            NEWLINE | NULL => {
                self.terminate();
                Some(Err(LexerError::new(
                    LexerErrorKind::UnclosedChar,
                    self.line,
                    column_start,
                )))
            }
            // '<ascii>
            0..=127 => match self.advance_char() {
                // '<ascii>': correct
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
                // '<ascii><\n> or '<ascii><end>: unclosed
                NEWLINE | NULL => {
                    self.terminate();
                    Some(Err(LexerError::new(
                        LexerErrorKind::UnclosedChar,
                        self.line,
                        self.column - 1,
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
            // '<non-ascii>: oob
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

    fn string(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        let index_start = self.index + 1;

        loop {
            match self.advance_byte() {
                QUOTATION | NEWLINE | NULL => break,
                _ => continue,
            }
        }

        // self.index points at QUOTATION, NEWLINE or NULL
        let value = str::from_utf8(&self.source[index_start..self.index]).unwrap();
        let len = value.graphemes(true).count() as u32;

        match self.current_byte() {
            // Unclosed
            NEWLINE | NULL => {
                let column = column_start + len;
                self.terminate();
                Some(Err(LexerError::new(
                    LexerErrorKind::UnclosedString,
                    self.line,
                    column,
                )))
            }
            // Correct string
            QUOTATION => {
                self.column += len + 2;
                // Hmm, can't use advance_byte() here.
                self.index += 1;
                Some(Ok(self.token_on_line_with_value(
                    TokenKind::String,
                    value,
                    column_start,
                )))
            }
            _ => unreachable!(),
        }
    }

    fn at(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        match self.advance_char() {
            QUOTATION => self.verbatim_string(self.line, column_start),
            _ => Some(Ok(self.token_on_line(TokenKind::Lambda, column_start))),
        }
    }

    fn verbatim_string(
        &mut self,
        line_start: u32,
        column_start: u32,
    ) -> Option<Result<Token, LexerError>> {
        //                     @"
        // self.index points at ^
        let index_start = self.index + 1;
        let mut last_newline_index = 0;

        loop {
            match self.advance_byte() {
                NULL => break,
                NEWLINE => {
                    // NOTE: column count will be calculated later in this method
                    self.line += 1;
                    // A newline counts as one grapheme, we don't want to include that
                    last_newline_index = self.index + 1;
                    continue;
                }
                QUOTATION => {
                    match self.peek_byte() {
                        QUOTATION => {
                            //                                       ... ""hello"" ...
                            // If we don't advance here, the next match will see ^
                            // and think that the verbatim string has ended
                            self.advance_byte();
                            continue;
                        }
                        // string ends
                        _ => break,
                    }
                }
                _ => continue,
            }
        }

        // self.index points at NULL or QUOTATION
        match self.current_byte() {
            // unclosed
            NULL => {
                if self.line != line_start {
                    let mut column = str::from_utf8(
                        self.source
                            .get(last_newline_index..self.index)
                            .unwrap_or(&[]),
                    )
                    .unwrap()
                    .graphemes(true)
                    .count() as u32;

                    if column == 0 {
                        column = 1;
                    }

                    self.terminate();
                    Some(Err(LexerError::new(
                        LexerErrorKind::UnclosedVerbatimString,
                        self.line,
                        column,
                    )))
                } else {
                    let columns =
                        str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[]))
                            .unwrap()
                            .graphemes(true)
                            .count() as u32;

                    self.terminate();
                    Some(Err(LexerError::new(
                        LexerErrorKind::UnclosedVerbatimString,
                        self.line,
                        //                        @"...
                        // column_start points at ^, we add one to advance it to the ",
                        // then advance by however many graphemes there are to the right
                        column_start + columns + 1,
                    )))
                }
            }
            // correct
            QUOTATION => {
                if self.line != line_start {
                    let value = String::from_utf8_lossy(
                        self.source.get(index_start..self.index).unwrap_or(&[]),
                    )
                    .into_owned();

                    let column = str::from_utf8(
                        self.source
                            .get(last_newline_index..self.index)
                            .unwrap_or(&[]),
                    )
                    .unwrap()
                    .graphemes(true)
                    .count() as u32
                        + 1; // to account for the ending "

                    self.advance_byte();
                    self.column = column + 1;

                    Some(Ok(Token::new(
                        TokenKind::VerbatimString,
                        value,
                        (line_start, column_start),
                        (self.line, column),
                    )))
                } else {
                    let value = String::from_utf8_lossy(
                        self.source.get(index_start..self.index).unwrap_or(&[]),
                    )
                    .into_owned();
                    let columns = value.graphemes(true).count() as u32;

                    self.advance_byte();
                    //                        @"..."
                    // column_start points at ^, we add two to account for the " pair,
                    // then advance by however many graphemes there are to the right,
                    // then add one because of advance_byte()
                    self.column = column_start + columns + 3;

                    Some(Ok(Token::new(
                        TokenKind::VerbatimString,
                        value,
                        (self.line, column_start),
                        // self.column points one column too far to the right
                        (self.line, self.column - 1),
                    )))
                }
            }

            _ => unreachable!(),
        }
    }

    fn comment(&mut self, column_start: u32) -> Option<Result<Token, LexerError>> {
        //                     //    #
        // self.index points at ^ or ^
        let is_shell_comment = match self.current_byte() {
            HASH => true,
            SLASH => false,
            _ => unreachable!(),
        };
        let index_start = self.index + 1;

        loop {
            match self.advance_byte() {
                NEWLINE | NULL => break,
                _ => continue,
            }
        }

        let value =
            str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[])).unwrap();
        let columns = value.graphemes(true).count() as u32;

        self.column += columns + 1;
        if is_shell_comment {
            Some(Ok(self.token_on_line_with_value(
                TokenKind::ShellComment,
                value,
                column_start,
            )))
        } else {
            Some(Ok(self.token_on_line_with_value(
                TokenKind::CComment,
                value,
                column_start,
            )))
        }
    }

    // TODO: This method's logic is very similar to verbatim_string,
    // find a way to DRY this?
    fn multi_line_comment(
        &mut self,
        line_start: u32,
        column_start: u32,
    ) -> Option<Result<Token, LexerError>> {
        //                     /*
        // self.index points at ^
        let index_start = self.index + 1;
        let mut last_newline_index = 0;

        loop {
            match self.advance_byte() {
                NULL => break,
                NEWLINE => {
                    self.line += 1;
                    // A newline counts as one grapheme, we don't want to include that
                    last_newline_index = self.index + 1;
                    continue;
                }
                ASTERISK => {
                    match self.peek_byte() {
                        // comment ends
                        SLASH => {
                            self.advance_byte();
                            break;
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }

        // self.index points at NULL or SLASH
        match self.current_byte() {
            // unclosed
            NULL => {
                if self.line != line_start {
                    let mut column = str::from_utf8(
                        self.source
                            .get(last_newline_index..self.index)
                            .unwrap_or(&[]),
                    )
                    .unwrap()
                    .graphemes(true)
                    .count() as u32;

                    if column == 0 {
                        column = 1;
                    }

                    self.terminate();
                    Some(Err(LexerError::new(
                        LexerErrorKind::UnclosedMultiLineComment,
                        self.line,
                        column,
                    )))
                } else {
                    let columns =
                        str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[]))
                            .unwrap()
                            .graphemes(true)
                            .count() as u32;

                    self.terminate();
                    Some(Err(LexerError::new(
                        LexerErrorKind::UnclosedMultiLineComment,
                        self.line,
                        column_start + columns + 1,
                    )))
                }
            }
            SLASH => {
                if self.line != line_start {
                    let value = String::from_utf8_lossy(
                        //           ... */
                        // Don't include ^
                        self.source.get(index_start..self.index - 1).unwrap_or(&[]),
                    )
                    .into_owned();

                    // This counter includes the "*" from above ...
                    let column = str::from_utf8(
                        self.source
                            .get(last_newline_index..self.index)
                            .unwrap_or(&[]),
                    )
                    .unwrap()
                    .graphemes(true)
                    .count() as u32
                        + 1; // ... so we only add 1 to compensate for the "/"

                    self.advance_byte();
                    self.column = column + 1;

                    Some(Ok(Token::new(
                        TokenKind::MultiLineComment,
                        value,
                        (line_start, column_start),
                        (self.line, column),
                    )))
                } else {
                    let value = String::from_utf8_lossy(
                        self.source.get(index_start..self.index - 1).unwrap_or(&[]),
                    )
                    .into_owned();
                    let columns = value.graphemes(true).count() as u32;

                    self.advance_byte();
                    // Add 4 to account for "*", "*/" and advance_byte()
                    self.column = column_start + columns + 4;

                    Some(Ok(Token::new(
                        TokenKind::MultiLineComment,
                        value,
                        (self.line, column_start),
                        // self.column points one column too far to the right
                        (self.line, self.column - 1),
                    )))
                }
            }
            _ => unreachable!(),
        }
    }

    fn number(&mut self) -> Option<Result<Token, LexerError>> {
        let column_start = self.column;
        let index_start = self.index;
        let first = self.current_byte();
        let second = self.advance_char();

        if first == ZERO {
            match second {
                ZERO..=SEVEN => {
                    return self.octal(index_start, column_start);
                }
                UPPER_X | LOWER_X => {
                    return self.hexadecimal(index_start, column_start);
                }
                EIGHT | NINE => {
                    return self.decimal(index_start, column_start);
                }
                DOT | UPPER_E | LOWER_E => {}
                _ => {
                    return Some(Ok(self.token_on_line_with_value(
                        TokenKind::Integer,
                        "0",
                        column_start,
                    )));
                }
            };
        }

        let mut kind = TokenKind::Integer;

        loop {
            match self.current_byte() {
                DOT => kind = TokenKind::Float,
                ZERO..=NINE => {}
                UPPER_E | LOWER_E => match self.advance_char() {
                    ZERO..=NINE => kind = TokenKind::Float,
                    PLUS | MINUS => match self.advance_char() {
                        ZERO..=NINE => kind = TokenKind::Float,
                        _ => return self.error(LexerErrorKind::MissingFloatExponent),
                    },
                    _ => return self.error(LexerErrorKind::MissingFloatExponent),
                },

                _ => break,
            }

            self.advance_char();
        }

        let value =
            str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[])).unwrap();
        Some(Ok(self.token_on_line_with_value(kind, value, column_start)))
    }

    fn octal(
        &mut self,
        index_start: usize,
        column_start: u32,
    ) -> Option<Result<Token, LexerError>> {
        //                     0n...
        // self.index points at ^, n must be in 0..=7
        loop {
            match self.advance_char() {
                ZERO..=SEVEN => continue,
                EIGHT | NINE => return self.error(LexerErrorKind::InvalidOctal),
                _ => break,
            }
        }

        let value =
            str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[])).unwrap();
        Some(Ok(self.token_on_line_with_value(
            TokenKind::Integer,
            value,
            column_start,
        )))
    }

    fn hexadecimal(
        &mut self,
        index_start: usize,
        column_start: u32,
    ) -> Option<Result<Token, LexerError>> {
        //                     0x.. 0X..
        // self.index points at ^ or ^
        while let UPPER_A..=UPPER_F | LOWER_A..=LOWER_F | ZERO..=NINE = self.advance_char() {
            continue;
        }

        let value =
            str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[])).unwrap();
        Some(Ok(self.token_on_line_with_value(
            TokenKind::Integer,
            value,
            column_start,
        )))
    }

    fn decimal(
        &mut self,
        index_start: usize,
        column_start: u32,
    ) -> Option<Result<Token, LexerError>> {
        //                     08.. 09..
        // self.index points at ^ or ^
        while let ZERO..=NINE = self.advance_char() {
            continue;
        }

        let value =
            str::from_utf8(self.source.get(index_start..self.index).unwrap_or(&[])).unwrap();
        Some(Ok(self.token_on_line_with_value(
            TokenKind::Integer,
            value,
            column_start,
        )))
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

    // Same as advance_char, but does not increment column count.
    // You should handle the column logic manually.
    fn advance_byte(&mut self) -> u8 {
        self.index += 1;
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
    /// An invalid octal number was encountered.
    ///
    /// Squirrel lexes octal numbers weirdly. If the second digit contains non-octal digits, i.e. 8
    /// and 9, the entire number is understood as decimal. Otherwise, it is octal and octal lexing
    /// rules apply.
    ///
    /// # Examples
    ///
    /// - `091` is decimal.
    /// - `072` is octal.
    /// - `028` is invalid octal.
    /// - `0006` is octal.
    InvalidOctal,
    /// An exponent for floating point numbers written in scientific notation was missing.
    MissingFloatExponent,
    /// A `char`-like literal was unclosed.
    UnclosedChar,
    /// A multi-line comment was unclosed.
    UnclosedMultiLineComment,
    /// A verbatim string was unclosed.
    UnclosedVerbatimString,
    /// A string was unclosed.
    UnclosedString,
    /// An unexpected symbol was encountered outside of comments or strings.
    UnexpectedSymbol,
}

/// An object returned by the [`Lexer`] when it encounters an error.
#[derive(Debug, PartialEq)]
pub struct LexerError {
    /// The kind of error encountered.
    pub kind: LexerErrorKind,
    /// The position of the error in source code.
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

    fn error(kind: LexerErrorKind, line: u32, column: u32) -> Option<Result<Token, LexerError>> {
        Some(Err(LexerError::new(kind, line, column)))
    }

    // Ascertain that, either:
    //
    // - An error has the correct kind and position,
    //   and that the token stream ends thereafter; or
    // - A token has the correct kind, value, starting and ending positions,
    //   and that it properly advances to the next character;
    //
    // from the input source string.
    macro_rules! assert_token {
        (
            $source: expr,
            $kind: expr,
            $line: expr,
            $column: expr
        ) => {{
            let mut lexer = Lexer::new($source);
            let vec_source = vec![lexer.next_token(), lexer.next_token()];
            let vec_compare = vec![error($kind, $line, $column), None];

            assert_eq!(vec_source, vec_compare);
        }};
        (
            $source: expr,
            $kind: expr,
            $value: expr,
            $start: expr,
            $end: expr
        ) => {{
            let mut lexer = Lexer::new($source);
            let vec_source = vec![lexer.next_token(), lexer.next_token()];
            let vec_compare = vec![
                token($kind, $value, $start, $end),
                token(Eof, "", $end, $end),
            ];

            assert_eq!(vec_source, vec_compare);
        }};
    }

    // Ascertain that a correct stream of tokens is produced from a given source string.
    macro_rules! assert_stream {
        (
            $source: expr,
            $(
                $token: expr
            ),+
        ) => {{
            let mut lexer = Lexer::new($source);
            let mut vec_source = Vec::new();

            let mut token = lexer.next_token();
            while token.is_some() {
                vec_source.push(token);
                token = lexer.next_token();
            }

            assert_eq!(vec_source, vec![$($token,)+]);
        }};
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
        assert_token!("if", If, "", (1, 1), (1, 2));
        assert_token!("in", In, "", (1, 1), (1, 2));

        assert_token!("for", For, "", (1, 1), (1, 3));
        assert_token!("try", Try, "", (1, 1), (1, 3));

        assert_token!("base", Base, "", (1, 1), (1, 4));
        assert_token!("case", Case, "", (1, 1), (1, 4));
        assert_token!("else", Else, "", (1, 1), (1, 4));
        assert_token!("enum", Enum, "", (1, 1), (1, 4));
        assert_token!("null", Null, "", (1, 1), (1, 4));
        assert_token!("this", This, "", (1, 1), (1, 4));
        assert_token!("true", True, "", (1, 1), (1, 4));

        assert_token!("break", Break, "", (1, 1), (1, 5));
        assert_token!("catch", Catch, "", (1, 1), (1, 5));
        assert_token!("class", Class, "", (1, 1), (1, 5));
        assert_token!("clone", Clone, "", (1, 1), (1, 5));
        assert_token!("const", Const, "", (1, 1), (1, 5));
        assert_token!("false", False, "", (1, 1), (1, 5));
        assert_token!("local", Local, "", (1, 1), (1, 5));
        assert_token!("throw", Throw, "", (1, 1), (1, 5));
        assert_token!("while", While, "", (1, 1), (1, 5));
        assert_token!("yield", Yield, "", (1, 1), (1, 5));

        assert_token!("delete", Delete, "", (1, 1), (1, 6));
        assert_token!("resume", Resume, "", (1, 1), (1, 6));
        assert_token!("return", Return, "", (1, 1), (1, 6));
        assert_token!("static", Static, "", (1, 1), (1, 6));
        assert_token!("switch", Switch, "", (1, 1), (1, 6));
        assert_token!("typeof", Typeof, "", (1, 1), (1, 6));

        assert_token!("default", Default, "", (1, 1), (1, 7));
        assert_token!("extends", Extends, "", (1, 1), (1, 7));
        assert_token!("foreach", Foreach, "", (1, 1), (1, 7));
        assert_token!("rawcall", Rawcall, "", (1, 1), (1, 7));

        assert_token!("__FILE__", File, "", (1, 1), (1, 8));
        assert_token!("__LINE__", Line, "", (1, 1), (1, 8));
        assert_token!("continue", Continue, "", (1, 1), (1, 8));
        assert_token!("function", Function, "", (1, 1), (1, 8));

        assert_token!("instanceof", InstanceOf, "", (1, 1), (1, 10));
        assert_token!("constructor", Constructor, "", (1, 1), (1, 11));
    }

    #[test]
    #[rustfmt::skip]
    fn identifiers() {
        // unused variable
        assert_token!("_", Identifier, "_", (1, 1), (1, 1));
        assert_token!("f", Identifier, "f", (1, 1), (1, 1));
        assert_token!("F", Identifier, "F", (1, 1), (1, 1));
        assert_token!("f1", Identifier, "f1", (1, 1), (1, 2));
        assert_token!("_1", Identifier, "_1", (1, 1), (1, 2));
        assert_token!("__", Identifier, "__", (1, 1), (1, 2));
        // general variable
        assert_token!("foo", Identifier, "foo", (1, 1), (1, 3));
        assert_token!("__fo", Identifier, "__fo", (1, 1), (1, 4));
        assert_token!("__2fo", Identifier, "__2fo", (1, 1), (1, 5));
        // PascalCase
        assert_token!("FooBar", Identifier, "FooBar", (1, 1), (1, 6));
        assert_token!("fOo2BaR", Identifier, "fOo2BaR", (1, 1), (1, 7));
        // camelCase
        assert_token!("fooBarBa", Identifier, "fooBarBa", (1, 1), (1, 8));
        // SCREAMING_SNAKE_CASE
        assert_token!("HALF_LIFE", Identifier, "HALF_LIFE", (1, 1), (1, 9));
        // snake_case
        assert_token!("portal_two", Identifier, "portal_two", (1, 1), (1, 10));
        // a general script function beginning with "_"
        assert_token!("__DumpScope", Identifier, "__DumpScope", (1, 1), (1, 11));
        assert_token!("__0foobarbaz", Identifier, "__0foobarbaz", (1, 1), (1, 12));
        assert_token!("___0123456789", Identifier, "___0123456789", (1, 1), (1, 13));
    }

    #[test]
    fn operators() {
        assert_token!("+", Add, "", (1, 1), (1, 1));
        assert_token!("+=", AddAssign, "", (1, 1), (1, 2));
        assert_token!("&&", And, "", (1, 1), (1, 2));
        assert_token!("=", Assign, "", (1, 1), (1, 1));
        assert_token!("&", BitAnd, "", (1, 1), (1, 1));
        assert_token!("<<", BitLeft, "", (1, 1), (1, 2));
        assert_token!("~", BitNot, "", (1, 1), (1, 1));
        assert_token!("|", BitOr, "", (1, 1), (1, 1));
        assert_token!(">>", BitRight, "", (1, 1), (1, 2));
        assert_token!("^", BitXor, "", (1, 1), (1, 1));
        assert_token!(",", Comma, "", (1, 1), (1, 1));
        assert_token!("--", Decrement, "", (1, 1), (1, 2));
        assert_token!("/", Div, "", (1, 1), (1, 1));
        assert_token!("/=", DivAssign, "", (1, 1), (1, 2));
        assert_token!("==", Eq, "", (1, 1), (1, 2));
        assert_token!(">=", Ge, "", (1, 1), (1, 2));
        assert_token!(">", Gt, "", (1, 1), (1, 1));
        assert_token!("++", Increment, "", (1, 1), (1, 2));
        assert_token!("<-", Ins, "", (1, 1), (1, 2));
        assert_token!("<=", Le, "", (1, 1), (1, 2));
        assert_token!("<", Lt, "", (1, 1), (1, 1));
        assert_token!("%", Mod, "", (1, 1), (1, 1));
        assert_token!("%=", ModAssign, "", (1, 1), (1, 2));
        assert_token!("*", Mult, "", (1, 1), (1, 1));
        assert_token!("*=", MultAssign, "", (1, 1), (1, 2));
        assert_token!("!=", Neq, "", (1, 1), (1, 2));
        assert_token!("!", Not, "", (1, 1), (1, 1));
        assert_token!("||", Or, "", (1, 1), (1, 2));
        assert_token!("<=>", Spaceship, "", (1, 1), (1, 3));
        assert_token!("-", Sub, "", (1, 1), (1, 1));
        assert_token!("-=", SubAssign, "", (1, 1), (1, 2));
        assert_token!(">>>", UnsignedRight, "", (1, 1), (1, 3));
    }

    #[test]
    fn misc_tokens() {
        assert_token!("}", BraceClose, "", (1, 1), (1, 1));
        assert_token!("{", BraceOpen, "", (1, 1), (1, 1));
        assert_token!(":", Colon, "", (1, 1), (1, 1));
        assert_token!(".", Dot, "", (1, 1), (1, 1));
        assert_token!("...", Ellipsis, "", (1, 1), (1, 3));
        assert_token!(")", ParenClose, "", (1, 1), (1, 1));
        assert_token!("(", ParenOpen, "", (1, 1), (1, 1));
        assert_token!("::", ScopeRes, "", (1, 1), (1, 2));
        assert_token!(";", Semicolon, "", (1, 1), (1, 1));
        assert_token!("]", SquareClose, "", (1, 1), (1, 1));
        assert_token!("[", SquareOpen, "", (1, 1), (1, 1));
        assert_token!("@", Lambda, "", (1, 1), (1, 1));
    }

    #[test]
    fn unexpected_symbol() {
        assert_token!("ändern", UnexpectedSymbol, 1, 1);
        // TODO: what if these symbols occur elsewhere, such as
        // `local möglich`?
    }

    #[test]
    fn char() {
        assert_token!("'f'", Char, "f", (1, 1), (1, 3));
    }

    #[test]
    fn char_oob() {
        assert_token!("'Ü'", CharOutOfBounds, 1, 2);
    }

    #[test]
    fn char_too_long() {
        assert_token!("'xd'", CharTooLong, 1, 3);
    }

    #[test]
    fn char_empty() {
        assert_token!("''", EmptyChar, 1, 2);
    }

    #[test]
    fn char_unclosed() {
        assert_token!("'\n", UnclosedChar, 1, 1);
        assert_token!("'f\n", UnclosedChar, 1, 2);
        assert_token!("'", UnclosedChar, 1, 1);
        assert_token!("'f", UnclosedChar, 1, 2);
    }

    #[test]
    fn string_empty() {
        assert_token!("\"\"", String, "", (1, 1), (1, 2));
    }

    #[test]
    fn string() {
        assert_token!("\" _0aZ!$█░ \"", String, " _0aZ!$█░ ", (1, 1), (1, 12));
    }

    #[test]
    fn string_unclosed() {
        assert_token!("\"\n", UnclosedString, 1, 1);
        assert_token!("\"a█\n", UnclosedString, 1, 3);
        assert_token!("\"", UnclosedString, 1, 1);
        assert_token!("\"a█", UnclosedString, 1, 3);
    }

    #[test]
    fn verbatim_string_empty() {
        assert_token!("@\"\"", VerbatimString, "", (1, 1), (1, 3));
    }

    #[test]
    fn verbatim_string() {
        assert_token!(
            "@\" _0aZ!$█░ \"",
            VerbatimString,
            " _0aZ!$█░ ",
            (1, 1),
            (1, 13)
        );
        assert_token!(
            r#"@" _0aZ!$█░ 
 _0aZ!$█░ ""#,
            VerbatimString,
            " _0aZ!$█░ \n _0aZ!$█░ ",
            (1, 1),
            (2, 11)
        );
        assert_token!(
            r#"@" _0aZ!$█░ 
""#,
            VerbatimString,
            " _0aZ!$█░ \n",
            (1, 1),
            (2, 1)
        );
    }

    #[test]
    fn verbatim_string_escapes() {
        assert_token!(r#"@"\r\n""#, VerbatimString, "\\r\\n", (1, 1), (1, 7));
    }

    #[test]
    fn verbatim_string_quotes() {
        assert_token!(
            r#"@"""hello""""#,
            VerbatimString,
            r#"""hello"""#,
            (1, 1),
            (1, 12)
        );
    }

    #[test]
    fn verbatim_string_unclosed() {
        assert_token!("@\"", UnclosedVerbatimString, 1, 2);
        assert_token!("@\" _0aZ!$█░ ", UnclosedVerbatimString, 1, 12);
        assert_token!(
            r#"@"
"#,
            UnclosedVerbatimString,
            2,
            1
        );
        assert_token!(
            r#"@"
 _0aZ!$█░ "#,
            UnclosedVerbatimString,
            2,
            10
        );
    }

    #[test]
    fn comment_empty() {
        assert_token!("//", CComment, "", (1, 1), (1, 2));
        assert_token!("#", ShellComment, "", (1, 1), (1, 1));
    }

    #[test]
    fn comment() {
        assert_token!("// _0aZ!$█░ ", CComment, " _0aZ!$█░ ", (1, 1), (1, 12));
        assert_token!("# _0aZ!$█░ ", ShellComment, " _0aZ!$█░ ", (1, 1), (1, 11));
    }

    #[test]
    fn multi_line_comment_empty() {
        assert_token!("/**/", MultiLineComment, "", (1, 1), (1, 4));
    }

    #[test]
    fn multi_line_comment() {
        assert_token!(
            "/* _0aZ!$█░ / * **/",
            MultiLineComment,
            " _0aZ!$█░ / * *",
            (1, 1),
            (1, 19)
        );

        // Notice the stripping of the "/*" and "*/" at the two ends
        assert_token!(
            r#"/** multi
                *  line
                **/"#,
            MultiLineComment,
            r#"* multi
                *  line
                *"#,
            (1, 1),
            (3, 19)
        );

        assert_token!(
            r#"/** multi
                *  line
                *   _0aZ!$█░  */"#,
            MultiLineComment,
            r#"* multi
                *  line
                *   _0aZ!$█░  "#,
            (1, 1),
            (3, 32)
        );
    }

    #[test]
    fn multi_line_comment_unclosed() {
        assert_token!("/*", UnclosedMultiLineComment, 1, 2);
        assert_token!("/* *", UnclosedMultiLineComment, 1, 4);
        assert_token!("/* * /", UnclosedMultiLineComment, 1, 6);
        assert_token!("/*\n", UnclosedMultiLineComment, 2, 1);
        assert_token!("/*\n* * /", UnclosedMultiLineComment, 2, 5);
    }

    #[test]
    fn octal() {
        assert_token!("0", Integer, "0", (1, 1), (1, 1));
        assert_token!("00000", Integer, "00000", (1, 1), (1, 5));
        assert_token!("07125", Integer, "07125", (1, 1), (1, 5));
        assert_token!("00000001", Integer, "00000001", (1, 1), (1, 8));
    }

    #[test]
    fn octal_invalid() {
        // The error pointing at one column before the offending digit
        // is correct Squirrel diagnosis.
        assert_token!("0080", InvalidOctal, 1, 2);
        assert_token!("04078", InvalidOctal, 1, 4);
    }

    #[test]
    fn decimal() {
        assert_token!("2", Integer, "2", (1, 1), (1, 1));
        assert_token!("42", Integer, "42", (1, 1), (1, 2));
        assert_token!("1337", Integer, "1337", (1, 1), (1, 4));
    }

    #[test]
    fn hexadecimals() {
        assert_token!("0x", Integer, "0x", (1, 1), (1, 2));
        assert_token!("0X", Integer, "0X", (1, 1), (1, 2));
        assert_token!("0x012aBc", Integer, "0x012aBc", (1, 1), (1, 8));
        assert_token!("0X034CdE", Integer, "0X034CdE", (1, 1), (1, 8));
        assert_token!("0x567AbC", Integer, "0x567AbC", (1, 1), (1, 8));
        assert_token!("0X890cDe", Integer, "0X890cDe", (1, 1), (1, 8));
    }

    #[test]
    fn float() {
        assert_token!("0.", Float, "0.", (1, 1), (1, 2));
        assert_token!("0.015", Float, "0.015", (1, 1), (1, 5));
        assert_token!("2.71", Float, "2.71", (1, 1), (1, 4));
        assert_token!("3e8", Float, "3e8", (1, 1), (1, 3));
        assert_token!("6.02e+23", Float, "6.02e+23", (1, 1), (1, 8));
        assert_token!("1.6e-19", Float, "1.6e-19", (1, 1), (1, 7));
        assert_token!("44.1E3", Float, "44.1E3", (1, 1), (1, 6));
        assert_token!("196E+3", Float, "196E+3", (1, 1), (1, 6));
        assert_token!("1.38E-23", Float, "1.38E-23", (1, 1), (1, 8));

        // The more insane guys
        assert_token!("5.35....1", Float, "5.35....1", (1, 1), (1, 9));
        assert_token!("0...e2", Float, "0...e2", (1, 1), (1, 6));
        assert_token!(
            "1e5.125..e8...e-1..E+12.0e+10",
            Float,
            "1e5.125..e8...e-1..E+12.0e+10",
            (1, 1),
            (1, 29)
        );
        assert_stream!(
            "4.5e+10-2",
            token(Float, "4.5e+10", (1, 1), (1, 7)),
            token(Sub, "", (1, 8), (1, 8)),
            token(Integer, "2", (1, 9), (1, 9)),
            token(Eof, "", (1, 9), (1, 9))
        );
    }

    #[test]
    fn float_missing_exponent() {
        assert_token!("0e", MissingFloatExponent, 1, 2);
        assert_token!("0E", MissingFloatExponent, 1, 2);
        assert_token!("1.2e+", MissingFloatExponent, 1, 5);
        assert_token!("1.2e-", MissingFloatExponent, 1, 5);
        assert_token!("3.E+", MissingFloatExponent, 1, 4);
        assert_token!("3.E-", MissingFloatExponent, 1, 4);
        assert_token!("4.56E-", MissingFloatExponent, 1, 6);
        assert_token!("4.56E-", MissingFloatExponent, 1, 6);
        // If the symbol after "e" is a + or -, the error points at the symbol.
        // Otherwise, it points at the "e".
        assert_token!("7e8..9.0e.", MissingFloatExponent, 1, 9);
        assert_token!("7e8..9.0e+a", MissingFloatExponent, 1, 10);
        assert_token!("7e8..9.0e-b", MissingFloatExponent, 1, 10);
    }
}
