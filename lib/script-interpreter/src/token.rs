
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    // Single char token
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    At,
    Colon,
    SemiColon,
    Percent,
    Star,
    Tilde,
    QuestionMark,
    Quote,
    DoubleQuote,
    Caret,
    // one or two char token
    LogicalOr,
    OrOp,
    LogicalAnd,
    AndOp,
    Slash,
    SlashStar,
    StarSlash,
    DoubleSlash,
    Sharp,
    DoubleSharp,
    Minus,
    DecrementOp,
    Plus,
    IncrementOp,
    Dot,
    DotAt,
    Dollar,
    DollarAt,
    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    LeftCaret,
    DoubleLeftCaret,
    LeftCaretEqual,
    RightCaret,
    DoubleRightCaret,
    RightCaretEqual,
    // Literal
    Identifier(String),
    String(String),
    Number(i32),
    Label(String),
    // Keywords
    If,
    Else,
    End,
    Set,
    For,
    Goto,
    Return,
    Switch,
    Case,
    Function,
    Break,

    Eof
}

impl TokenType {
    pub fn from_keyword(value: &str) -> Option<TokenType> {
        match value {
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "end" => Some(TokenType::End),
            "set" => Some(TokenType::Set),
            "for" => Some(TokenType::For),
            "goto" => Some(TokenType::Goto),
            "return" => Some(TokenType::Return),
            "switch" => Some(TokenType::Switch),
            "case" => Some(TokenType::Case),
            "function" => Some(TokenType::Function),
            "break" => Some(TokenType::Break),
            _ => None
        }
    }
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}