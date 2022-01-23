use std::fmt::{Debug, Display, Formatter};

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
impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text: String = match self {
            TokenType::LeftParen => String::from("("),
            TokenType::RightParen => String::from(")"),
            TokenType::LeftBrace => String::from("{"),
            TokenType::RightBrace => String::from("}"),
            TokenType::LeftBracket => String::from("["),
            TokenType::RightBracket => String::from("]"),
            TokenType::Comma => String::from(","),
            TokenType::At => String::from("@"),
            TokenType::Colon => String::from(":"),
            TokenType::SemiColon => String::from(";"),
            TokenType::Percent => String::from("%"),
            TokenType::Star => String::from("*"),
            TokenType::Tilde => String::from("~"),
            TokenType::QuestionMark => String::from("?"),
            TokenType::Quote => String::from("'"),
            TokenType::DoubleQuote => String::from("\""),
            TokenType::Caret => String::from("-"),
            // one or two char token
            TokenType::LogicalOr => String::from("|"),
            TokenType::OrOp => String::from("||"),
            TokenType::LogicalAnd => String::from("&"),
            TokenType::AndOp => String::from("&&"),
            TokenType::Slash => String::from("/"),
            TokenType::SlashStar => String::from("/*"),
            TokenType::StarSlash => String::from("*/"),
            TokenType::DoubleSlash => String::from("//"),
            TokenType::Sharp => String::from("#"),
            TokenType::DoubleSharp => String::from("##"),
            TokenType::Minus => String::from("-"),
            TokenType::DecrementOp => String::from("--"),
            TokenType::Plus => String::from("+"),
            TokenType::IncrementOp => String::from("++"),
            TokenType::Dot => String::from("."),
            TokenType::DotAt => String::from(".@"),
            TokenType::Dollar => String::from("$"),
            TokenType::DollarAt => String::from("$@"),
            TokenType::Bang => String::from("!"),
            TokenType::BangEqual => String::from("!="),
            TokenType::Equal => String::from("="),
            TokenType::DoubleEqual => String::from("=="),
            TokenType::LeftCaret => String::from("<"),
            TokenType::DoubleLeftCaret => String::from("<<"),
            TokenType::LeftCaretEqual => String::from("<="),
            TokenType::RightCaret => String::from(">"),
            TokenType::DoubleRightCaret => String::from(">>"),
            TokenType::RightCaretEqual => String::from(">="),
            // Literal
            TokenType::Identifier(n) => n.to_string(),
            TokenType::String(n) => n.to_string(),
            TokenType::Number(n) => {
                format!("{}", n)
            },
            TokenType::Label(n) => n.to_string(),
            // Keywords
            TokenType::If => String::from("if"),
            TokenType::Else => String::from("else"),
            TokenType::End => String::from("end"),
            TokenType::Set => String::from("set"),
            TokenType::For => String::from("for"),
            TokenType::Goto => String::from("goto"),
            TokenType::Return => String::from("return"),
            TokenType::Switch => String::from("switch"),
            TokenType::Case => String::from("case"),
            TokenType::Function => String::from("function"),
            TokenType::Break => String::from("break"),

            TokenType::Eof => String::from("eof"),
        };
        write!(f, "{}", text)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}