// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_basic() {
    let input: Vec<char> = "GC".chars().collect();
    let counter = counts(&input);

    assert_eq!(counter.g, 1);
    assert_eq!(counter.c, 1);
    assert_eq!(counter.a, 0);
    assert_eq!(counter.t, 0);

    assert_eq!(dna_complement(&input),         vec!['C', 'G']);
    assert_eq!(reverse_rna_complement(&input), vec!['G', 'C']);
}
