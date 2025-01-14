use std::mem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(char),
    Not(Box<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedExpr,
    UnexpectedUnaryOp,
    UnexpectedBinOp,
    UnexpectedParen,
    UnexpectedEnd,
}

enum Op {
    And,
    Or,
}

#[derive(Default, PartialEq, Eq)]
enum Pending {
    #[default]
    Expr,
    Op,
}

impl Pending {
    fn expect_expr_or(&self, err: ParseError) -> Result<(), ParseError> {
        match self {
            Pending::Expr => Ok(()),
            Pending::Op => Err(err),
        }
    }

    fn expect_op_or(&self, err: ParseError) -> Result<(), ParseError> {
        match self {
            Pending::Op => Ok(()),
            Pending::Expr => Err(err),
        }
    }
}

/// Парсър за прост израз, който не съдържа скоби
#[derive(Default)]
pub struct SimpleExprParser {
    exprs: Vec<Expr>,
    exprs_op: Option<Op>,
    not_count: usize,
    pending_token: Pending,
}

impl SimpleExprParser {
    pub fn new() -> SimpleExprParser {
        SimpleExprParser::default()
    }

    /// Приема атом.
    ///
    /// `c` ще бъде валиден символ за атом.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_atom(&mut self, c: char) -> Result<(), ParseError> {
        self.push_expr(Expr::Atom(c))
    }

    /// Приема символ за операция.
    ///
    /// `op` ще бъде едно от '&', '|', '!'.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_op(&mut self, op: char) -> Result<(), ParseError> {
        match op {
            '&' => {
                self.pending_token.expect_op_or(ParseError::UnexpectedBinOp)?;
                self.pending_token = Pending::Expr;

                match self.exprs_op.replace(Op::And) {
                    None | Some(Op::And) => {}
                    Some(Op::Or) => {
                        let old_exprs = mem::replace(&mut self.exprs, vec![]);
                        self.exprs.push(Expr::Or(old_exprs));
                    }
                }
            }
            '|' => {
                self.pending_token.expect_op_or(ParseError::UnexpectedBinOp)?;
                self.pending_token = Pending::Expr;

                match self.exprs_op.replace(Op::Or) {
                    None | Some(Op::Or) => {}
                    Some(Op::And) => {
                        let old_exprs = mem::replace(&mut self.exprs, vec![]);
                        self.exprs.push(Expr::And(old_exprs));
                    }
                }
            }
            '!' => {
                self.pending_token.expect_expr_or(ParseError::UnexpectedUnaryOp)?;
                self.not_count += 1;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn push_expr(&mut self, expr: Expr) -> Result<(), ParseError> {
        self.pending_token.expect_expr_or(ParseError::UnexpectedExpr)?;

        let not_expr = (0..self.not_count).fold(expr, |acc, _| Expr::Not(Box::new(acc)));

        match (not_expr, &self.exprs_op) {
            (Expr::And(and_list), Some(Op::And) | None) => {
                self.exprs.extend(and_list);
                self.exprs_op = Some(Op::And);
            }
            (Expr::Or(or_list), Some(Op::Or) | None) => {
                self.exprs.extend(or_list);
                self.exprs_op = Some(Op::Or);
            }
            (other_expr, _) => self.exprs.push(other_expr),
        }

        self.pending_token = Pending::Op;
        self.not_count = 0;

        Ok(())
    }

    /// Завършва парсването и връща построения израз.
    pub fn finish(self) -> Result<Expr, ParseError> {
        self.pending_token.expect_op_or(ParseError::UnexpectedEnd)?;

        match self.exprs_op {
            None => match self.exprs.len() {
                1 => Ok(self.exprs.into_iter().next().unwrap()),
                _ => unreachable!(),
            },
            Some(Op::And) => Ok(Expr::And(self.exprs)),
            Some(Op::Or) => Ok(Expr::Or(self.exprs)),
        }
    }
}

/// Парсър за пълния израз
#[derive(Default)]
pub struct ExprParser {
    stack: Vec<SimpleExprParser>,
}

impl ExprParser {
    pub fn new() -> ExprParser {
        ExprParser {
            stack: vec![SimpleExprParser::default()],
        }
    }

    /// Приема атом.
    ///
    /// `c` ще бъде валиден символ за атом.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_atom(&mut self, c: char) -> Result<(), ParseError> {
        self.stack.last_mut().unwrap().push_atom(c)
    }

    /// Приема символ за операция.
    ///
    /// `op` ще бъде едно от '&', '|', '!'.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_op(&mut self, op: char) -> Result<(), ParseError> {
        self.stack.last_mut().unwrap().push_op(op)
    }

    /// Приема отваряща скоба.
    pub fn open_paren(&mut self) -> Result<(), ParseError> {
        self.stack.last_mut().unwrap().pending_token.expect_expr_or(ParseError::UnexpectedExpr)?;
        self.stack.push(SimpleExprParser::default());
        Ok(())
    }

    /// Приема затваряща скоба.
    pub fn close_paren(&mut self) -> Result<(), ParseError> {
        if self.stack.len() < 2 {
            return Err(ParseError::UnexpectedParen);
        }

        let level = self.stack.pop().unwrap();
        let expr = level.finish()
            .map_err(|_| ParseError::UnexpectedParen)?;

        self.stack.last_mut().unwrap().push_expr(expr).unwrap();
        Ok(())
    }

    /// Завършва парсването и връща построения израз.
    pub fn finish(mut self) -> Result<Expr, ParseError> {
        if self.stack.len() != 1 {
            return Err(ParseError::UnexpectedEnd);
        }

        match self.stack.pop() {
            None => panic!(),
            Some(level) => level.finish(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    True,
    False,
    Expr(Expr),
}

pub fn eval(expr: &Expr, truthy: &[char], falsy: &[char]) -> Value {
    match expr {
        Expr::Atom(c) => {
            if truthy.contains(c) {
                Value::True
            } else if falsy.contains(c) {
                Value::False
            } else {
                Value::Expr(Expr::Atom(*c))
            }
        }
        Expr::Not(expr) => match eval(expr, truthy, falsy) {
            Value::True => Value::False,
            Value::False => Value::True,
            Value::Expr(e) => Value::Expr(Expr::Not(Box::new(e))),
        },
        Expr::And(list) => 'blk: {
            let mut new_list = vec![];
            for expr in list {
                match eval(expr, truthy, falsy) {
                    Value::True => {}
                    Value::False => break 'blk Value::False,
                    Value::Expr(expr) => new_list.push(expr),
                }
            }

            match new_list.len() {
                0 => Value::True,
                1 => Value::Expr(new_list.into_iter().next().unwrap()),
                _ => Value::Expr(Expr::And(new_list)),
            }
        }
        Expr::Or(list) => 'blk: {
            let mut new_list = vec![];
            for expr in list {
                match eval(expr, truthy, falsy) {
                    Value::False => {}
                    Value::True => break 'blk Value::True,
                    Value::Expr(expr) => new_list.push(expr),
                }
            }

            match new_list.len() {
                0 => Value::False,
                1 => Value::Expr(new_list.into_iter().next().unwrap()),
                _ => Value::Expr(Expr::Or(new_list)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_nested_same_op_left() {
        // (A & B) & C
        let mut parser = ExprParser::new();
        parser.open_paren().unwrap();
        parser.push_atom('A').unwrap();
        parser.push_op('&').unwrap();
        parser.push_atom('B').unwrap();
        parser.close_paren().unwrap();
        parser.push_op('&').unwrap();
        parser.push_atom('C').unwrap();

        assert_eq!(
            parser.finish().unwrap(),
            Expr::And(vec![Expr::Atom('A'), Expr::Atom('B'), Expr::Atom('C')]),
        );
    }

    #[test]
    fn test_merge_nested_same_op_right() {
        // A & (B & C)
        let mut parser = ExprParser::new();
        parser.push_atom('A').unwrap();
        parser.push_op('&').unwrap();
        parser.open_paren().unwrap();
        parser.push_atom('B').unwrap();
        parser.push_op('&').unwrap();
        parser.push_atom('C').unwrap();
        parser.close_paren().unwrap();

        assert_eq!(
            parser.finish().unwrap(),
            Expr::And(vec![Expr::Atom('A'), Expr::Atom('B'), Expr::Atom('C')]),
        );
    }
}
