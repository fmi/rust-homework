// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::io::{self, Read, BufRead, BufReader};

// За тестване че някакъв резултат пасва на някакъв pattern:
macro_rules! assert_match {
    ($expr:expr, $pat:pat) => {
        if let $pat = $expr {
            // all good
        } else {
            assert!(false, "Expression {:?} does not match the pattern {:?}", $expr, stringify!($pat));
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

impl BufRead for ErroringReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(io::ErrorKind::Other, "fill_buf error!"))
    }

    fn consume(&mut self, _amt: usize) { }
}

#[test]
fn test_string_parsing() {
    assert_eq!(skip_next("[test]", '['), Some("test]"));
    assert_eq!(take_until("one/two", '/'), ("one", "/two"));
    assert_eq!(take_and_skip("one/two", '/'), Some(("one", "two")));
}

#[test]
fn test_csv_error() {
    assert_match!(Csv::new(ErroringReader {}).err(), Some(CsvError::IO(_)));
}

#[test]
fn test_basic_csv() {
    let data = r#"
        name, age, birth date
        "Gen Z. Person", "20", "2000-01-01"
    "#.trim().as_bytes();

    let mut csv = Csv::new(BufReader::new(data)).unwrap();
    csv.apply_selection(|_row| Ok(true));

    // Парсене на един ред:
    let row = csv.parse_line(r#""Basic Name","13","2020-01-01""#).unwrap();
    assert_eq! {
        (row["name"].as_str(), row["age"].as_str(), row["birth date"].as_str()),
        ("Basic Name", "13", "2020-01-01"),
    };

    // Употреба като итератор:
    let filtered_names = csv.map(|row| row.unwrap()["name"].clone()).collect::<Vec<_>>();
    assert_eq!(filtered_names, &["Gen Z. Person"]);

    // Писане в някакъв изход
    let mut csv = Csv::new(BufReader::new(data)).unwrap();
    csv.apply_selection(|_row| Ok(true));

    let mut output = Vec::new();
    csv.write_to(&mut output).unwrap();

    let output_lines = output.lines().
        map(Result::unwrap).
        collect::<Vec<String>>();

    assert_eq!(output_lines, &[
        "name, age, birth date",
        "\"Gen Z. Person\", \"20\", \"2000-01-01\"",
    ]);
}
