use crate::tokenizer::{Token, Keyword};
use crate::ast::{Expression, BinaryOperator, UnaryOperator};

pub struct PrattParser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> PrattParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.peek() {
            Some(tok) if tok == expected => {
                self.advance();
                Ok(())
            }
            Some(tok) => Err(format!("Expected token {:?}, but found {:?}", expected, tok)),
            None => Err(format!("Expected token {:?}, but found end of input", expected)),
        }
    }
    // Optional debug method for tracing parsing steps
    #[allow(dead_code)]
   fn debug_print(&self, _message: &str) {
    println!("[DEBUG] {} at position {}", _message, self.position);
}

    pub fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, String> {
        let mut left = match self.advance() {
            Some(Token::Identifier(name)) => Expression::Identifier(name.clone()),
            Some(Token::Number(n)) => Expression::Number(*n),
            Some(Token::String(s)) => Expression::String(s.clone()),
            Some(Token::Keyword(Keyword::True)) => Expression::Boolean(true),
            Some(Token::Keyword(Keyword::False)) => Expression::Boolean(false),
            Some(Token::Keyword(Keyword::Not)) => {
                let expr = self.parse_expression(6)?; // Highest precedence for NOT
                Expression::UnaryOperation {
                    operator: UnaryOperator::Not,
                    operand: Box::new(expr),
                }
            }
            Some(Token::Minus) => {
                let expr = self.parse_expression(6)?;
                Expression::UnaryOperation {
                    operator: UnaryOperator::Negate,
                    operand: Box::new(expr),
                }
            }
            Some(Token::LeftParentheses) => {
                let expr = self.parse_expression(1)?;
                self.expect(&Token::RightParentheses)?;
                Expression::Grouped(Box::new(expr))
            }
            Some(t) => return Err(format!("Unexpected token at start of expression: {:?}", t)),
            None => return Err("Unexpected end of input while parsing expression".to_string()),
        };

        loop {
            let op = match self.peek() {
                Some(tok) if get_precedence(tok) >= min_precedence => tok.clone(),
                _ => break,
            };

            let precedence = get_precedence(&op);
            self.advance(); // consume the operator

            let right = self.parse_expression(precedence + 1)?;

            let operator = match op {
                Token::Equal => BinaryOperator::Equals,
                Token::NotEqual => BinaryOperator::NotEquals,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                Token::LessThan => BinaryOperator::LessThan,
                Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::Keyword(Keyword::And) => BinaryOperator::And,
                Token::Keyword(Keyword::Or) => BinaryOperator::Or,
                _ => return Err(format!("Unknown binary operator: {:?}", op)),
            };

            left = Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator,
                right_operand: Box::new(right),
            };
        }

        Ok(left)
    }
}

fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Keyword(Keyword::Or) => 1,
        Token::Keyword(Keyword::And) => 2,
        Token::Equal | Token::NotEqual => 3,
        Token::GreaterThan | Token::GreaterThanOrEqual |
        Token::LessThan | Token::LessThanOrEqual => 4,
        Token::Plus | Token::Minus => 5,
        Token::Multiply | Token::Divide => 6,
        _ => 0,
    }
}
