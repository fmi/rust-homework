use solution::*;

macro_rules! expr {
    ( atom($c:expr) ) => {
        Expr::Atom($c)
    };
    ( not( $tag:ident ( $($e:tt)* ) ) ) => {
        Expr::Not(Box::new(expr!( $tag($( $e )*) )))
    };
    ( and( $( $tag:ident ( $($e:tt)* ) ),* ) ) => {
        Expr::And(vec![$( expr!($tag($( $e )*)) ),*])
    };
    ( or( $( $tag:ident ( $($e:tt)* ) ),* ) ) => {
        Expr::Or(vec![$( expr!($tag($( $e )*)) ),*])
    };
}

// помощна функция
fn feed_simple(parser: &mut SimpleExprParser, text: &str) -> Result<(), ParseError> {
    for c in text.chars() {
        match c {
            ' ' => {}
            '&' | '|' | '!' => parser.push_op(c)?,
            _ => parser.push_atom(c)?,
        }
    }

    Ok(())
}


// помощна функция
fn feed_full(parser: &mut ExprParser, text: &str) -> Result<(), ParseError> {
    for c in text.chars() {
        match c {
            ' ' => {}
            '&' | '|' | '!' => parser.push_op(c)?,
            '(' => parser.open_paren()?,
            ')' => parser.close_paren()?,
            _ => parser.push_atom(c)?,
        }
    }

    Ok(())
}

#[test]
fn test_parser_atom() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(atom('A')));
}

#[test]
fn test_parser_and_or() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), atom('B'))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A | B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(atom('A'), atom('B'))));
}

#[test]
fn test_parser_not() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(not(atom('B'))));
}

#[test]
fn test_parser_expr_and_not() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & !B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), not(atom('B')))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!A | B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(not(atom('A')), atom('B'))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!A & !B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(not(atom('A')), not(atom('B')))));
}

#[test]
fn test_parser_multiple_atoms_same_op() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & B & C").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), atom('B'), atom('C'))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "X | Y | Z | W").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(atom('X'), atom('Y'), atom('Z'), atom('W')))
    );
}

#[test]
fn test_parser_multiple_not() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!!!B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(not(not(not(atom('B'))))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A | !!B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(atom('A'), not(not(atom('B'))))));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!!A | B").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(not(not(atom('A'))), atom('B'))));
}

#[test]
fn test_parser_alternating_ops() {
    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A & B | C & D").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(or(and(atom('A'), atom('B')), atom('C')), atom('D')))
    );

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A | B & C | D").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(and(or(atom('A'), atom('B')), atom('C')), atom('D')))
    );
}

#[test]
fn test_paren_around_expr() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(atom('A')));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A & B)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(and(atom('A'), atom('B'))));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A | B)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(or(atom('A'), atom('B'))));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(!A)").unwrap();
    assert_eq!(parser.finish().unwrap(), expr!(not(atom('A'))));
}

#[test]
fn test_paren_expr_priority() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "X | (A & B)").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(atom('X'), and(atom('A'), atom('B'))))
    );

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A | B) & X").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(or(atom('A'), atom('B')), atom('X')))
    );
}

#[test]
fn test_paren_not() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "X & !(A | B)").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(and(atom('X'), not(or(atom('A'), atom('B')))))
    );

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "!(A | B) | X").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(not(or(atom('A'), atom('B'))), atom('X')))
    );
}

#[test]
fn test_paren_surrounded() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "X | (A & B) | (C & D) | Y").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(or(
            atom('X'),
            and(atom('A'), atom('B')),
            and(atom('C'), atom('D')),
            atom('Y')
        ))
    );
}

#[test]
fn test_paren_nested() {
    let mut parser = ExprParser::new();
    feed_full(&mut parser, "!(A & !(B & !(C & D)))").unwrap();
    assert_eq!(
        parser.finish().unwrap(),
        expr!(not(and(atom('A'), not(and(atom('B'), not(and(atom('C'), atom('D'))))))))
    );
}


