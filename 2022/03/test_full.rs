use solution::*;

use std::io::{self, Read, Write};

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

// За да избегнем infinite loops :/
macro_rules! timeout {
    ($time:expr, $body:block) => {
        use std::panic::catch_unwind;

        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            if let Err(e) = catch_unwind(|| $body) {
                sender.send(Err(e)).unwrap();
                return;
            }

            match sender.send(Ok(())) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });

        if let Err(any) = receiver.recv_timeout(std::time::Duration::from_millis($time)).unwrap() {
            panic!("{}", any.downcast_ref::<String>().unwrap());
        }
    }
}

// За тестване на IO грешки:
struct ErroringReader {}

impl Read for ErroringReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "read error!"))
    }
}

struct ErroringWriter {}

impl Write for ErroringWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "Write Error!"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "Flush Error!"))
    }
}

#[test]
fn test_basic_print() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 PRINT 42").unwrap();
            interpreter.add("20 PRINT 24").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "42\n24\n");
    });
}

#[test]
fn test_line_order_and_overwriting() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("20 PRINT 2").unwrap();
            interpreter.add("10 PRINT 1").unwrap();
            interpreter.add("30 PRINT 3").unwrap();
            interpreter.add("30 PRINT 4").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "1\n2\n4\n");
    });
}

#[test]
fn test_basic_input() {
    timeout!(1000, {
        let input: &[u8] = b"13\n";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 READ N").unwrap();
            interpreter.add("20 PRINT N").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "13\n");
    });
}

#[test]
fn test_basic_read() {
    timeout!(1000, {
        let input: &[u8] = b"1\n2\n3\n";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 READ A").unwrap();
            interpreter.add("20 READ B").unwrap();
            interpreter.add("30 READ A").unwrap();
            interpreter.add("40 PRINT A").unwrap();
            interpreter.add("50 PRINT B").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "3\n2\n");
    });
}

#[test]
fn test_print_vars_and_strings() {
    timeout!(1000, {
        let input: &[u8] = b"1\n2\n";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 READ A").unwrap();
            interpreter.add("20 READ Abc").unwrap();
            interpreter.add("30 PRINT A").unwrap();
            interpreter.add("40 PRINT Abc").unwrap();
            interpreter.add("50 PRINT abc").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "1\n2\nabc\n");
    });
}

#[test]
fn test_print_cyrillic() {
    timeout!(1000, {
        let input: &[u8] = b"37\n999\n";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 READ Щ").unwrap();
            interpreter.add("20 READ Юнак").unwrap();
            interpreter.add("30 PRINT Щ").unwrap();
            interpreter.add("40 PRINT Юнак").unwrap();
            interpreter.add("50 PRINT евала").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "37\n999\nевала\n");
    });
}

#[test]
fn test_basic_goto() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 PRINT 1").unwrap();
            interpreter.add("20 GOTO 40").unwrap();
            interpreter.add("30 PRINT 2").unwrap();
            interpreter.add("40 PRINT 3").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "1\n3\n");
    });
}

#[test]
fn test_erroring_goto() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);

        interpreter.add("10 PRINT 1").unwrap();
        interpreter.add("20 GOTO 999").unwrap();
        assert_match!(interpreter.run(), Err(InterpreterError::RuntimeError { line_number: 20, .. }));
    });
}

#[test]
fn test_basic_if() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 IF 2 > 1 GOTO 30").unwrap();
            interpreter.add("20 PRINT 0").unwrap();
            interpreter.add("30 PRINT 1").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "1\n");
    });
}

#[test]
fn test_full_program() {
    timeout!(1000, {
        let input: &[u8] = b"13\n97\n42\n";
        let mut output = Vec::<u8>::new();
        {
            let mut interpreter = Interpreter::new(input, &mut output);

            interpreter.add("10 READ Guess").unwrap();
            interpreter.add("20 IF Guess > 42 GOTO 100").unwrap();
            interpreter.add("30 IF Guess < 42 GOTO 200").unwrap();
            interpreter.add("40 IF Guess = 42 GOTO 300").unwrap();
            interpreter.add("100 PRINT too_high").unwrap();
            interpreter.add("110 GOTO 10").unwrap();
            interpreter.add("200 PRINT too_low").unwrap();
            interpreter.add("210 GOTO 10").unwrap();
            interpreter.add("300 PRINT you_got_it!").unwrap();
            interpreter.run().unwrap();
        }

        assert_eq!(String::from_utf8(output).unwrap(), "too_low\ntoo_high\nyou_got_it!\n");
    });
}

