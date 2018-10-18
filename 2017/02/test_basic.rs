// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::Polynomial;

#[test]
fn test_basic() {
    let p = Polynomial::from(vec![1.0, 2.0, 1.0]);

    assert!(p.has(&(-1.0, 0.0)));

    assert_eq!(p, p.clone());
    assert_eq!(p, Polynomial::from(vec![1.0, 2.0, 1.0]));

    assert_eq! {
        p,
        Polynomial::interpolate(vec! {
            (-1.0, 0.0),
            ( 0.0, 1.0),
            ( 1.0, 4.0),
        }).unwrap()
    };

    assert_eq!(Polynomial::from(vec![0.0]), Polynomial::default());

    assert_eq!(p.clone() * 2.0, Polynomial::from(vec![2.0, 4.0, 2.0]));
    assert_eq!(p.clone() / 1.0, Polynomial::from(vec![1.0, 2.0, 1.0]));

    assert_eq! {
        p.clone() * Polynomial::from(vec![2.0]),
        Polynomial::from(vec![2.0, 4.0, 2.0])
    };

    assert_eq! {
        p.clone() + Polynomial::from(vec![2.0]),
        Polynomial::from(vec![1.0, 2.0, 3.0])
    };
}
