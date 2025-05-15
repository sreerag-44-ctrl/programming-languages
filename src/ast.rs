// SQL AST components for Rust SQL parser
// Author: Sreerag Devadasan

/// Represents an expression in SQL (e.g., identifiers, numbers, logical operations).
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Number(u64),
    String(String),
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    BinaryOperation {
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    Boolean(bool),
    Null,
    Grouped(Box<Expression>),
}

/// Binary operators used in expressions (e.g., +, -, =, AND).
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Unary operators used in expressions (e.g., NOT, -).
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}
/// Represents a SQL statement (currently only SELECT is supported).
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Select {
        columns: Vec<String>,
        table: String,
        selection: Option<Expression>,
        order_by: Option<Vec<String>>,
        limit: Option<u64>,
    },
}
impl Statement {
    /// Convenience constructor for Select statement
    pub fn new_select(
        columns: Vec<String>,
        table: String,
        selection: Option<Expression>,
        order_by: Option<Vec<String>>,
        limit: Option<u64>,
    ) -> Self {
        Statement::Select {
            columns,
            table,
            selection,
            order_by,
            limit,
        }
    }
}