#[test]
fn test_syntax_errors_1() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);

        assert_match!(interpreter.add("10 READ guess"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 READ 13"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 READ ..."), Err(InterpreterError::SyntaxError { .. }));

        assert_match!(interpreter.add("10 GOTO 123456789"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 GOTO -3"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 GOTO some_line"), Err(InterpreterError::SyntaxError { .. }));

        assert_match!(interpreter.add("10 IF 2 > 1 GOTO 987654321"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 IF 1 < 2 GOTO -17"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 IF 3 = 3 GOTO other_line"), Err(InterpreterError::SyntaxError { .. }));
    });
}

#[test]
fn test_syntax_errors_2() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);

        assert_match!(interpreter.add("10 PRINT A B"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 PRINT"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("PRINT 13"), Err(InterpreterError::SyntaxError { .. }));

        assert_match!(interpreter.add("10 READ A B"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 READ"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("READ A"), Err(InterpreterError::SyntaxError { .. }));

        assert_match!(interpreter.add("10 GOTO 1 2"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 GOTO"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("GOTO 100"), Err(InterpreterError::SyntaxError { .. }));

        assert_match!(interpreter.add("10 IF 2 > 1 GOTO"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 IF 1 < 2"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("10 3 = 3 GOTO 10"), Err(InterpreterError::SyntaxError { .. }));
        assert_match!(interpreter.add("IF 3 = 3 GOTO 10"), Err(InterpreterError::SyntaxError { .. }));
    });
}

#[test]
fn test_runtime_errors() {
    timeout!(1000, {
        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);
        interpreter.add("10 PRINT 13").unwrap();
        interpreter.add("11 PRINT A").unwrap();
        assert_match!(interpreter.run(), Err(InterpreterError::RuntimeError { line_number: 11, .. }));

        let input: &[u8] = b"1\nfoobar\n";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);
        interpreter.add("30 READ A").unwrap();
        interpreter.add("40 READ B").unwrap();
        interpreter.add("50 READ C").unwrap();
        assert_match!(interpreter.run(), Err(InterpreterError::RuntimeError { line_number: 40, .. }));

        let input: &[u8] = b"";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);
        interpreter.add("10 GOTO 123").unwrap();
        interpreter.add("123 GOTO 1000").unwrap();
        assert_match!(interpreter.run(), Err(InterpreterError::RuntimeError { line_number: 123, .. }));

        let input: &[u8] = b"1\n2\n";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);
        interpreter.add("11 READ A").unwrap();
        interpreter.add("22 READ B").unwrap();
        interpreter.add("33 IF A = C GOTO 1000").unwrap();
        assert_match!(interpreter.run(), Err(InterpreterError::RuntimeError { line_number: 33, .. }));

        let input: &[u8] = b"2\n2\n";
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);
        interpreter.add("11 READ A").unwrap();
        interpreter.add("22 READ B").unwrap();
        interpreter.add("33 IF A = B GOTO 1000").unwrap();
        assert_match!(interpreter.run(), Err(InterpreterError::RuntimeError { line_number: 33, .. }));
    });
}

#[test]
fn test_io_error_read() {
    timeout!(1000, {
        let input = ErroringReader {};
        let mut output = Vec::<u8>::new();
        let mut interpreter = Interpreter::new(input, &mut output);

        interpreter.add("10 PRINT this_is_fine").unwrap();
        interpreter.add("20 READ Oh_No").unwrap();

        assert_match!(interpreter.run(), Err(InterpreterError::IoError(_)));
    });
}

#[test]
fn test_io_error_write() {
    timeout!(1000, {
        let input: &[u8] = b"13\n";
        let mut output = ErroringWriter {};
        let mut interpreter = Interpreter::new(input, &mut output);

        interpreter.add("10 READ ThisIsFine").unwrap();
        interpreter.add("20 PRINT ThisIsFine").unwrap();

        assert_match!(interpreter.run(), Err(InterpreterError::IoError(_)));
    });
}

