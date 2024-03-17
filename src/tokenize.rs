#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumberLiteral,
    Identifier,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    NewLine,
}

use TokenType::*;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut position = 0;
    let mut result = Vec::new();
    while position < source_code.len() {
        let current_char = source_code.chars().nth(position).unwrap();
        match current_char {
            '=' => result.push(Token {
                token_type: Equal,
                lexeme: "=".to_string(),
            }),
            '+' => result.push(Token {
                token_type: Plus,
                lexeme: "+".to_string(),
            }),
            '-' => result.push(Token {
                token_type: Minus,
                lexeme: "-".to_string(),
            }),
            '*' => result.push(Token {
                token_type: Star,
                lexeme: "*".to_string(),
            }),
            '/' => result.push(Token {
                token_type: Slash,
                lexeme: "/".to_string(),
            }),
            '(' => result.push(Token {
                token_type: LeftParen,
                lexeme: "(".to_string(),
            }),
            ')' => result.push(Token {
                token_type: RightParen,
                lexeme: ")".to_string(),
            }),
            '\n' => result.push(Token {
                token_type: NewLine,
                lexeme: '\n'.to_string(),
            }),
            x if x.is_digit(10) => {
                let mut number_lexeme = x.to_string();

                position += 1;

                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if next_char == ' ' || next_char == ')' || next_char == '\n' {
                        // FIXME: shouldn't this decrement the position first?
                        break;
                    }

                    if next_char.is_digit(10) {
                        number_lexeme.push(next_char);
                    } else {
                        panic!("Invalid character: '{}'", next_char);
                    }
                    position += 1;
                }

                result.push(Token {
                    token_type: NumberLiteral,
                    lexeme: number_lexeme,
                });

                continue; // should this be here? it's not in the scope of a loop??
            }
            ' ' => {}
            c => {
                // identifier
                let mut lexeme = c.to_string();
                position += 1;
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();

                    if !is_valid_identifier_char(next_char) {
                        break;
                    }
                    lexeme.push(next_char);
                    position += 1;
                }
                result.push(Token {
                    token_type: Identifier,
                    lexeme,
                });
                continue;
            }
        }
        position += 1;
    }
    return result;
}

fn is_valid_identifier_char(ch: char) -> bool {
    return ch.is_alphanumeric() || ch == '_';
}
