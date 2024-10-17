use solution::*;

macro_rules! assert_close {
    ($eps:expr, $left:expr, $right:expr) => {
        let left_str = $left;
        let right_str = $right;
        let eps_str = stringify!($eps);

        assert!(($left - $right).abs() < $eps, "{left_str} was not {eps_str}-close to {right_str}");
    }
}

#[test]
fn test_calculate_pi_using_wallis() {
    let mut series = InfiniteSeries::new(1.0, (2.0, 1.0), |value, (n, d)| {
        let value = value * (n * n / (d * (d + 2.0)));
        let new_state = (n + 2.0, d + 2.0);

        (value, new_state)
    });

    series.nth(200);
    assert_close!(0.01, series.value, std::f64::consts::PI / 2.0);

    series.nth(2000);
    assert_close!(0.001, series.value, std::f64::consts::PI / 2.0);
}

#[test]
fn test_calculate_pi_using_leibniz() {
    let mut series = InfiniteSeries::new(0.0, 1.0, |value, d| {
        let value = value + (4.0 / d);
        let new_state = -(d + d.signum() * 2.0);

        (value, new_state)
    });

    series.nth(200);
    assert_close!(0.01, series.value, std::f64::consts::PI);

    series.nth(2000);
    assert_close!(0.001, series.value, std::f64::consts::PI);
}

#[test]
fn test_calculate_pi_using_the_close_enough_method() {
    let mut series = InfiniteSeries::new(3.1, 0.0, |value, state| (value, state));

    series.nth(200);
    assert_close!(0.01, series.value, 3.1);

    series.nth(2000);
    assert_close!(0.001, series.value, 3.1);
}