#[test]
fn test_parser_errors_basic() {
    let mut parser = SimpleExprParser::new();
    assert!(matches!(parser.push_op('&'), Err(_)));

    let mut parser = SimpleExprParser::new();
    assert!(matches!(parser.push_op('|'), Err(_)));

    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    assert!(matches!(parser.push_atom('B'), Err(_)));

    let mut parser = SimpleExprParser::new();
    parser.push_atom('A').unwrap();
    assert!(matches!(parser.push_op('!'), Err(_)));

    let mut parser = SimpleExprParser::new();
    parser.push_op('!').unwrap();
    parser.push_atom('A').unwrap();
    assert!(matches!(parser.push_atom('B'), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A &").unwrap();
    assert!(matches!(parser.push_op('&'), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A &").unwrap();
    assert!(matches!(parser.push_op('|'), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A |").unwrap();
    assert!(matches!(parser.push_op('&'), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A |").unwrap();
    assert!(matches!(parser.push_op('|'), Err(_)));
}

#[test]
fn test_parser_error_unexpected_end() {
    let mut parser = SimpleExprParser::new();
    assert!(matches!(parser.finish(), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A &").unwrap();
    assert!(matches!(parser.finish(), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "A |").unwrap();
    assert!(matches!(parser.finish(), Err(_)));

    let mut parser = SimpleExprParser::new();
    feed_simple(&mut parser, "!").unwrap();
    assert!(matches!(parser.finish(), Err(_)));
}

#[test]
fn test_error_paren_mismatched() {
    let mut parser = ExprParser::new();
    assert!(matches!(parser.close_paren(), Err(_)));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    assert!(matches!(parser.close_paren(), Err(_)));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    parser.push_atom('A').unwrap();
    parser.push_op('&').unwrap();
    assert!(matches!(parser.close_paren(), Err(_)));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    parser.push_op('!').unwrap();
    assert!(matches!(parser.close_paren(), Err(_)));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    assert!(matches!(parser.finish(), Err(_)));

    let mut parser = ExprParser::new();
    parser.open_paren().unwrap();
    parser.push_atom('A').unwrap();
    assert!(matches!(parser.finish(), Err(_)));

    let mut parser = ExprParser::new();
    parser.push_atom('A').unwrap();
    assert!(matches!(
        parser.open_paren(),
        Err(_)
    ));

    let mut parser = ExprParser::new();
    feed_full(&mut parser, "(A & B)").unwrap();
    assert!(matches!(
        parser.open_paren(),
        Err(_)
    ));
}

#[test]
fn test_eval_full() {
    assert_eq!(eval(&expr!(atom('A')), &['A'], &[]), Value::True);
    assert_eq!(eval(&expr!(atom('A')), &[], &['A']), Value::False);

    assert_eq!(eval(&expr!(not(atom('B'))), &['A'], &['B']), Value::True);
    assert_eq!(eval(&expr!(not(atom('B'))), &['B'], &['A']), Value::False);

    assert_eq!(eval(&expr!(and(atom('A'), atom('B'))), &['A', 'B'], &[]), Value::True);
    assert_eq!(eval(&expr!(and(atom('A'), atom('B'))), &['A'], &['B']), Value::False);
    assert_eq!(eval(&expr!(or(atom('A'), atom('B'))), &['A'], &['B']), Value::True);
    assert_eq!(eval(&expr!(or(atom('A'), atom('B'))), &[], &['A', 'B']), Value::False);
}

#[test]
fn test_eval_not_and_or() {
    assert_eq!(
        eval(&expr!(not(and(atom('A'), atom('B')))), &['A', 'B'], &[]),
        Value::False
    );
    assert_eq!(
        eval(&expr!(not(and(atom('A'), atom('B')))), &['A'], &['B']),
        Value::True
    );
    assert_eq!(
        eval(&expr!(not(or(atom('A'), atom('B')))), &['A'], &['B']),
        Value::False
    );
    assert_eq!(
        eval(&expr!(not(or(atom('A'), atom('B')))), &[], &['A', 'B']),
        Value::True
    );
}

#[test]
fn test_eval_partial() {
    assert_eq!(eval(&expr!(atom('A')), &[], &[]), Value::Expr(expr!(atom('A'))));
    assert_eq!(
        eval(&expr!(not(atom('B'))), &[], &[]),
        Value::Expr(expr!(not(atom('B'))))
    );

    assert_eq!(
        eval(&expr!(and(atom('A'), atom('B'), atom('C'))), &['B'], &[]),
        Value::Expr(expr!(and(atom('A'), atom('C'))))
    );
    assert_eq!(
        eval(&expr!(and(atom('A'), atom('B'), atom('C'))), &[], &['B']),
        Value::False
    );

    assert_eq!(
        eval(&expr!(or(atom('A'), atom('B'), atom('C'))), &['B'], &[]),
        Value::True
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), atom('B'), atom('C'))), &[], &['B']),
        Value::Expr(expr!(or(atom('A'), atom('C'))))
    );

    assert_eq!(
        eval(&expr!(and(atom('A'), not(atom('B')), atom('C'))), &[], &['B']),
        Value::Expr(expr!(and(atom('A'), atom('C'))))
    );
    assert_eq!(
        eval(&expr!(and(atom('A'), not(atom('B')), atom('C'))), &['B'], &[]),
        Value::False
    );

    assert_eq!(
        eval(&expr!(or(atom('A'), not(atom('B')), atom('C'))), &[], &['B']),
        Value::True
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), not(atom('B')), atom('C'))), &['B'], &[]),
        Value::Expr(expr!(or(atom('A'), atom('C'))))
    );
}

#[test]
fn test_eval_unwrap_and_or() {
    assert_eq!(
        eval(&expr!(and(atom('A'), atom('B'), atom('C'))), &['A', 'C'], &[]),
        Value::Expr(expr!(atom('B')))
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), atom('B'), atom('C'))), &[], &['A', 'C']),
        Value::Expr(expr!(atom('B')))
    );

    assert_eq!(
        eval(&expr!(and(atom('A'), not(atom('B')), atom('C'))), &['A', 'C'], &[]),
        Value::Expr(expr!(not(atom('B'))))
    );
    assert_eq!(
        eval(&expr!(or(atom('A'), not(atom('B')), atom('C'))), &[], &['A', 'C']),
        Value::Expr(expr!(not(atom('B'))))
    );
}

#[test]
fn test_eval_unwrap_nested() {
    assert_eq!(
        eval(
            &expr!(or(
                atom('X'),
                and(atom('A'), atom('B')),
                not(and(atom('C'), atom('D'))),
                atom('Y')
            )),
            &['A', 'C'],
            &[]
        ),
        Value::Expr(expr!(or(atom('X'), atom('B'), not(atom('D')), atom('Y'))))
    );
}
