mod tokenize;
mod expr;
mod parser;


use expr::{Expr, BinaryOperationImpl, NumberImpl, AssignmentImpl, VariableImpl, FunCallImpl};
use tokenize::{tokenize, TokenType::*};
use parser::parse;
use std::collections::HashMap;

type Env = HashMap<String, i32>;

fn interpret(exprs: &Vec<Expr>, env: &mut Env) {
    for expr in exprs {
        evaluate(expr, env);
    }
}

fn evaluate(expr: &Expr, env: &mut Env) -> i32 {
    match expr {
        Expr::Number(NumberImpl { value, .. }) => return *value,
        Expr::BinaryOperation(BinaryOperationImpl { lhs, operation, rhs }) => {
            let lhs_value = evaluate(lhs, env);
            let rhs_value = evaluate(rhs, env);
            match &operation.token_type {
                Plus => return lhs_value + rhs_value,
                Minus => return lhs_value - rhs_value,
                Star => return lhs_value * rhs_value,
                Slash => return lhs_value / rhs_value,
                t => panic!("Invalid binary operation: {:?}", t),
            }
        }
        Expr::Assignment(AssignmentImpl { target, value }) => {
            let value = evaluate(value, env);
            env.insert(target.name.lexeme.clone(), value);
            return value;
        }
        Expr::Variable(VariableImpl { name }) => {
            if let Some(value) = env.get(&name.lexeme) {
                return *value;
            } else {
                panic!("Varible {} isn't defined", name.lexeme);
            }
        }
        Expr::FunCall(FunCallImpl{name, arg}) => {
            if name.name.lexeme == "print" {
                let value = evaluate(arg, env);
                println!("{}", value);
                return value;
            } else {
                panic!("Undefined function {}", name.name.lexeme);
            }
        }
    }
}




fn main() {
    let src = "a_number = (123 - 3)/40 -2
        x = a_number + 4
        print(a_number)
        print(x)
        y = x *2
        print(y)
        "; 
    let tokens = tokenize(src);
    let _exprs = parse(tokens);
    let mut env = Env::new();
    interpret(&_exprs, &mut env);
}
