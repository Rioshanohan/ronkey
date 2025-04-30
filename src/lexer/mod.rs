use crate::token::{Token, TokenType, lookup_ident};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.to_owned(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::new(TokenType::ILLEGAL, &(self.ch as char).to_string());

        self.skip_whitespace();

        tok = match self.ch {
            b'=' => Token::new(TokenType::ASSIGN, "="),
            b';' => Token::new(TokenType::SEMICOLON, ";"),
            b'(' => Token::new(TokenType::LPAREN, "("),
            b')' => Token::new(TokenType::RPAREN, ")"),
            b'{' => Token::new(TokenType::LBRACE, "{"),
            b'}' => Token::new(TokenType::RBRACE, "}"),
            b',' => Token::new(TokenType::COMMA, ","),
            b'+' => Token::new(TokenType::PLUS, "+"),
            0 => Token::new(TokenType::EOF, ""),
            _ => {
                if Self::is_letter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = lookup_ident(&tok.literal);
                    return tok;
                } else if Self::is_digit(self.ch) {
                    tok.literal = self.read_number();
                    tok.token_type = TokenType::INT;
                    return tok;
                }
                tok
            }
        };
        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_owned()
    }
    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_owned()
    }
    fn is_digit(ch: u8) -> bool {
        b'0' <= ch && ch <= b'9'
    }
    fn is_letter(ch: u8) -> bool {
        b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
    }
}

#[test]
fn test_next_token() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";
    let tests: Vec<Token> = vec![
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "five"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "ten"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "add"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::FUNCTION, "fn"),
        Token::new(TokenType::LPAREN, "("),
        Token::new(TokenType::IDENT, "x"),
        Token::new(TokenType::COMMA, ","),
        Token::new(TokenType::IDENT, "y"),
        Token::new(TokenType::RPAREN, ")"),
        Token::new(TokenType::LBRACE, "{"),
        Token::new(TokenType::IDENT, "x"),
        Token::new(TokenType::PLUS, "+"),
        Token::new(TokenType::IDENT, "y"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::RBRACE, "}"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "result"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::IDENT, "add"),
        Token::new(TokenType::LPAREN, "("),
        Token::new(TokenType::IDENT, "five"),
        Token::new(TokenType::COMMA, ","),
        Token::new(TokenType::IDENT, "ten"),
        Token::new(TokenType::RPAREN, ")"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::EOF, ""),
    ];

    let mut l = Lexer::new(input);

    for tt in tests.iter() {
        let tok = l.next_token();
        if tt.literal != tok.literal || tt.token_type != tok.token_type {
            panic!(
                "expected: {:#?}, {} - actual: {:#?}, {}",
                tt.token_type, tt.literal, tok.token_type, tok.literal
            );
        }
    }
}
