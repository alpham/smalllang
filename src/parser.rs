use crate::tokenize::{Token, TokenType};
use crate::expr::*;


pub fn parse(mut tokens: Vec<Token>) -> Vec<Expr> {
    tokens.reverse();
    let mut result = Vec::new();

    while tokens.len() > 0 {
        let expr = parse_expr(&mut tokens);

        expect(TokenType::NewLine, &mut tokens);

        result.push(expr);
    }
    return result;
}

pub fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
    return parse_assignment(tokens);
}

pub fn parse_assignment(tokens: &mut Vec<Token>) -> Expr {
    if tokens.len() > 1 && &tokens[tokens.len() - 2].token_type == &TokenType::Equal {
        let variable = parse_variable(tokens);
        expect(TokenType::Equal, tokens);
        let value = parse_expr(tokens);

        return Expr::Assignment(AssignmentImpl {
            target: variable,
            value: Box::new(value),
        });
    } else {
        return parse_term(tokens);
    }
}

pub fn parse_variable(tokens: &mut Vec<Token>) -> VariableImpl {
    // expect(TokenType::Identifier, tokens);
    // return VariableImpl {
    //     name: tokens.pop().unwrap(),
    // };
    let token = tokens.pop().unwrap();
    if token.token_type == TokenType::Identifier {
        return VariableImpl {
            name: token,
        }
    } else {
        panic!("Expected Identifier, found {:?}", token.token_type);
    }
}

pub fn parse_term(tokens: &mut Vec<Token>) -> Expr {
    let mut result = parse_factor(tokens);

    while tokens.len() > 0 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.token_type {
            TokenType::Plus | TokenType::Minus => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_factor(tokens);
                result = Expr::BinaryOperation(BinaryOperationImpl {
                    lhs: Box::new(result),
                    operation: op_token,
                    rhs: Box::new(rhs),
                });
            }
            _ => break,
        }
    }
    return result;
}

pub fn parse_factor(tokens: &mut Vec<Token>) -> Expr {
    let mut result = parse_primary(tokens);
    while tokens.len() > 1 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.token_type {
            TokenType::Star | TokenType::Slash => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_primary(tokens);
                result = Expr::BinaryOperation(BinaryOperationImpl {
                    lhs: Box::new(result),
                    operation: op_token,
                    rhs: Box::new(rhs),
                });
            }
            _ => break,
        }
    }

    return result;
}

pub fn parse_primary(tokens: &mut Vec<Token>) -> Expr {
    let token = tokens.pop().unwrap();

    match token.token_type {
        TokenType::NumberLiteral => {
            return Expr::Number(NumberImpl {
                value: parse_number(&token.lexeme),
                token,
            });
        }
        TokenType::Identifier => {
            if tokens.len() > 0 {
                let next_token = &tokens[tokens.len() - 1];
                if next_token.token_type == TokenType::LeftParen {
                    let fun_name = VariableImpl { name: token };
                    // Remove LeftParen
                    tokens.pop().unwrap();
                    let arg = parse_expr(tokens);
                    expect(TokenType::RightParen, tokens);
                    return Expr::FunCall(FunCallImpl{
                        name: fun_name,
                        arg: Box::new(arg),
                    });
                }
            }
            return Expr::Variable(VariableImpl { name: token });
        }
        TokenType::LeftParen => {
            let expr = parse_expr(tokens);
            expect(TokenType::RightParen, tokens);
            return expr;
        }
        t => panic!("Unexpected token type: {:?}", t),
    }
}

fn parse_number(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

pub fn expect(expected: TokenType, tokens: &mut Vec<Token>) {
    match tokens.pop() {
        None => {}
        Some(token) => {
            if token.token_type != expected {
                panic!(
                    "Expected token '{:?}' got '{:?}'",
                    expected, token.token_type
                );
            }
        }
    }
}


