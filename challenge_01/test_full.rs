// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_full() {
    assert_eq!(fib(0), 1);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1 + 1);
    assert_eq!(fib(3), 1 + 2);
    assert_eq!(fib(4), 2 + 3);
    assert_eq!(fib(5), 3 + 5);
    assert_eq!(fib(6), 5 + 8);
    assert_eq!(fib(7), 8 + 13);
    assert_eq!(fib(8), 13 + 21);
    assert_eq!(fib(9), 21 + 34);
    assert_eq!(fib(10), 34 + 55);
    assert_eq!(fib(11), 55 + 89);
    assert_eq!(fib(12), 89 + 144);
}
