# Смятане на π

Този път ще ви улесним, като просто ви дадем теста, с който ще проверяваме. Ще ви затрудним, като не ви дадем почти нищо друго. Искаме да имплементирате структура, която да приеме някакво начално състояние и функция, и с всяка итерация да акумулира безкрайна сума/произведение. Конкретните формули сме взели оттук: <https://www.codeproject.com/Articles/813185/Calculating-the-Number-PI-Through-Infinite-Sequenc>

Ето как ще изглежда теста, който ще трябва да подкарате:

``` rust
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
```

Както виждате, има структура с конструктор, която имплементира `Iterator` (макар че използваме само `.nth`, така че 🤷). Всяка следваща стъпка на итерацията ще викне подадената функция с начална стойност и междинно състояние и ще получи следващата стойност и следващото междинно състояние. Стойността се чете през полето `value`, което е `f64`.

Какви точно generics ѝ трябват, за да може да приеме тези три параметъра, да си ги запази, и да извиква функцията за да си получи изчисленията -- измислете го с помощта на компилатора. Можехме да ви сложим и "скелет" на структурата както обикновено, но имайки пълен тест, не би трябвало да е проблем да имплементирате всичко, което ви липсва.
