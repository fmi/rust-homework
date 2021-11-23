// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

macro_rules! i32_cell_vec {
    ($($x : expr), + $(,) ?) => {
        vec![ $($x),* ].into_iter().map(Cell).collect::<Vec<Cell<i32>>>()
    }
}

macro_rules! string_cell_vec {
    ($($x : expr), + $(,) ?) => {
        vec![ $($x),* ].into_iter().map(String::from).map(Cell).collect::<Vec<Cell<String>>>()
    }
}

#[test]
fn test_iterating_i32s() {
    assert_eq!(Matrix::new(&[1, 2, 3, 4]).by_row(), i32_cell_vec![1, 2, 3, 4]);
    assert_eq!(Matrix::new(&[13, 42, 37, 24]).by_row(), i32_cell_vec![13, 42, 37, 24]);

    assert_eq!(Matrix::new(&[1, 2, 3, 4]).by_col(), i32_cell_vec![1, 3, 2, 4]);
    assert_eq!(Matrix::new(&[13, 42, 37, 24]).by_col(), i32_cell_vec![13, 37, 42, 24]);
}

#[test]
fn test_iterating_strings() {
    let matrix = Matrix::new(&[String::from("a"), String::from("b"), String::from("c"), String::from("d")]);
    assert_eq!(matrix.by_row(), string_cell_vec!["a", "b", "c", "d"]);

    let matrix = Matrix::new(&[String::from("a"), String::from("b"), String::from("c"), String::from("d")]);
    assert_eq!(matrix.by_col(), string_cell_vec!["a", "c", "b", "d"]);
}

#[test]
fn test_adding_int_and_string_positive() {
    let cell1 = Cell(4);
    let cell2 = Cell(String::from("badger"));

    assert_eq!((cell1 + cell2).0, String::from("4 badger"));
}

#[test]
fn test_adding_int_and_string_zero() {
    let cell1 = Cell(0);
    let cell2 = Cell(String::from("badger"));

    assert_eq!((cell1 + cell2).0, String::from("0 badger"));
}

#[test]
fn test_adding_int_and_string_negative() {
    let cell1 = Cell(-2);
    let cell2 = Cell(String::from("badger"));

    assert_eq!((cell1 + cell2).0, String::from("regdab 2"));
}

#[test]
fn test_adding_int_and_string_unicode() {
    let string_cell = Cell(String::from("опа"));

    assert_eq!((Cell(0) + string_cell.clone()).0, String::from("0 опа"));
    assert_eq!((Cell(2) + string_cell.clone()).0, String::from("2 опа"));
    assert_eq!((Cell(-3) + string_cell.clone()).0, String::from("апо 3"));
}

#[test]
fn test_multiplying_int_and_string_positive() {
    let cell1 = Cell(4);
    let cell2 = Cell(String::from("badger"));

    assert_eq!((cell1 * cell2).0, String::from("badgerbadgerbadgerbadger"));
}

#[test]
fn test_multiplying_int_and_string_zero() {
    let cell1 = Cell(0);
    let cell2 = Cell(String::from("badger"));

    assert_eq!((cell1 * cell2).0, String::from(""));
}

#[test]
fn test_multiplying_int_and_string_negative() {
    let cell1 = Cell(-2);
    let cell2 = Cell(String::from("woo!"));

    assert_eq!((cell1 * cell2).0, String::from("!oow!oow"));
}

#[test]
fn test_multiplying_int_and_string_unicode() {
    let string_cell = Cell(String::from("опа"));

    assert_eq!((Cell(0) * string_cell.clone()).0, String::from(""));
    assert_eq!((Cell(2) * string_cell.clone()).0, String::from("опаопа"));
    assert_eq!((Cell(-3) * string_cell.clone()).0, String::from("апоапоапо"));
}

#[test]
fn test_blank_strings() {
    assert_eq!((Cell(375) * Cell(String::from(""))).0, String::from(""));
    assert_eq!((Cell(573) + Cell(String::from(""))).0, String::from("573 "));
}

#[test]
fn test_adding_matrices_1() {
    let matrix1 = Matrix::new(&[1, 2, 3, 4]);
    let matrix2 = Matrix::new(&[
        String::from("one"), String::from("two"),
        String::from("three"), String::from("four")
    ]);

    assert_eq!((matrix1 + matrix2).by_row(), string_cell_vec!["1 one", "2 two", "3 three", "4 four"]);
}

#[test]
fn test_adding_matrices_2() {
    let matrix1 = Matrix::new(&[1, 0, -3, -37]);
    let matrix2 = Matrix::new(&[
        String::from("едно"), String::from("две"),
        String::from(" "), String::from("четири ")
    ]);

    assert_eq!((matrix1 + matrix2).by_row(), string_cell_vec!["1 едно", "0 две", "  3", " иритеч 37"]);
}

#[test]
fn test_multiplying_matrices_1() {
    let matrix1 = Matrix::new(&[
        1, 2,
        3, 1
    ]);
    let matrix2 = Matrix::new(&[
        String::from("one"), String::from("two"),
        String::from("three"), String::from("you get it")
    ]);

    assert_eq!(matrix1 * matrix2, String::from("one threethree twotwotwo you get it"));
}

#[test]
fn test_multiplying_matrices_2() {
    let matrix1 = Matrix::new(&[
        1, 0,
        -3, -2,
    ]);
    let matrix2 = Matrix::new(&[
        String::from("едно"), String::from("две"),
        String::from(" "), String::from("четири ")
    ]);

    assert_eq!(matrix1 * matrix2, String::from("едно  евдевдевд  иритеч иритеч"));
}
