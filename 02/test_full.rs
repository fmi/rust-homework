// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::Polynomial;

fn valid_lp(poly: &Polynomial, points: &Vec<(f64, f64)>) -> bool {
    points.iter().all(|point| poly.has(point))
}

#[test]
fn test_create_poly() {
    let p = Polynomial::from(Vec::new());
    assert_eq!(p, Polynomial::from(vec![0.0]));
    assert_eq!(p, Polynomial::default());

    let p = Polynomial::from(vec![1.0]);
    assert_eq!(p, Polynomial::from(vec![1.0]));

    let p = Polynomial::from(vec![0.0, 0.0, 1.0, 2.0, 5.0, 0.0]);
    assert_eq!(p, Polynomial::from(vec![1.0, 2.0, 5.0, 0.0]));
}

#[test]
fn test_fp_comparison() {
    let p = Polynomial::from(vec![1.0, 1e-25]);

    assert_eq!(p.clone() * 3.0, Polynomial::from(vec![3.0, 3e-25]));
    assert_eq!(p.clone() / 0.5, Polynomial::from(vec![2.0, 2e-25]));
    assert_eq!(p.clone() + p.clone(), Polynomial::from(vec![2.0, 2e-25]));
}

#[test]
fn test_mul_poly_f64_zero() {
    let p0 = Polynomial::from(Vec::new());
    let p1 = p0.clone() * 2.0;
    assert_eq!(p0, Polynomial::from(vec![0.0]));
    assert_eq!(p1, Polynomial::from(vec![0.0]));
}

#[test]
fn test_mul_poly_f64() {
    let p0 = Polynomial::from(vec![1.0, -2.0, 5.0]);
    let p1 = p0.clone() * 5.0;
    assert_eq!(p1, Polynomial::from(vec![5.0, -10.0, 25.0]));
}

#[test]
fn test_div_poly_f64_zero() {
    let p0 = Polynomial::from(Vec::new());
    let p1 = p0.clone() / 2.0;
    assert_eq!(p0, Polynomial::from(vec![0.0]));
    assert_eq!(p1, Polynomial::from(vec![0.0]));
}

#[test]
fn test_div_poly_f64() {
    let p0 = Polynomial::from(vec![1.0, -2.0, 5.0]);
    let p1 = p0.clone() / -6.0;
    assert_eq!(p1, Polynomial::from(vec![1.0 / -6.0, -2.0 / -6.0, 5.0 / -6.0]));
}

#[test]
fn test_add_poly_zero_one() {
    let a = Polynomial::from(vec![]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a + b, Polynomial::from(vec![2.0, 0.0, 3.0]));

    let a = Polynomial::from(vec![1.0]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a + b, Polynomial::from(vec![2.0, 0.0, 4.0]));
}

#[test]
fn test_add_poly() {
    let a = Polynomial::from(vec![1.0, -1.0]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a + b, Polynomial::from(vec![2.0, 1.0, 2.0]));

    let a = Polynomial::from(vec![1.0, -1.0, 0.0]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a + b, Polynomial::from(vec![3.0, -1.0, 3.0]));

    let a = Polynomial::from(vec![1.0,  2.0]);
    let b = Polynomial::from(vec![-1.0, 3.0]);

    assert_eq!(a + b, Polynomial::from(vec![5.0]));
}

#[test]
fn test_arithmetic_properties() {
    // a + (-a) == 0
    let a = Polynomial::from(vec![2.0, 0.0, 3.0]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]) * -1.0;

    assert_eq!(a + b, Polynomial::default());

    // a + b == b + a
    let a = Polynomial::from(vec![0.0, 1.0, 2.0, 3.0, 0.0]);
    let b = Polynomial::from(vec![4.0, 7.3, 13.37, 188.0]);

    assert_eq!(a.clone() * b.clone(), b.clone() * a.clone());
    assert_eq!(a.clone() + b.clone(), b.clone() + a.clone());
    assert!(a.clone() + b.clone() == b.clone() + a.clone());

    // (a + b) + c == a + (b + c)
    let a = Polynomial::from(vec![0.0, 1.0, 2.0, 3.0, 0.0]);
    let b = Polynomial::from(vec![4.0, 7.3, 13.37, 188.0]);
    let c = Polynomial::from(vec![1.0 / 3.0]);

    assert_eq! {
        (a.clone() + b.clone()) + c.clone(),
        a.clone() + (b.clone() + c.clone())
    };

    // (a + b) + c == a * c + b * c
    assert_eq! {
        (a.clone() + b.clone()) * c.clone(),
        a.clone() * c.clone() + b.clone() * c.clone()
    };
}

#[test]
fn test_mul_poly_zero_one() {
    let a = Polynomial::from(vec![]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a * b, Polynomial::from(vec![0.0, 0.0, 0.0]));

    let a = Polynomial::from(vec![1.0]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a * b, Polynomial::from(vec![2.0, 0.0, 3.0]));
}

#[test]
fn test_mul_poly() {
    let a = Polynomial::from(vec![1.0, -5.0]);
    let b = Polynomial::from(vec![1.0, 5.0]);

    // (x+c)*(x-c) = x^2 - c^2
    assert_eq!(a * b, Polynomial::from(vec![1.0, 0.0, -25.0]));

    let a = Polynomial::from(vec![1.0, -1.0]);
    let b = Polynomial::from(vec![2.0, 0.0, 3.0]);

    assert_eq!(a * b, Polynomial::from(vec![2.0, -2.0, 3.0, -3.0]));
}

#[test]
fn test_has_point() {
    let p = Polynomial::from(vec![1.0, 2.0, 1.0]);

    assert!(p.has(&(-1.0, 0.0)));
    assert!(p.has(&(5.0, 36.0)));

    let p = Polynomial::from(vec![1.0, 1e-25]);
    assert!(p.has(&(-1e-25, 0.0)));
    assert!(p.has(&(-3e-25, -2e-25)));
}

#[test]
fn test_lagrange_poly_1() {
    // x^2 + 2*x + 1
    let points = vec![
        (-1.0, 0.0),
        ( 0.0, 1.0),
        ( 1.0, 4.0)
    ];

    let poly = Polynomial::interpolate(points.clone());
    let poly = poly.unwrap();
    assert_eq!(poly, Polynomial::from(vec![1.0, 2.0, 1.0]));
    assert!(valid_lp(&poly, &points));
}

#[test]
fn test_lagrange_poly_2() {
    let points = vec![
        (-2.0, 0.0),
        ( 1.3, -10.0),
        ( 5.9, 4.0),
        ( 7.2, 4.0),
        ( 10.0, 4.0)
    ];

    let poly = Polynomial::interpolate(points.clone());
    assert!(poly.is_some());
    assert!(valid_lp(&poly.unwrap(), &points));
}

#[test]
fn test_lagrange_poly_err_eq_x() {
    let points = vec![
        (-1.0, 0.0),
        ( 1.0, 1.0),
        ( 1.0, 4.0)
    ];
    let poly = Polynomial::interpolate(points.clone());
    assert!(poly.is_none());
}
