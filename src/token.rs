struct Token {
    kind: TokenKind,
    literal: String,
}

enum TokenKind {
    Illegal, // 잘못된 토큰일 때.
    Eof,

    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}
