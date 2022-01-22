// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_basic() {
    let room = Room {
        name: String::from("Living Room"),
        north: String::from("Balcony"),
        south: String::from("Bathroom"),
        east: String::from("Kitchen"),
        west: String::from("Entrance"),
    };

    let expected = "
           [   Balcony   ]
                  |
           +------N------+
Entrance - | Living Room | - Kitchen
           +------S------+
                  |
           [  Bathroom   ]";

    assert_eq!(format!("{}", room), expected);
}

#[test]
fn test_cyrillic() {
    let room = Room {
        name: String::from("Дневна"),
        north: String::from("Тераса"),
        south: String::from("Баня"),
        east: String::from("Кухня"),
        west: String::from("Вход"),
    };

    let expected = "
       [ Тераса  ]
            |
       +----N----+
Вход - | Дневна  | - Кухня
       +----S----+
            |
       [  Баня   ]";

    assert_eq!(format!("{}", room), expected);
}

#[test]
fn test_short() {
    let room = Room {
        name: String::from("X"),
        north: String::from("N"),
        south: String::from("S"),
        east: String::from("E"),
        west: String::from("W"),
    };

    let expected = "
    [ N ]
      |
    +-N-+
W - | X | - E
    +-S-+
      |
    [ S ]";

    assert_eq!(format!("{}", room), expected);
}
