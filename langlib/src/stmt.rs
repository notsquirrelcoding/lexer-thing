use crate::{
    lexer::token::Token,
    parser::{error::ParserError, Parser},
};

use super::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    Assignment(Assignment),
    Print(String),
    ExprStatement(Expr),
}

impl Stmt {
    pub fn execute(&self) -> Option<Expr> {
        match self {
            Stmt::Assignment(_) => todo!(),
            Stmt::Print(str) => {
                println!("{str}");
                None
            }
            Stmt::ExprStatement(expr) => Some(expr.to_owned()),
        }
    }

    #[allow(clippy::single_match)]
    pub fn from_tokens(tokens: &[Token]) -> Result<Self, ParserError> {
        match &tokens[0] {
            Token::Keyword(keyword) => match keyword.as_str() {
                "let" => {
                    // let statements must be at least 4 tokens long.
                    if tokens.len() < 4 {
                        return Err(ParserError::InvalidLetStatement);
                    }

                    // Check for identifier
                    let ident = match tokens[1].clone() {
                        Token::Ident(ident) => ident,
                        _ => return Err(ParserError::Expected(Token::Ident("".to_owned()))),
                    };

                    // Check if there is an assignment sign.
                    if Token::AssignmentSign != tokens[2] {
                        return Err(ParserError::Expected(Token::AssignmentSign));
                    }

                    let expr = Parser::new(tokens[3..].to_vec()).expr()?;

                    Ok(Self::Assignment(Assignment { ident, val: expr }))
                }
                _ => Err(ParserError::StmtErr(StmtErr::UnknownKeyword)),
            },
            _ => {
                let expr = Parser::new(tokens.to_vec()).expr()?;

                Ok(Stmt::ExprStatement(expr))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum StmtErr {
    #[error("A failed conversion occured")]
    FailedConversion,

    #[error("An unknown keyword has been encountered. I don't even know how this is supposed to happen.")]
    UnknownKeyword,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Assignment {
    pub ident: String,
    pub val: Expr,
}

#[cfg(test)]
mod stmt_tests {
    use crate::{
        expr::Expr,
        lexer::{op::Op, token::Token},
        stmt::Assignment,
    };

    use super::Stmt;

    #[test]
    fn successful_let_stmt() {
        let tokens = vec![
            Token::Keyword("let".to_owned()),
            Token::Ident("coolVariable".to_owned()),
            Token::AssignmentSign,
            Token::LeftBracket,
            Token::Int(1),
            Token::Op(Op::Add),
            Token::Int(1),
            Token::RightBracket,
            Token::Semi,
        ];

        let binding = Stmt::from_tokens(&tokens);

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_eq!(
            binding,
            Stmt::Assignment(Assignment {
                ident: "coolVariable".to_owned(),
                val: Expr::Num(2)
            })
        )
    }

    #[test]
    fn bad_let_stmt() {
        let tokens = vec![
            Token::Keyword("let".to_owned()),
            Token::Ident("coolVariable".to_owned()),
            Token::AssignmentSign,
            Token::LeftBracket,
            Token::Int(1),
            Token::Op(Op::Add),
            Token::Int(1),
            Token::RightBracket,
            Token::Semi,
        ];

        let binding = Stmt::from_tokens(&tokens);

        assert!(binding.is_ok());

        let binding = binding.unwrap();

        assert_ne!(
            binding,
            Stmt::Assignment(Assignment {
                ident: "coolVariable".to_owned(),
                val: Expr::Num(3)
            })
        )
    }
}
