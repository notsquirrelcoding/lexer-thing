use std::{fmt::Display, mem};

use colored::Colorize;

use crate::{lexer::op::UnOp, parser::err::ParserError};

use super::lexer::op::BinOp;

#[derive(Debug, Clone, Eq, PartialEq)]

pub enum Expr {
    Num(i32),
    Str(String),
    Var(String),
    Bool(bool),
    Bin(BinExpr),
    Unary(UnOp, Box<Expr>),
    Null,
}

impl Expr {
    pub fn eval(&self) -> Result<Expr, ParserError> {
        match self {
            Expr::Bin(expr) => expr.to_owned().eval(),

            Expr::Unary(op, expr) => {
                if mem::discriminant(&Expr::Bool(false)) != mem::discriminant(&expr.eval()?) {
                    return Err(ParserError::ExprError(ExprError::InvalidUnaryOperation));
                }

                let result: bool = expr.eval()?.try_into()?;

                if op != &UnOp::Bang {
                    return Err(ParserError::ExprError(ExprError::InvalidUnaryOperation));
                }

                Ok(Expr::Bool(!result))
            }
            _ => Ok(self.to_owned()),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(num) => write!(f, "{}", format!("{num}").yellow()),

            Expr::Str(string) => write!(f, "{}", format!("\"{string}\"").green()),

            Expr::Bool(bool) => write!(f, "{}", format!("{bool}").yellow()),
            Expr::Null => write!(f, "{}", "null".bright_black()),
            other => write!(f, "{other:?}"),
        }
    }
}

impl TryInto<i32> for Expr {
    type Error = ParserError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Expr::Num(num) => Ok(num),
            _ => Err(ParserError::ExprError(ExprError::FailedConversion)),
        }
    }
}

impl TryInto<bool> for Expr {
    type Error = ParserError;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Expr::Bool(bool) => Ok(bool),
            Expr::Num(num) => Ok(num > 0),
            Expr::Str(s) => Ok(s.len() > 0),
            Expr::Null => Ok(false),
            _ => Err(ParserError::ExprError(ExprError::FailedConversion)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: BinOp,
}

impl BinExpr {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>, op: BinOp) -> Self {
        Self { lhs, rhs, op }
    }

    /// Attempts to convert the operands into numbers.
    fn try_into_nums(&self) -> Result<(i32, i32), ParserError> {
        let lhs: i32 = (*self.lhs.to_owned()).eval()?.try_into()?;

        let rhs: i32 = (*self.rhs.to_owned()).eval()?.try_into()?;

        Ok((lhs, rhs))
    }

    /// Attempts to convert the operands into booleans.
    fn try_into_bools(&self) -> Result<(bool, bool), ParserError> {
        let lhs: bool = (*self.lhs.to_owned()).eval()?.try_into()?;

        let rhs: bool = (*self.rhs.to_owned()).eval()?.try_into()?;

        Ok((lhs, rhs))
    }

    /// Evaluates the expression, and consumes itself.
    pub fn eval(self) -> Result<Expr, ParserError> {
        match self.op {
            BinOp::Add => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Num(lhs + rhs))
            }
            BinOp::Sub => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Num(lhs - rhs))
            }
            BinOp::Mul => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Num(lhs * rhs))
            }
            BinOp::Div => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Num(lhs / rhs))
            }
            BinOp::EqSign => Ok(Expr::Bool(self.lhs.eval()? == self.rhs.eval()?)),
            BinOp::GreaterSign => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Bool(lhs > rhs))
            }
            BinOp::LessSign => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Bool(lhs < rhs))
            }
            BinOp::GreaterEqSign => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Bool(lhs >= rhs))
            }
            BinOp::LessEqSign => {
                let (lhs, rhs) = self.try_into_nums()?;

                Ok(Expr::Bool(lhs <= rhs))
            }
            BinOp::And => {
                let (lhs, rhs) = self.try_into_bools()?;
                Ok(Expr::Bool(lhs && rhs))
            }
            BinOp::Or => {
                let (lhs, rhs) = self.try_into_bools()?;

                Ok(Expr::Bool(lhs || rhs))
            }
            BinOp::NeqSign => Ok(Expr::Bool(self.lhs.eval()? != self.rhs.eval()?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ExprError {
    #[error("A failed conversion occured.")]
    FailedConversion,

    #[error("The parser failed to evaluate a binary expression")]
    FailedBinEvaluation,

    #[error("The parser failed to evaluate a unary expression")]
    InvalidUnaryOperation,

    #[error("The parser failed to compare two values.")]
    InvalidComparision,
}
