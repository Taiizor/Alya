#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Say,        // say (like print)
    Let,        // let (variable declaration)
    If,         // if
    Else,       // else
    While,      // while
    For,        // for
    In,         // in
    Function,   // function
    End,        // end
    Return,     // return
    When,       // when (pattern matching)
    Is,         // is
    Then,       // then
    Repeat,     // repeat (infinite loop)
    Break,      // break
    Continue,   // continue
    Ask,        // ask (input)

    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    True,
    False,

    // Operators
    Plus,       // +
    Minus,      // -
    Multiply,   // *
    Divide,     // /
    Modulo,     // %
    Assign,     // =
    Equal,      // ==
    NotEqual,   // !=
    Less,       // <
    Greater,    // >
    LessEqual,  // <=
    GreaterEqual, // >=
    And,        // and
    Or,         // or
    Not,        // not

    // Delimiters
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    DotDot,     // ..
    Newline,    // \n

    // Special
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}
