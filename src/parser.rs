use crate::tokenizer::{Token, Keyword};
use crate::ast::{Statement};
use crate::pratt::PrattParser;
use crate::tokenizer::ParseError;

pub struct SQLParser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> SQLParser<'a> {
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

    fn expect_keyword(&mut self, keyword: Keyword) -> Result<(), ParseError> {
        match self.advance() {
            Some(Token::Keyword(k)) if *k == keyword => Ok(()),
            Some(_tok) => Err(ParseError::ExpectedKeyword(format!("{:?}", keyword))),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            Some(_) => Err(ParseError::ExpectedIdentifier),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    #[allow(dead_code)]
    fn debug_print(&self, message: &str) {
    println!("[DEBUG] {} at position {}", message, self.position);
}


    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            Some(Token::Keyword(Keyword::Select)) => self.parse_select(),
            Some(tok) => Err(ParseError::UnknownStartOfStatement(format!("Unexpected start of statement: {:?}", tok))),
            None => Err(ParseError::General("Empty input".to_string())),
        }
    }

    fn parse_select(&mut self) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Select)?;

        let mut columns = Vec::new();

        // Parse column list until we hit FROM
        loop {
            match self.advance() {
                Some(Token::Identifier(name)) => columns.push(name.clone()),
                Some(Token::Comma) => continue,
                Some(Token::Keyword(Keyword::From)) => break,
                Some(tok) => {
                    return Err(ParseError::General(format!("Unexpected token in column list: {:?}", tok)))
                }
                None => {
                    return Err(ParseError::General("Unexpected end of input while reading columns.".to_string()))
                }
            }
        }

        let table = self.expect_identifier()?;
        let mut selection = None;

        // Handle optional WHERE clause
        if let Some(Token::Keyword(Keyword::Where)) = self.peek() {
            self.advance(); // consume WHERE
            let remaining_tokens = &self.tokens[self.position..];
            let mut expr_parser = PrattParser::new(remaining_tokens);
            let expr = expr_parser
                .parse_expression(1)
                .map_err(ParseError::InvalidExpression)?;
            selection = Some(expr);
        }

        // Handle optional ORDER BY clause
        let mut order_by = None;

        if let Some(Token::Keyword(Keyword::Order)) = self.peek() {
            self.advance(); // consume ORDER
            self.expect_keyword(Keyword::By)?; // expect BY

            let mut order_columns = Vec::new();

            loop {
                match self.advance() {
                    Some(Token::Identifier(name)) => order_columns.push(name.clone()),
                    Some(Token::Comma) => continue,
                    Some(Token::Semicolon) | Some(Token::Eof) => break,
                    Some(tok) => {
                        return Err(ParseError::General(format!("Unexpected token in ORDER BY: {:?}", tok)))
                    }
                    None => return Err(ParseError::UnexpectedEnd),
                }
            }

            order_by = Some(order_columns);
        }

Ok(Statement::Select {
    columns,
    table,
    selection,
    order_by,
    limit: None,
})





    }
}