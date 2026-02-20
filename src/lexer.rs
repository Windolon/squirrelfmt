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
            _ => todo!(),
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
    fn keywords() {
        assert_eq!(tok_from("if"), tok_wrapped(If, "", (1, 1), (1, 2)));
        assert_eq!(tok_from("in"), tok_wrapped(In, "", (1, 1), (1, 2)));
    }
}
