// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;
use std::f64;

const EPS: f64 = f64::EPSILON;
const PI: f64 = f64::consts::PI;

macro_rules! v { ($x:expr, $y:expr, $z:expr) => { Vector::new($x, $y, $z) } }
macro_rules! p { ($x:expr, $y:expr, $z:expr) => { Point::new($x, $y, $z) } }

macro_rules! cheat_guard {
    () => {
        assert_ne!(v!(0.0, 0.0, 0.0), v!(1.0, 1.0, 1.0));
        assert_ne!(p!(0.0, 0.0, 0.0), p!(1.0, 1.0, 1.0));

        assert_eq!(v!(0.0, 0.0, 0.0), v!(0.0, 0.0, 0.0));
        assert_eq!(p!(0.0, 0.0, 0.0), p!(0.0, 0.0, 0.0));
    }
}

#[test]
fn test_equality_basic() {
    assert_eq!(p!(0.0, 0.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_eq!(v!(0.0, 0.0, 0.0), v!(0.0, 0.0, 0.0));

    assert_ne!(v!(1.0, 0.0, 0.0), v!(0.0, 0.0, 0.0));
    assert_ne!(v!(0.0, 1.0, 0.0), v!(0.0, 0.0, 0.0));
    assert_ne!(v!(0.0, 0.0, 1.0), v!(0.0, 0.0, 0.0));

    assert_ne!(p!(1.0, 0.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_ne!(p!(0.0, 1.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_ne!(p!(0.0, 0.0, 1.0), p!(0.0, 0.0, 0.0));
}

#[test]
fn test_equality_floating() {
    assert_eq!(p!(0.1, 0.0, 0.0) + v!(0.2, 0.0, 0.0), p!(0.3, 0.0, 0.0));

    assert_eq!(p!(EPS / 1000.0, 0.0, 0.0), p!(0.0, 0.0, 0.0));

    assert_eq!(p!(EPS / 1000.0, 0.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_eq!(p!(0.0, EPS / 1000.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_eq!(p!(0.0, 0.0, EPS / 1000.0), p!(0.0, 0.0, 0.0));

    assert_ne!(p!(EPS * 1000.0, 0.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_ne!(p!(0.0, EPS * 1000.0, 0.0), p!(0.0, 0.0, 0.0));
    assert_ne!(p!(0.0, 0.0, EPS * 1000.0), p!(0.0, 0.0, 0.0));

    assert_eq!(v!(EPS / 1000.0, 0.0, 0.0), v!(0.0, 0.0, 0.0));
    assert_eq!(v!(0.0, EPS / 1000.0, 0.0), v!(0.0, 0.0, 0.0));
    assert_eq!(v!(0.0, 0.0, EPS / 1000.0), v!(0.0, 0.0, 0.0));

    assert_ne!(v!(EPS * 1000.0, 0.0, 0.0), v!(0.0, 0.0, 0.0));
    assert_ne!(v!(0.0, EPS * 1000.0, 0.0), v!(0.0, 0.0, 0.0));
    assert_ne!(v!(0.0, 0.0, EPS * 1000.0), v!(0.0, 0.0, 0.0));
}

#[test]
fn test_equailty_symmetry() {
    assert_eq!(p!(0.0, 0.0, 0.0), p!(EPS / 1000.0, 0.0, 0.0));
    assert_eq!(p!(0.0, 0.0, 0.0), p!(0.0, EPS / 1000.0, 0.0));
    assert_eq!(p!(0.0, 0.0, 0.0), p!(0.0, 0.0, EPS / 1000.0));

    assert_ne!(p!(0.0, 0.0, 0.0), p!(EPS * 1000.0, 0.0, 0.0));
    assert_ne!(p!(0.0, 0.0, 0.0), p!(0.0, EPS * 1000.0, 0.0));
    assert_ne!(p!(0.0, 0.0, 0.0), p!(0.0, 0.0, EPS * 1000.0));

    assert_eq!(v!(0.0, 0.0, 0.0), v!(EPS / 1000.0, 0.0, 0.0));
    assert_eq!(v!(0.0, 0.0, 0.0), v!(0.0, EPS / 1000.0, 0.0));
    assert_eq!(v!(0.0, 0.0, 0.0), v!(0.0, 0.0, EPS / 1000.0));

    assert_ne!(v!(0.0, 0.0, 0.0), v!(EPS * 1000.0, 0.0, 0.0));
    assert_ne!(v!(0.0, 0.0, 0.0), v!(0.0, EPS * 1000.0, 0.0));
    assert_ne!(v!(0.0, 0.0, 0.0), v!(0.0, 0.0, EPS * 1000.0));
}

#[test]
fn test_points_plus_vectors() {
    cheat_guard!();

    assert_eq!(p!(0.0, 0.0, 0.0) + v!(1.0, 2.0, 3.0), p!(1.0, 2.0, 3.0));
    assert_eq!(p!(1.0, 2.0, 3.0) + v!(0.0, 0.0, 0.0), p!(1.0, 2.0, 3.0));

    assert_eq!(p!(0.0, 0.0, 0.0) + v!(-1.0, -2.0, -3.0), p!(-1.0, -2.0, -3.0));
    assert_eq!(p!(1.0, 2.0, 3.0) + v!(-0.0, -0.0, -0.0), p!(1.0, 2.0, 3.0));
    assert_eq!(p!(1.0, 2.0, 3.0) + v!(-1.0, -2.0, -3.0), p!(0.0, 0.0, 0.0));

    assert_eq!(p!(1.0, 2.0, 3.0) + v!(EPS / 1000.0, EPS / 1000.0, EPS / 1000.0), p!(1.0, 2.0, 3.0));
    assert_ne!(p!(1.0, 2.0, 3.0) + v!(EPS * 1000.0, EPS * 1000.0, EPS * 1000.0), p!(1.0, 2.0, 3.0));
    assert_eq!(p!(1.0, 2.0, 3.0) + v!(-EPS / 1000.0, -EPS / 1000.0, -EPS / 1000.0), p!(1.0, 2.0, 3.0));
    assert_ne!(p!(1.0, 2.0, 3.0) + v!(-EPS * 1000.0, -EPS * 1000.0, -EPS * 1000.0), p!(1.0, 2.0, 3.0));
}

#[test]
fn test_points_minus_points() {
    cheat_guard!();

    assert_eq!(p!(0.0, 0.0, 0.0) - p!(1.0, 2.0, 3.0), v!(-1.0, -2.0, -3.0));
    assert_eq!(p!(1.0, 2.0, 3.0) - p!(0.0, 0.0, 0.0), v!(1.0, 2.0, 3.0));
    assert_eq!(p!(1.0, 2.0, 3.0) - p!(1.0, 2.0, 3.0), v!(0.0, 0.0, 0.0));
}

#[test]
fn test_number_by_vector() {
    cheat_guard!();

    assert_eq!(2.0 * v!(1.0, 2.0, 3.0), v!(2.0, 4.0, 6.0));
    assert_eq!(1.0/2.0 * v!(1.0, 2.0, 3.0), v!(0.5, 1.0, 1.5));
    assert_eq!(0.0 * v!(13.5, 3.333333, PI), v!(0.0, 0.0, 0.0));
}

#[test]
fn test_vector_by_vector() {
    cheat_guard!();

    assert_eq!(v!(1.0, 1.0, 1.0) * v!(2.0, 2.0, 2.0), 6.0);
    assert_eq!(v!(1.0, 2.0, 3.0) * v!(1.0, 2.0, 3.0), 14.0);
    assert_eq!(v!(0.0, 0.0, 0.0) * v!(1.0, 2.0, 3.0), 0.0);
    assert_eq!(v!(1.0, -1.5, 0.0) * v!(-1.0, 1.5, 0.0), -3.25);
    assert_eq!(v!(1.0, -1.0, 0.0) * v!(-1.0, -1.0, 0.0), 0.0);
}

#[test]
fn test_vector_by_vector_cross() {
    cheat_guard!();

    // Opposite vectors:
    assert_eq!(v!(1.0, -1.0, 0.0) ^ v!(-1.0, 1.0, 0.0), v!(0.0, 0.0, 0.0));
    // Same vectors:
    assert_eq!(v!(1.0, -1.0, 0.0) ^ v!(2.0, -2.0, 0.0), v!(0.0, 0.0, 0.0));

    // Different vectors
    assert_eq!(v!(1.0, 2.0, 3.0) ^ v!(3.0, 2.0, 1.0), v!(-4.0, 8.0, -4.0));
    assert_eq!(v!(1.0, 1.0, 0.0) ^ v!(1.0, 2.0, 0.0), v!(0.0, 0.0, 1.0));
    assert_eq!(v!(-1.0, 1.0, 0.0) ^ v!(-1.0, 2.0, 0.0), v!(0.0, 0.0, -1.0));
}

#[test]
fn test_number_vector_multiplication_with_precision() {
    cheat_guard!();

    assert_eq!((EPS/1000.0) * v!(1.0, 1.0, 1.0), v!(0.0, 0.0, 0.0));
    assert_ne!((EPS*1000.0) * v!(1.0, 1.0, 1.0), v!(0.0, 0.0, 0.0));

    assert_eq!((EPS/1000.0) * v!(-1.0, -1.0, -1.0), v!(0.0, 0.0, 0.0));
    assert_ne!((EPS*1000.0) * v!(-1.0, -1.0, -1.0), v!(0.0, 0.0, 0.0));

    assert_eq!((-EPS/1000.0) * v!(1.0, 1.0, 1.0), v!(0.0, 0.0, 0.0));
    assert_ne!((-EPS*1000.0) * v!(1.0, 1.0, 1.0), v!(0.0, 0.0, 0.0));

    assert_eq!((-EPS/1000.0) * v!(-1.0, -1.0, -1.0), v!(0.0, 0.0, 0.0));
    assert_ne!((-EPS*1000.0) * v!(-1.0, -1.0, -1.0), v!(0.0, 0.0, 0.0));
}

#[test]
fn test_line_constructors() {
    let p1 = p!(0.0, 0.0, 0.0);
    let p2 = p!(1.0, 1.0, 1.0);
    let p3 = p!(1.0, -1.0, 1.0);

    assert_eq!(Line::from_pp(p1, p2), Line::from_pp(p1, p2));
    assert_ne!(Line::from_pp(p1, p2), Line::from_pp(p1, p3));

    let p = p!(0.0, 0.0, 0.0);
    let v1 = v!(1.0, 1.0, 1.0);
    let v2 = v!(1.0, -1.0, 1.0);

    assert_eq!(Line::from_pv(p, v1), Line::from_pv(p, v1));
    assert_ne!(Line::from_pv(p, v1), Line::from_pv(p, v2));
}

#[test]
fn test_point_distance() {
    let p1 = p!(0.0, 0.0, 0.0);
    let p2 = p!(1.0, 0.0, 0.0);
    let p3 = p!(0.0, 1.0, 0.0);
    let p4 = p!(0.5, 0.5, 0.0);

    let line = Line::from_pp(p2, p3).unwrap();
    assert!((line.distance(p1) - 2.0_f64.sqrt()/2.0).abs() < EPS * 1000.0);
    assert!(line.distance(p4) < EPS * 1000.0);
}

#[test]
fn test_line_validity() {
    let p1 = p!(0.0, 0.0, 0.0);
    let p2 = p!(1.0, 1.0, 1.0);

    assert!(Line::from_pp(p1, p1).is_none());
    assert!(Line::from_pp(p2, p2).is_none());
    assert!(Line::from_pp(p1, p2).is_some());

    let zero = v!(0.0, 0.0, 0.0);

    assert!(Line::from_pv(p1, zero).is_none());
    assert!(Line::from_pv(p2, zero).is_none());
}

#[test]
fn test_line_equality_by_points() {
    let p1 = p!(0.0, 0.0, 0.0);
    let p2 = p!(0.1, 0.1, 0.1);
    assert_eq!(Line::from_pp(p1, p2), Line::from_pp(p2, p1));

    let offset = p2 - p1;
    assert_eq!(Line::from_pp(p1, p2), Line::from_pp(p1 + offset, p2 + offset));

    let offset = p1 - p2;
    assert_eq!(Line::from_pp(p1, p2), Line::from_pp(p1 + offset, p2 + offset));

    let shift = v!(1.0, 2.0, 3.0);
    assert_ne!(Line::from_pp(p1, p2), Line::from_pp(p1, p2 + shift));
    assert_ne!(Line::from_pp(p1, p2), Line::from_pp(p1 + shift, p2));
    assert_ne!(Line::from_pp(p1, p2), Line::from_pp(p1 + shift, p2 + shift));
}

#[test]
fn test_line_equality_by_vectors() {
    let p = p!(0.0, 0.4, 0.0);
    let v = v!(0.1, -0.2, 0.5);
    let small = v!(EPS/1000.0, EPS/1000.0, EPS/1000.0);

    assert_eq!(Line::from_pv(p, v), Line::from_pv(p, 2.0 * v));
    assert_eq!(Line::from_pv(p, v), Line::from_pv(p, (1.0/4.0) * v));

    assert_eq!(Line::from_pv(p, v), Line::from_pv(p, v + small));
    assert_eq!(Line::from_pv(p, v), Line::from_pv(p + small, v));
    assert_eq!(Line::from_pv(p, v), Line::from_pv(p + small, v + small));

    assert_ne!(Line::from_pv(p, v), Line::from_pv(p, v + v!(0.1, 0.0, 0.0)));
    assert_ne!(Line::from_pv(p, v), Line::from_pv(p + v!(0.1, 0.0, 0.0), v));
}

#[test]
fn test_line_equality_by_points_and_vectors() {
    let p1 = p!(0.0, 0.0, 0.0);
    let p2 = p!(0.3, -0.1, 0.25);
    let p3 = p!(-0.1, 0.3, 0.25);

    assert_eq!(Line::from_pp(p1, p2), Line::from_pv(p1, p2 - p1));
    assert_eq!(Line::from_pp(p1, p2), Line::from_pv(p1, p1 - p2));
    assert_eq!(Line::from_pp(p1, p2), Line::from_pv(p1, 2.0 * (p1 - p2)));
    assert_ne!(Line::from_pp(p1, p2), Line::from_pv(p1, p1 - p3));
}
