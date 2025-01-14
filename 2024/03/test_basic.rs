use solution::*;

#[test]
fn test_basic_simple_parser() {
    // A & B
    let mut simple_parser = SimpleExprParser::new();
    let _ = simple_parser.push_atom('A');
    let _ = simple_parser.push_op('&');
    let _ = simple_parser.push_atom('B');
    let expr = simple_parser.finish().unwrap();

    eval(&expr, &['A'], &['B']);
}

#[test]
fn test_basic_expr_parser() {
    // A & (B | !C)
    let mut full_parser = ExprParser::new();
    let _ = full_parser.push_atom('A');
    let _ = full_parser.push_op('&');
    let _ = full_parser.open_paren();
    let _ = full_parser.push_atom('B');
    let _ = full_parser.push_op('|');
    let _ = full_parser.push_op('!');
    let _ = full_parser.push_atom('C');
    let _ = full_parser.close_paren();
    let expr = full_parser.finish().unwrap();

    eval(&expr, &['A'], &['B']);
}

#[test]
fn test_basic_errors() {
    let mut simple_parser = SimpleExprParser::new();
    let _ = simple_parser.push_atom('A');
    let _ = simple_parser.push_op('&');
    assert_eq!(simple_parser.push_op('&'), Err(ParseError::UnexpectedBinOp));

    let mut simple_parser = SimpleExprParser::new();
    let _ = simple_parser.push_atom('A');
    let _ = simple_parser.push_op('&');
    let _ = simple_parser.push_atom('B');
    assert_eq!(simple_parser.push_atom('B'), Err(ParseError::UnexpectedExpr));
}
