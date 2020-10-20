use solution::*;

#[test]
fn test_basic() {
    let expected = vec![1.to_string(), 2.to_string(), String::from("Fizz")];

    assert_eq!(fizzbuzz(3), expected);
    assert_eq!(custom_buzz(3, 3, 5), expected);

    let mut fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz")
        ],
    };
    assert_eq!(fizzbuzzer.take(3), expected);
    fizzbuzzer.change_label(0, &String::from("Fiz"));
}
