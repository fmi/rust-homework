extern crate forth;
use forth::*;

#[test]
fn test_basic() {
    let mut interpreter = Interpreter::new();
    interpreter.run("1 2");
    assert_eq!(interpreter.stack().collect::<Vec<_>>(), vec![2, 1]);

    assert_eq!(Interpreter::new().run("1 DUP"), Ok(Some(1)));
    assert_eq!(Interpreter::new().run("1 POP"), Ok(None));
    assert_eq!(Interpreter::new().run("1 2 SWAP"), Ok(Some(1)));

    assert_eq!(Interpreter::new().run("1 1 ADD"), Ok(Some(2)));
    assert_eq!(Interpreter::new().run("1 1 SUB"), Ok(Some(0)));
    assert_eq!(Interpreter::new().run("1 1 MUL"), Ok(Some(1)));
    assert_eq!(Interpreter::new().run("1 1 DIV"), Ok(Some(1)));

    let mut interpreter = Interpreter::new();
    interpreter.def_var("THREE", 3);
    interpreter.def_unary_op("PLUS1", |x| x + 1);
    interpreter.def_binary_op("PLUS", |x, y| x + y);
}
