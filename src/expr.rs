use crate::tokenize::Token;

pub struct AssignmentImpl {
    pub target: VariableImpl,
    pub value: Box<Expr>, // we use Box here becase Expr is referring to this type.
}
pub struct BinaryOperationImpl {
    pub lhs: Box<Expr>,
    pub operation: Token,
    pub rhs: Box<Expr>,
}

pub struct NumberImpl {
    pub value: i32,
    pub token: Token,
}

pub struct VariableImpl {
    pub name: Token,
}

pub struct FunCallImpl {
    pub name: VariableImpl,
    pub arg: Box<Expr>,
}

pub enum Expr {
    Assignment(AssignmentImpl),
    BinaryOperation(BinaryOperationImpl),
    Number(NumberImpl),
    Variable(VariableImpl),
    FunCall(FunCallImpl),
}


