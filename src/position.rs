// This is functionally equivalent to just using a (u32, u32) to record positions,
// but I prefer reading "foo.start.line" over "foo.start.0".

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}
