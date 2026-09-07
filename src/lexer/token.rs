#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Say,        // say (like print)
    Let,        // let (variable declaration)
    If,         // if
    Else,       // else
    Elif,       // elif
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
    Try,        // try
    Catch,      // catch


    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    True,
    False,

    // Operators
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    MultiplyAssign, // *=
    DivideAssign,   // /=
    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    And,            // and, &&
    Or,             // or, ||
    Not,            // not, !

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

impl TokenType {
    pub fn from_identifier(ident: &str) -> Self {
        match ident {
            "say" => TokenType::Say,
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elif" => TokenType::Elif,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "function" => TokenType::Function,
            "end" => TokenType::End,
            "return" => TokenType::Return,
            "when" => TokenType::When,
            "is" => TokenType::Is,
            "then" => TokenType::Then,
            "repeat" => TokenType::Repeat,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "ask" => TokenType::Ask,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            _ => TokenType::Identifier(ident.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Say => write!(f, "'say'"),
            TokenType::Let => write!(f, "'let'"),
            TokenType::If => write!(f, "'if'"),
            TokenType::Else => write!(f, "'else'"),
            TokenType::Elif => write!(f, "'elif'"),
            TokenType::While => write!(f, "'while'"),
            TokenType::For => write!(f, "'for'"),
            TokenType::In => write!(f, "'in'"),
            TokenType::Function => write!(f, "'function'"),
            TokenType::End => write!(f, "'end'"),
            TokenType::Return => write!(f, "'return'"),
            TokenType::When => write!(f, "'when'"),
            TokenType::Is => write!(f, "'is'"),
            TokenType::Then => write!(f, "'then'"),
            TokenType::Repeat => write!(f, "'repeat'"),
            TokenType::Break => write!(f, "'break'"),
            TokenType::Continue => write!(f, "'continue'"),
            TokenType::Ask => write!(f, "'ask'"),
            TokenType::Try => write!(f, "'try'"),
            TokenType::Catch => write!(f, "'catch'"),
            TokenType::Number(n) => write!(f, "number '{}'", n),
            TokenType::String(s) => write!(f, "\"{}\"", s),
            TokenType::Identifier(s) => write!(f, "identifier '{}'", s),
            TokenType::True => write!(f, "'true'"),
            TokenType::False => write!(f, "'false'"),
            TokenType::Plus => write!(f, "'+'"),
            TokenType::Minus => write!(f, "'-'"),
            TokenType::Multiply => write!(f, "'*'"),
            TokenType::Divide => write!(f, "'/'"),
            TokenType::Modulo => write!(f, "'%'"),
            TokenType::Assign => write!(f, "'='"),
            TokenType::PlusAssign => write!(f, "'+='"),
            TokenType::MinusAssign => write!(f, "'-='"),
            TokenType::MultiplyAssign => write!(f, "'*='"),
            TokenType::DivideAssign => write!(f, "'/='"),
            TokenType::Equal => write!(f, "'=='"),
            TokenType::NotEqual => write!(f, "'!='"),
            TokenType::Less => write!(f, "'<'"),
            TokenType::Greater => write!(f, "'>'"),
            TokenType::LessEqual => write!(f, "'<='"),
            TokenType::GreaterEqual => write!(f, "'>='"),
            TokenType::And => write!(f, "'and'"),
            TokenType::Or => write!(f, "'or'"),
            TokenType::Not => write!(f, "'not'"),
            TokenType::LeftParen => write!(f, "'('"),
            TokenType::RightParen => write!(f, "')'"),
            TokenType::LeftBrace => write!(f, "'{{'"),
            TokenType::RightBrace => write!(f, "'}}'"),
            TokenType::Comma => write!(f, "','"),
            TokenType::Dot => write!(f, "'.'"),
            TokenType::DotDot => write!(f, "'..'"),
            TokenType::Newline => write!(f, "newline"),
            TokenType::Eof => write!(f, "end of file"),
        }
    }
}

