// S -> E end
// E -> T { ("+" | "-") T }
// T -> F { ("*" | "/" | "%") F }
// F -> value | frac | "-" F | "(" E ")"
// value -> [0-9]+
// frac -> [0-9]+ / [1-9][0-9]*  // Ensure denominator is nonzero

use crate::common::Value;

use std::{convert::TryFrom, error::Error, fmt, io::prelude::*, iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Dash,
    Star,
    Slash,
    RightParen,
    LeftParen,
    End,
    Number(Value),
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Negative,
}

impl TryFrom<Token> for Operator {
    type Error = &'static str;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(Operator::Add),
            Token::Star => Ok(Operator::Multiply),
            Token::Slash => Ok(Operator::Divide),
            Token::Dash => Ok(Operator::Subtract),
            _ => Err("Token is not an operator"),
        }
    }
}

fn lex(code: String) -> Result<Vec<Token>, SyntaxError> {
    let mut iter = code.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut leftover: Option<char> = None;

    loop {
        let ch = match leftover {
            Some(ch) => ch,
            None => match iter.next() {
                None => break,
                Some(ch) => ch,
            },
        };
        leftover = None;
        match ch {
            ' ' => continue,
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            ')' => tokens.push(Token::LeftParen),
            '(' => tokens.push(Token::RightParen),
            '-' => tokens.push(Token::Dash),
            ch if ch.is_ascii_digit() => {
                let number_stream: String = iter
                    .by_ref()
                    .take_while(|c| match c.is_ascii_digit() {
                        true => true,
                        false => {
                            leftover = Some(*c);
                            false
                        }
                    })
                    .collect();
                let number: Value = format!("{}{}", ch, number_stream).parse().unwrap();
                tokens.push(Token::Number(number));
            }
            _ => {
                return Err(SyntaxError::new_lex_error(format!(
                    "Unrecognized character {}",
                    ch
                )))
            }
        }
    }

    tokens.push(Token::End);

    Ok(tokens)
}

#[derive(Debug)]
pub enum Expr {
    BinExpr(Operator, Box<Expr>, Box<Expr>),
    UnaryExpr(Operator, Box<Expr>),
    ValExrp(Value),
}

impl Expr {
    pub fn eval(&mut self) -> Result<Value, SyntaxError> {
        match self {
            Expr::ValExrp(num) => Ok((*num).clone()),
            Expr::UnaryExpr(Operator::Negative, expr) => Ok(-expr.eval()?),
            Expr::BinExpr(Operator::Add, left, right) => Ok(left.eval()? + right.eval()?),
            Expr::BinExpr(Operator::Subtract, left, right) => Ok(left.eval()? - right.eval()?),
            Expr::BinExpr(Operator::Multiply, left, right) => Ok(left.eval()? * right.eval()?),
            Expr::BinExpr(Operator::Divide, left, right) => {
                let right_val = right.eval()?;
                if right_val.is_zero() {
                    Err(SyntaxError::new_parse_error("Division by Zero".to_string()))
                } else {
                    Ok(left.eval()? / right_val)
                }
            }
            _ => Err(SyntaxError::new_parse_error(format!(
                "Unreachable code: for expr {:?}",
                self
            ))),
        }
    }
}

#[derive(Debug)]
struct SyntaxError {
    message: String,
    level: String,
}

impl SyntaxError {
    fn new_lex_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Lex".to_string(),
        }
    }

    fn new_parse_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Parse".to_string(),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

impl Error for SyntaxError {}

pub struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        Parser { iter }
    }

    fn assert_next(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();
        if let None = next {
            return Err(SyntaxError::new_parse_error(
                "Unexpected end of input".to_string(),
            ));
        }

        if *next.unwrap() != token {
            return Err(SyntaxError::new_parse_error(format!(
                "Expected {:?} actual {:?}",
                token,
                next.unwrap(),
            )));
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<Expr, SyntaxError> {
        let ast = self.expression()?;
        self.assert_next(Token::End)?;
        Ok(ast)
    }
    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        let next = self.iter.next().unwrap();

        match next {
            Token::Number(n) => Ok(Expr::ValExrp((*n).clone())),
            Token::RightParen => {
                let expr = self.expression()?;
                self.assert_next(Token::LeftParen)?;
                Ok(expr)
            }
            Token::Dash => {
                let expr = self.factor()?;
                Ok(Expr::UnaryExpr(Operator::Negative, Box::new(expr)))
            }
            _ => Err(SyntaxError::new_parse_error(format!(
                "Unexpected token {:?}",
                next
            ))),
        }
    }
    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let expr = self.primary()?;
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr: Expr = self.factor()?;

        loop {
            let next = self.iter.peek().unwrap();
            match next {
                Token::Star => {
                    self.iter.next();
                    let rhs = self.factor()?;
                    expr = Expr::BinExpr(Operator::Multiply, Box::new(expr), Box::new(rhs));
                }
                Token::Slash => {
                    self.iter.next();
                    let rhs = self.factor()?;
                    expr = Expr::BinExpr(Operator::Divide, Box::new(expr), Box::new(rhs));
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr: Expr = self.term()?;

        loop {
            let next = self.iter.peek().unwrap();
            match next {
                Token::Plus => {
                    self.iter.next();
                    let rhs = self.term()?;
                    expr = Expr::BinExpr(Operator::Add, Box::new(expr), Box::new(rhs));
                }
                Token::Dash => {
                    self.iter.next();
                    let rhs = self.term()?;
                    expr = Expr::BinExpr(Operator::Subtract, Box::new(expr), Box::new(rhs));
                }
                _ => break,
            };
        }

        Ok(expr)
    }
}

pub fn eval(line: String) -> Result<(), Box<dyn Error>> {
    let tokens = lex(line)?;
    let mut token_iter: Peekable<Iter<'_, Token>> = tokens.iter().peekable();
    let mut parser = Parser::new(&mut token_iter);
    let result = parser.parse();
    match result {
        Ok(mut ast) => println!("{}", ast.eval()?),
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}

pub fn eval_to_string(input: String) -> Result<String, Box<dyn Error>> {
    let tokens = lex(input)?;
    let mut token_iter: Peekable<Iter<'_, Token>> = tokens.iter().peekable();
    let mut parser = Parser::new(&mut token_iter);
    let mut result = parser.parse()?;
    Ok(result.eval().map(|val| val.to_string())?)
}

fn get_line() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_s) => {}
        Err(_e) => {}
    };
    input.trim().to_string()
}

fn run_repl() -> Result<(), Box<dyn Error>> {
    loop {
        let line = get_line();
        if line == "quit" {
            return Ok(());
        }
        if let Err(e) = eval(line) {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    run_repl()
}
