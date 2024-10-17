use solution::*;

// За тестване че някакъв резултат пасва на някакъв pattern:
macro_rules! assert_match {
    ($expr:expr, $pat:pat) => {
        let result = $expr;

        if let $pat = result {
            // all good
        } else {
            assert!(false, "Expression {:?} does not match the pattern {:?}", result, stringify!($pat));
        }
    }
}

#[test]
fn test_basic_1() {
    // Забележете `b""` -- низ от байтове
    let input: &[u8] = b"1\n2\n";
    let mut output = Vec::<u8>::new();
    let mut interpreter = Interpreter::new(input, &mut output);

    interpreter.add("10 READ A").unwrap();
    interpreter.add("20 READ B").unwrap();
    interpreter.add("30 PRINT A").unwrap();
    interpreter.add("40 PRINT B").unwrap();

    interpreter.run().unwrap();
    assert_eq!(interpreter.eval_value("A").unwrap(), 1_u16);
    assert_eq!(String::from_utf8(output).unwrap(), "1\n2\n");
}

#[test]
fn test_basic_2() {
    let input: &[u8] = b"";
    let mut output = Vec::<u8>::new();
    let mut interpreter = Interpreter::new(input, &mut output);

    assert_match!(interpreter.add("10 PRINT"), Err(InterpreterError::SyntaxError { .. }));
}

struct NotBuffered {}

impl std::io::Read for NotBuffered {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

#[test]
fn test_not_buffered() {
    let input = NotBuffered {};
    let mut output = Vec::<u8>::new();
    let mut interpreter = Interpreter::new(input, &mut output);

    interpreter.add("10 PRINT 10").unwrap();
    interpreter.run().unwrap();
}
