// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

#[test]
fn test_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();

    assert_eq!(code_identifier.camelcase(), "someVar");
    assert_eq!(code_identifier.titlecase(), "SomeVar");
    assert_eq!(code_identifier.underscore(), "some_var");
    assert_eq!(code_identifier.kebabcase(), "some-var");
    assert_eq!(code_identifier.screaming_snakecase(), "SOME_VAR");
}
