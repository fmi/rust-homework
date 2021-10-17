// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_push() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 1",
        "PUSH -25",
        "PUSH 13",
    ]);

    interpreter.forward().unwrap();
    interpreter.forward().unwrap();
    assert_eq!(interpreter.stack, &[1, -25]);

    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[1]);

    interpreter.run().unwrap();
    assert_eq!(interpreter.stack, &[1, -25, 13]);
}

#[test]
fn test_pop() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 1",
        "POP",
        "PUSH 13",
        "POP",
    ]);

    interpreter.forward().unwrap();
    assert_eq!(interpreter.stack.len(), 1);

    interpreter.forward().unwrap();
    assert_eq!(interpreter.stack.len(), 0);

    interpreter.run().unwrap();
    assert_eq!(interpreter.stack.len(), 0);
}

#[test]
fn test_arithmetic_basic() {
    let mut interpreter = Interpreter::new();

    // 2 + 1 = 3
    interpreter.add_instructions(&[
        "PUSH 1",
        "PUSH 2",
        "ADD",
    ]);
    interpreter.run().unwrap();
    assert_eq!(interpreter.stack.last().unwrap(), &3);

    // 3 * 3 = 9
    interpreter.add_instructions(&[
        "PUSH 3",
        "MUL",
    ]);
    interpreter.run().unwrap();
    assert_eq!(interpreter.stack.last().unwrap(), &9);

    // 4 - 9 = -5
    interpreter.add_instructions(&[
        "PUSH 4",
        "SUB",
    ]);
    interpreter.run().unwrap();
    assert_eq!(interpreter.stack.last().unwrap(), &-5);

    // 5 / -5 = -1
    interpreter.add_instructions(&[
        "PUSH 5",
        "DIV",
    ]);
    interpreter.run().unwrap();
    assert_eq!(interpreter.stack.last().unwrap(), &-1);
}

#[test]
fn test_arithmetic_back() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 1",
        "PUSH 2",
        "ADD",
        "PUSH 3",
        "MUL",
        "PUSH 4",
        "SUB",
        "PUSH 5",
        "DIV",
    ]);

    interpreter.run().unwrap();

    // Before 5 / -5 = -1
    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[-5, 5]);

    // Before 4 - 9 = -5
    interpreter.back().unwrap();
    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[9, 4]);

    // Before 3 * 3 = 9
    interpreter.back().unwrap();
    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[3, 3]);

    // Before 2 + 1 = 3
    interpreter.back().unwrap();
    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[1, 2]);
}

#[test]
fn test_div_1() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 7",
        "PUSH 14",
        "DIV",
    ]);

    // 14 / 7 = 2
    interpreter.run().unwrap();
    assert_eq!(interpreter.stack, &[2]);

    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[7, 14]);
}

#[test]
fn test_div_2() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 5",
        "PUSH 7",
        "DIV",
    ]);

    // 7 / 5 = 1
    interpreter.run().unwrap();
    assert_eq!(interpreter.stack, &[1]);

    interpreter.back().unwrap();
    assert_eq!(interpreter.stack, &[5, 7]);
}

#[test]
fn test_errors_1() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&["POP"]);
    assert_eq!(interpreter.forward(), Err(RuntimeError::StackUnderflow));

    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&["PUSH 1"]);
    interpreter.forward().unwrap();
    assert_eq!(interpreter.forward(), Err(RuntimeError::NoInstructions));

    interpreter.back().unwrap();
    assert_eq!(interpreter.back(), Err(RuntimeError::NoInstructions));
}

#[test]
fn test_errors_2() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 0",
        "PUSH 7",
        "DIV",
    ]);

    interpreter.forward().unwrap();
    interpreter.forward().unwrap();

    assert_eq!(interpreter.forward(), Err(RuntimeError::DivideByZero));
}

#[test]
fn test_restoring_instructions() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 1",
        "PUSH 2",
        "ADD",
    ]);

    interpreter.forward().unwrap();
    interpreter.forward().unwrap();
    assert_eq!(interpreter.instructions, &["ADD"]);

    interpreter.back().unwrap();
    assert_eq!(interpreter.instructions, &["PUSH 2", "ADD"]);
}

#[test]
fn test_instructions_after_error() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 0",
        "PUSH 2",
        "DIV",
        "PUSH 3",
    ]);

    interpreter.forward().unwrap();
    interpreter.forward().unwrap();
    assert_eq!(interpreter.forward(), Err(RuntimeError::DivideByZero));
    assert_eq!(interpreter.instructions, &["DIV", "PUSH 3"]);

    interpreter.back().unwrap();
    interpreter.back().unwrap();
    assert_eq!(interpreter.instructions, &["PUSH 0", "PUSH 2", "DIV", "PUSH 3"]);

    *interpreter.current_instruction().unwrap() = String::from("PUSH 1");
    interpreter.run().unwrap();
}

#[test]
fn test_arg_number() {
    let invalid_instructions = [
        "ADD 1 2",
        "ADD 42",
        "SUB -1",
        "DIV 1 3 5",
        "MUL 12",
        "POP 1",
        "PUSH",
        "PUSH 1 2",
    ];

    for instruction in &invalid_instructions {
        let mut interpreter = Interpreter::new();
        interpreter.add_instructions(&[instruction]);

        assert_eq! {
            interpreter.forward(),
            Err(RuntimeError::InvalidCommand),
            "Should have been invalid: {}", instruction
        };
    }
}

#[test]
fn test_invalid_args() {
    let invalid_instructions = [
        "POSH",
        "PULL",
        "VID",
        "ADDIFY",
        "DIVISIVIDE",
        "UNPOP 12",
    ];

    for instruction in &invalid_instructions {
        let mut interpreter = Interpreter::new();
        interpreter.add_instructions(&[instruction]);

        assert_eq! {
            interpreter.forward(),
            Err(RuntimeError::InvalidCommand),
            "Should have been invalid: {}", instruction
        };
    }
}
