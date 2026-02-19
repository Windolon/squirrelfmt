const NULL: u8 = 0;

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

    fn tok(kind: TokenKind, value: &str, lines: (u32, u32), columns: (u32, u32)) -> Token {
        let pos = Position::new(lines, columns);
        Token::new(kind, value.to_string(), pos)
    }

    #[test]
    fn eof_empty_none() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), Some(Ok(tok(Eof, "", (1, 1), (1, 1)))));
        assert_eq!(lexer.next_token(), None);
        assert_eq!(lexer.next_token(), None);
    }
}
