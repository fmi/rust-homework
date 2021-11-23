// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

// За помощни цели:
fn string_cell_vec(s1: &str, s2: &str, s3: &str, s4: &str) -> Vec<Cell<String>> {
    [s1, s2, s3, s4].into_iter().map(String::from).map(Cell).collect::<Vec<Cell<String>>>()
}

#[test]
fn test_basic() {
    assert_eq!((Cell(4) + Cell(String::from("badger"))).0, String::from("4 badger"));
    assert_eq!((Cell(2) * Cell(String::from("mushroom"))).0, String::from("mushroommushroom"));

    let matrix1 = Matrix::new(&[1, 2, 3, 4]);
    let matrix2 = Matrix::new(&[
        String::from("one"), String::from("two"),
        String::from("three"), String::from("four")
    ]);
    assert_eq!(matrix1.by_row()[0], Cell(1));
    assert_eq!(matrix1.by_col()[0], Cell(1));

    assert_eq!(
        (matrix1 + matrix2).by_row(),
        string_cell_vec("1 one", "2 two", "3 three", "4 four")
    );

    let matrix1 = Matrix::new(&[1, 1, 1, 1]);
    let matrix2 = Matrix::new(&[
        String::from("one"), String::from("two"),
        String::from("three"), String::from("four")
    ]);
    assert_eq!(matrix1 * matrix2, String::from("one three two four"));
}
