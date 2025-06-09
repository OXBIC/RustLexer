use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    posicion: usize,      // current byte offset
    caracter_actual: Option<char>,
    linea: usize,
    columna: usize,
}


impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            posicion: 0,
            caracter_actual: None,
            linea: 1,
            columna: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.posicion >= self.input.len() {
            self.caracter_actual = None;
        } else {
            let c = self.input[self.posicion..].chars().next().unwrap();
            self.caracter_actual = Some(c);
            self.posicion += c.len_utf8();

            if c == '\n' {
                self.linea += 1;
                self.columna = 0;
            } else {
                self.columna += 1;
            }
        }
    }
}

impl Lexer {
    pub fn siguiente_token(&mut self) -> Token {
        self.skip_espacioblanco();

        let line = self.linea;
        let column = self.columna;

        let token = match self.caracter_actual {
            Some('=') => Token {
                token_type: TokenType::Assign,
                linea: line,
                columna: column,
            },
            Some('+') => Token {
                token_type: TokenType::Plus,
                linea: line,
                columna: column,
            },
            Some(';') => Token {
                token_type: TokenType::Semicolon,
                linea: line,
                columna: column,
            },
            Some('(') => Token {
                token_type: TokenType::LParen,
                linea: line,
                columna: column,
            },
            Some(')') => Token {
                token_type: TokenType::RParen,
                linea: line,
                columna: column,
            },
            Some(c) if c.is_ascii_alphabetic() => {
                let ident = self.leer_identificador();
                let kind = match ident.as_str() {
                    "let" => TokenType::Let,
                    _ => TokenType::Ident(ident),
                };
                return Token { token_type: kind, linea: line, columna: column };
            }
            Some(c) if c.is_ascii_digit() => {
                let num = self.leer_numero();
                return Token {
                    token_type: TokenType::Int(num),
                    linea: line,
                    columna: column,
                };
            }
            None => Token {
                token_type: TokenType::EOF,
                linea: line,
                columna: column,
            },
            Some(c) => Token {
                token_type: TokenType::Illegal(c),
                linea: line,
                columna: column,
            },
        };

        self.read_char();
        token
    }

    // unchanged helper methods...

    fn skip_espacioblanco(&mut self) {
        while let Some(c) = self.caracter_actual {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn leer_identificador(&mut self) -> String {
        let mut identificador_literal = String::new();
        while let Some(c) = self.caracter_actual {
            if c.is_ascii_alphanumeric() || c == '_' {
                identificador_literal.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        identificador_literal
    }

    fn leer_numero(&mut self) -> i64 {
        let mut numero_literal = String::new();
        while let Some(c) = self.caracter_actual {
            if c.is_ascii_digit() {
                numero_literal.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        numero_literal.parse::<i64>().unwrap()
    }
}


