// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::io::{self, Read, BufRead, BufReader};

macro_rules! assert_match {
    ($expr:expr, $pat:pat) => {
        if let $pat = $expr {
            // all good
        } else {
            assert!(false, "Expression {:?} does not match the pattern {:?}", $expr, stringify!($pat));
        }
    }
}

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
fn test_csv_basic() {
    timeout!(1000, {
        let reader = BufReader::new(r#"
            name, age, birth date
            "Douglas Adams", "42", "1952-03-11"
        "#.trim().as_bytes());

        let mut csv = Csv::new(reader).unwrap();

        assert_eq!(csv.columns, &["name", "age", "birth date"]);

        let row = csv.next().unwrap().unwrap();

        assert_eq!(row["name"], "Douglas Adams");
        assert_eq!(row["age"].parse::<u32>().unwrap(), 42);
        assert_eq!(row["birth date"], "1952-03-11");

        assert!(csv.next().is_none());
    });
}

#[test]
fn test_csv_empty() {
    assert_match!(Csv::new(BufReader::new("".as_bytes())).err(), Some(CsvError::InvalidHeader(_)));
}

#[test]
fn test_csv_duplicate_columns() {
    let data = r#"
        name, age, age
        "Douglas Adams", "42", "Douglas Adams"
    "#.trim().as_bytes();

    assert_match!(Csv::new(BufReader::new(data)).err(), Some(CsvError::InvalidHeader(_)));

    let data = r#"
        name, age, name
        "Douglas Adams", "42", "Douglas Adams"
    "#.trim().as_bytes();

    assert_match!(Csv::new(BufReader::new(data)).err(), Some(CsvError::InvalidHeader(_)));

    let data = r#"
        foo, foo, bar
        "Douglas Adams", "42", "Douglas Adams"
    "#.trim().as_bytes();

    assert_match!(Csv::new(BufReader::new(data)).err(), Some(CsvError::InvalidHeader(_)));
}

#[test]
fn test_skip_next() {
    assert_eq!(skip_next("[test]", '['), Some("test]"));
    assert_eq!(skip_next("<test>", '<'), Some("test>"));

    assert_eq!(skip_next("[test]", '<'), None);
    assert_eq!(skip_next("", '<'), None);
}

#[test]
fn test_take_until() {
    assert_eq!(take_until("one/two", '/'), ("one", "/two"));
    assert_eq!(take_until("баба/яга", '/'), ("баба", "/яга"));
    assert_eq!(take_until("", '/'), ("", ""));
}

#[test]
fn test_take_and_skip() {
    assert_eq!(take_and_skip("one/two", '/'), Some(("one", "two")));
    assert_eq!(take_and_skip("баба/яга", '/'), Some(("баба", "яга")));
    assert_eq!(take_until("", '/'), ("", ""));
}

#[test]
fn test_parsing_helpers_for_unicode() {
    assert_eq!(skip_next("↓яга", '↓'), Some("яга"));
    assert_eq!(take_until("баба↓яга", '↓'), ("баба", "↓яга"));
    assert_eq!(take_and_skip("баба↓яга", '↓'), Some(("баба", "яга")));
}

#[test]
fn test_csv_parse_line() {
    let reader = BufReader::new("name, age, birth date".trim().as_bytes());
    let mut csv = Csv::new(reader).unwrap();

    let row = csv.parse_line(r#""Basic Name","13","2020-01-01""#).unwrap();
    assert_eq! {
        (row["name"].as_str(), row["age"].as_str(), row["birth date"].as_str()),
        ("Basic Name", "13", "2020-01-01"),
    };

    let row = csv.parse_line(r#"" Name With  Spaces  "," 13 ","0-0-0""#).unwrap();
    assert_eq! {
        (row["name"].as_str(), row["age"].as_str(), row["birth date"].as_str()),
        (" Name With  Spaces  ", " 13 ", "0-0-0"),
    };
}

#[test]
fn test_csv_parse_line_with_commas() {
    let reader = BufReader::new("age, name".trim().as_bytes());
    let mut csv = Csv::new(reader).unwrap();

    let row = csv.parse_line(r#""13", "Name, Basic""#).unwrap();
    assert_eq! {
        (row["name"].as_str(), row["age"].as_str()),
        ("Name, Basic", "13"),
    };

    let row = csv.parse_line(r#""13, or maybe 14","Basic Name""#).unwrap();
    assert_eq! {
        (row["name"].as_str(), row["age"].as_str()),
        ("Basic Name", "13, or maybe 14"),
    };
}

#[test]
fn test_csv_single_column_no_data() {
    let mut csv = Csv::new(BufReader::new("singe column".as_bytes())).unwrap();
    assert_eq!(csv.columns.len(), 1);

    assert!(csv.next().is_none());
}

#[test]
fn test_csv_iterating_with_no_selection() {
    timeout!(1000, {
        let reader = BufReader::new(r#"
            name, age, birth date
            "Douglas Adams", "42", "1952-03-11"
            "Gen Z. Person", "20", "2000-01-01"
            "Ada Lovelace", "36", "1815-12-10"
        "#.trim().as_bytes());

        let csv = Csv::new(reader).unwrap();

        let filtered_names = csv.map(|row| row.unwrap()["name"].clone()).collect::<Vec<_>>();
        assert_eq!(filtered_names, &["Douglas Adams", "Gen Z. Person", "Ada Lovelace"]);
    });
}

#[test]
fn test_csv_iterating_with_a_selection() {
    let reader = BufReader::new(r#"
        name, age, birth date
        "Douglas Adams", "42", "1952-03-11"
        "Gen Z. Person", "20", "2000-01-01"
        "Ada Lovelace", "36", "1815-12-10"
    "#.trim().as_bytes());

    let mut csv = Csv::new(reader).unwrap();

    csv.apply_selection(|row| {
        let age = row.get("age").unwrap();
        let age = age.parse::<u32>().unwrap();

        Ok(age > 30)
    });

    let filtered_names = csv.map(|row| row.unwrap()["name"].clone()).collect::<Vec<_>>();
    assert_eq!(filtered_names, &["Douglas Adams", "Ada Lovelace"]);
}

#[test]
fn test_csv_writing_without_a_selection() {
    timeout!(1000, {
        let reader = BufReader::new(r#"
            name,  age    ,birth date
            "Douglas Adams","42","1952-03-11"
            "Gen Z. Person",    "20"   ,   "2000-01-01"
            "Ada Lovelace","36","1815-12-10"
        "#.trim().as_bytes());

        let csv = Csv::new(reader).unwrap();
        let mut output = Vec::new();
        csv.write_to(&mut output).unwrap();

        let output_lines = output.lines().
            map(Result::unwrap).
            collect::<Vec<String>>();

        assert_eq!(output_lines, &[
            "name, age, birth date",
            "\"Douglas Adams\", \"42\", \"1952-03-11\"",
            "\"Gen Z. Person\", \"20\", \"2000-01-01\"",
            "\"Ada Lovelace\", \"36\", \"1815-12-10\"",
        ]);
    });
}

#[test]
fn test_csv_selection_and_writing() {
    timeout!(1000, {
        let reader = BufReader::new(r#"
            name,  age    ,birth date
            "Douglas Adams","42","1952-03-11"
            "Gen Z. Person",    "20"   ,   "2000-01-01"
            "Ada Lovelace","36","1815-12-10"
        "#.trim().as_bytes());

        let mut csv = Csv::new(reader).unwrap();
        csv.apply_selection(|row| Ok(row["name"].contains(".")));

        let mut output = Vec::new();
        csv.write_to(&mut output).unwrap();

        let output_lines = output.lines().
            map(Result::unwrap).
            collect::<Vec<String>>();

        assert_eq!(output_lines, &[
            "name, age, birth date",
            "\"Gen Z. Person\", \"20\", \"2000-01-01\"",
        ]);
    });
}

#[test]
fn test_csv_writing_without_any_rows() {
    timeout!(1000, {
        let reader = BufReader::new(r#"
            name, age, birth date
        "#.trim().as_bytes());

        let csv = Csv::new(reader).unwrap();
        let mut output = Vec::new();
        csv.write_to(&mut output).unwrap();

        let output_lines = output.lines().
            map(Result::unwrap).
            collect::<Vec<String>>();

        assert_eq!(output_lines, &[
            "name, age, birth date",
        ]);
    });
}
