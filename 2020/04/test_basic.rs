// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_basic() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 1",
        "PUSH 2",
        "PUSH 3",
        "ADD",
    ]);

    assert_eq!(interpreter.instructions, &[
        "PUSH 1",
        "PUSH 2",
        "PUSH 3",
        "ADD",
    ]);
    assert_eq!(interpreter.stack, &[]);

    interpreter.forward().unwrap();
    interpreter.forward().unwrap();
    interpreter.forward().unwrap();

    assert_eq!(interpreter.instructions, &["ADD"]);
    assert_eq!(interpreter.stack, &[1, 2, 3]);

    interpreter.run().unwrap();

    assert_eq!(interpreter.instructions.len(), 0);
    assert_eq!(interpreter.stack, &[1, 5]);

    interpreter.back().unwrap();
    interpreter.back().unwrap();

    assert_eq!(interpreter.instructions, &[
        "PUSH 3",
        "ADD",
    ]);
    assert_eq!(interpreter.stack, &[1, 2]);

    interpreter.add_instructions(&["ADD", "ADD"]);

    assert_eq!(interpreter.run(), Err(RuntimeError::StackUnderflow));
    assert_eq!(interpreter.current_instruction().unwrap(), "ADD");
}
