// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::io::{BufRead, BufReader};

#[test]
fn test_string_parsing() {
    assert_eq!(skip_next("[test]", '['), Some("test]"));
    assert_eq!(take_until("one/two", '/'), ("one", "/two"));
    assert_eq!(take_and_skip("one/two", '/'), Some(("one", "two")));
}

#[test]
fn test_basic_csv() {
    let data = r#"
        name, age, birth date
        "Gen Z. Person", "20", "2000-01-01"
    "#.trim().as_bytes();

    let mut csv = Csv::new(BufReader::new(data)).unwrap();
    csv.apply_selection(|_row| Ok(true));

    // Parse line:
    let row = csv.parse_line(r#""Basic Name","13","2020-01-01""#).unwrap();
    assert_eq! {
        (row["name"].as_str(), row["age"].as_str(), row["birth date"].as_str()),
        ("Basic Name", "13", "2020-01-01"),
    };

    // Iteration:
    let filtered_names = csv.map(|row| row.unwrap()["name"].clone()).collect::<Vec<_>>();
    assert_eq!(filtered_names, &["Gen Z. Person"]);

    // Writing:
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
