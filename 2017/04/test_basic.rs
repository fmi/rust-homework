// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

macro_rules! set {
    ($($item:expr),*) => {
        {
            let mut hash_set = ::std::collections::HashSet::new();
            $( hash_set.insert($item); );*
            hash_set
        }
    };
}

#[test]
fn test_basic() {
    let text = String::from("one, two, three");
    let words = extract_words(&text);
    assert_eq!(vec!["one", "two", "three"], words);

    let mut index = TextIndex::new();
    index.push("one, two, three");
    assert_eq!(set!{"one, two, three"}, index.search("two"));
}
