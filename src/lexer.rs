const NULL: u8 = 0;
const UPPER_A: u8 = 65;
const UPPER_Z: u8 = 90;
const UNDERSCORE: u8 = 95;
const LOWER_A: u8 = 97;
const LOWER_Z: u8 = 122;
const ZERO: u8 = 48;
const NINE: u8 = 57;

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
            _ => todo!(),
        }
    }

    fn eof(&mut self) -> Option<Result<Token, LexerError>> {
        let line = self.line;
        let column = self.column;

        if self.did_send_eof {
            return None;
        }

        self.did_send_eof = true;

        // TODO: Rewrite this with a wrapper method
        if column == 1 {
            let pos = Position::new((line, line), (column, column));
            Some(Ok(Token::new(TokenKind::Eof, "".to_string(), pos)))
        } else {
            let pos = Position::new((line, line), (column - 1, column - 1));
            Some(Ok(Token::new(TokenKind::Eof, "".to_string(), pos)))
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

        // The loop above should have made sure that we are dealing with valid bytes in range,
        // so it is ok to use from_utf8_lossy and direct indexing to avoid having to unwrap things.
        let value = String::from_utf8_lossy(&self.source[index_start..self.index]).into_owned();

        // This logic follows Inko's implementation.
        //
        // If we did a simple match against all keywords, each identifier
        // would need to be run through every match arm before exiting.
        let kind = match value.len() {
            2 => match value.as_str() {
                "if" => TokenKind::If,
                "in" => TokenKind::In,
                _ => TokenKind::Identifier,
            },
            3 => match value.as_str() {
                "for" => TokenKind::For,
                "try" => TokenKind::Try,
                _ => TokenKind::Identifier,
            },
            4 => match value.as_str() {
                "base" => TokenKind::Base,
                "case" => TokenKind::Case,
                "else" => TokenKind::Else,
                "enum" => TokenKind::Enum,
                "null" => TokenKind::Null,
                "this" => TokenKind::This,
                "true" => TokenKind::True,
                _ => TokenKind::Identifier,
            },
            5 => match value.as_str() {
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
            6 => match value.as_str() {
                "delete" => TokenKind::Delete,
                "resume" => TokenKind::Resume,
                "return" => TokenKind::Return,
                "static" => TokenKind::Static,
                "switch" => TokenKind::Switch,
                "typeof" => TokenKind::Typeof,
                _ => TokenKind::Identifier,
            },
            7 => match value.as_str() {
                "default" => TokenKind::Default,
                "extends" => TokenKind::Extends,
                "foreach" => TokenKind::Foreach,
                "rawcall" => TokenKind::Rawcall,
                _ => TokenKind::Identifier,
            },
            8 => match value.as_str() {
                "__FILE__" => TokenKind::File,
                "__LINE__" => TokenKind::Line,
                "continue" => TokenKind::Continue,
                "function" => TokenKind::Function,
                _ => TokenKind::Identifier,
            },
            10 => match value.as_str() {
                "instanceof" => TokenKind::InstanceOf,
                _ => TokenKind::Identifier,
            },
            11 => match value.as_str() {
                "constructor" => TokenKind::Constructor,
                _ => TokenKind::Identifier,
            },
            _ => TokenKind::Identifier,
        };

        let position = Position::new((self.line, self.line), (column_start, self.column - 1));

        if kind == TokenKind::Identifier {
            Some(Ok(Token::new(kind, value, position)))
        } else {
            Some(Ok(Token::new(kind, "".to_string(), position)))
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

    fn tok_from(source: &str) -> Option<Result<Token, LexerError>> {
        let mut lexer = Lexer::new(source);
        lexer.next_token()
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
        assert_eq!(tok_from("if"), tok_wrapped(If, "", (1, 1), (1, 2)));
        assert_eq!(tok_from("in"), tok_wrapped(In, "", (1, 1), (1, 2)));

        assert_eq!(tok_from("for"), tok_wrapped(For, "", (1, 1), (1, 3)));
        assert_eq!(tok_from("try"), tok_wrapped(Try, "", (1, 1), (1, 3)));

        assert_eq!(tok_from("base"), tok_wrapped(Base, "", (1, 1), (1, 4)));
        assert_eq!(tok_from("case"), tok_wrapped(Case, "", (1, 1), (1, 4)));
        assert_eq!(tok_from("else"), tok_wrapped(Else, "", (1, 1), (1, 4)));
        assert_eq!(tok_from("enum"), tok_wrapped(Enum, "", (1, 1), (1, 4)));
        assert_eq!(tok_from("null"), tok_wrapped(Null, "", (1, 1), (1, 4)));
        assert_eq!(tok_from("this"), tok_wrapped(This, "", (1, 1), (1, 4)));
        assert_eq!(tok_from("true"), tok_wrapped(True, "", (1, 1), (1, 4)));

        assert_eq!(tok_from("break"), tok_wrapped(Break, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("catch"), tok_wrapped(Catch, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("class"), tok_wrapped(Class, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("clone"), tok_wrapped(Clone, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("const"), tok_wrapped(Const, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("false"), tok_wrapped(False, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("local"), tok_wrapped(Local, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("throw"), tok_wrapped(Throw, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("while"), tok_wrapped(While, "", (1, 1), (1, 5)));
        assert_eq!(tok_from("yield"), tok_wrapped(Yield, "", (1, 1), (1, 5)));

        assert_eq!(tok_from("delete"), tok_wrapped(Delete, "", (1, 1), (1, 6)));
        assert_eq!(tok_from("resume"), tok_wrapped(Resume, "", (1, 1), (1, 6)));
        assert_eq!(tok_from("return"), tok_wrapped(Return, "", (1, 1), (1, 6)));
        assert_eq!(tok_from("static"), tok_wrapped(Static, "", (1, 1), (1, 6)));
        assert_eq!(tok_from("switch"), tok_wrapped(Switch, "", (1, 1), (1, 6)));
        assert_eq!(tok_from("typeof"), tok_wrapped(Typeof, "", (1, 1), (1, 6)));

        assert_eq!(tok_from("default"), tok_wrapped(Default, "", (1, 1), (1, 7)));
        assert_eq!(tok_from("extends"), tok_wrapped(Extends, "", (1, 1), (1, 7)));
        assert_eq!(tok_from("foreach"), tok_wrapped(Foreach, "", (1, 1), (1, 7)));
        assert_eq!(tok_from("rawcall"), tok_wrapped(Rawcall, "", (1, 1), (1, 7)));

        assert_eq!(tok_from("__FILE__"), tok_wrapped(File, "", (1, 1), (1, 8)));
        assert_eq!(tok_from("__LINE__"), tok_wrapped(Line, "", (1, 1), (1, 8)));
        assert_eq!(tok_from("continue"), tok_wrapped(Continue, "", (1, 1), (1, 8)));
        assert_eq!(tok_from("function"), tok_wrapped(Function, "", (1, 1), (1, 8)));

        assert_eq!(tok_from("instanceof"), tok_wrapped(InstanceOf, "", (1, 1), (1, 10)));
        assert_eq!(tok_from("constructor"), tok_wrapped(Constructor, "", (1, 1), (1, 11)));
    }

    #[test]
    #[rustfmt::skip]
    fn identifiers() {
        assert_eq!(tok_from("_"), tok_wrapped(Identifier, "_", (1, 1), (1, 1)));
        assert_eq!(tok_from("f"), tok_wrapped(Identifier, "f", (1, 1), (1, 1)));
        assert_eq!(tok_from("F"), tok_wrapped(Identifier, "F", (1, 1), (1, 1)));
        assert_eq!(tok_from("f1"), tok_wrapped(Identifier, "f1", (1, 1), (1, 2)));
        assert_eq!(tok_from("_1"), tok_wrapped(Identifier, "_1", (1, 1), (1, 2)));
        assert_eq!(tok_from("__"), tok_wrapped(Identifier, "__", (1, 1), (1, 2)));
        assert_eq!(tok_from("foo"), tok_wrapped(Identifier, "foo", (1, 1), (1, 3)));
        assert_eq!(tok_from("__fo"), tok_wrapped(Identifier, "__fo", (1, 1), (1, 4)));
        assert_eq!(tok_from("__2fo"), tok_wrapped(Identifier, "__2fo", (1, 1), (1, 5)));
        assert_eq!(tok_from("FooBar"), tok_wrapped(Identifier, "FooBar", (1, 1), (1, 6)));
        assert_eq!(tok_from("fOo2BaR"), tok_wrapped(Identifier, "fOo2BaR", (1, 1), (1, 7)));
        assert_eq!(tok_from("HALFLIFE"), tok_wrapped(Identifier, "HALFLIFE", (1, 1), (1, 8)));
        assert_eq!(tok_from("fooBarBaz"), tok_wrapped(Identifier, "fooBarBaz", (1, 1), (1, 9)));
        assert_eq!(tok_from("portal_two"), tok_wrapped(Identifier, "portal_two", (1, 1), (1, 10)));
        assert_eq!(tok_from("__DumpScope"), tok_wrapped(Identifier, "__DumpScope", (1, 1), (1, 11)));
        assert_eq!(tok_from("__0foobarbaz"), tok_wrapped(Identifier, "__0foobarbaz", (1, 1), (1, 12)));
        assert_eq!(tok_from("___0123456789"), tok_wrapped(Identifier, "___0123456789", (1, 1), (1, 13)));
    }
}
