use super::token::{get_keyword_token, Token};

pub fn lex(string: String) -> Vec<Token> {
    let mut l = Lexer::new(string);

    let next_token = |l: &mut Lexer, is_finished: fn(&Lexer) -> bool| {
        while l.has_char() {
            if is_finished(l) {
                return;
            } else {
                l.next_char();
            }
        }
    };

    while let Some(c) = l.next_char() {
        // default to illegal if no catch is made
        let mut tok: Token = Token::ILLEGAL(c);
        match c {
            '(' => tok = Token::LPAREN,
            ')' => tok = Token::RPAREN,
            '[' => tok = Token::LBRACE,
            ']' => tok = Token::RBRACE,
            '&' => tok = Token::LIFETIMEREFERENCE,
            ',' | ';' => tok = Token::DELIMITER(c),
            ' ' | '\t' => {
                tok = Token::SPACE
                // TODO
                // implement counting of spaces
            }
            '\n' => tok = Token::LINEBREAK,
            '"' => {
                // VALUE
                let expr = |l: &Lexer| -> bool {
                    match l.view_next_char() {
                        Some(c) => return c == '"',
                        None => true,
                    }
                };
                next_token(&mut l, expr);
                // Skip the ending quotes
                l.next_char();
                let mut buffer = l.collect_buffer();
                // Remove the beginning quotes
                buffer.retain(|&x| x != '"');
                tok = Token::VALUE(buffer);
            }
            _ => {
                // KEYWORD
                if is_ident(c) {
                    let expr = |l: &Lexer| -> bool {
                        match l.view_next_char() {
                            Some(c) => return !is_ident(c),
                            None => true,
                        }
                    };
                    next_token(&mut l, expr);
                    let keyword = &l.collect_buffer();
                    match get_keyword_token(keyword) {
                        Ok(token) => {
                            tok = token;
                        }
                        // TODO
                        // Put number here
                        Err(_) => tok = Token::IDENT(keyword.to_vec()),
                    }
                // LIFETIME
                } else if c == '\'' {
                    let expr = |l: &Lexer| -> bool {
                        match l.view_next_char() {
                            Some(c) => return !is_lifetime(c),
                            None => true,
                        }
                    };
                    next_token(&mut l, expr);
                    tok = Token::LIFETIME(l.collect_buffer());
                }
            }
        }
        l.clear_buffer();
        l.tokens.push(tok);
    }
    return l
        .tokens
        .iter()
        .map(|x| x.clone())
        .filter(|x| !matches!(*x, Token::SPACE | Token::LINEBREAK))
        .collect();
}

fn is_ident(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ('0' <= ch && ch <= '9') || ch == '_'
}

fn is_lifetime(ch: char) -> bool {
    ch == '\'' || ch == 's' || ch == 'd' || ch == 'u' || ch == 'c'
}

struct Lexer {
    input: Vec<char>,
    cur_pos: usize,
    ch: Option<char>,
    buffer: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            cur_pos: 0,
            ch: None,
            buffer: vec![],
            tokens: vec![],
        }
    }

    pub fn has_char(&self) -> bool {
        return self.ch.is_some();
    }

    pub fn view_next_char(&self) -> Option<char> {
        if self.cur_pos >= self.input.len() {
            return None;
        } else {
            return Some(self.input[self.cur_pos]);
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c: Option<char>;

        if self.cur_pos >= self.input.len() {
            self.ch = None;
            c = None;
        } else {
            let new_char = self.input[self.cur_pos];
            self.ch = Some(self.input[self.cur_pos]);
            self.buffer.push(new_char);
            c = self.ch
        }

        self.cur_pos += 1;
        return c;
    }

    pub fn collect_buffer(&mut self) -> Vec<char> {
        // Returns the buffer and removes the entries
        let buffer: Vec<char> = self.buffer.iter().map(|x| *x).collect();
        self.buffer.clear();
        return buffer;
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}
