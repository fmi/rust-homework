# Fmi Buzz 2

Шефа от първото домашно се завърна (The Fizz Boss) и този път иска повече performance.

Какъв е тоя вектор? Много място заема в паметта, по-добре да е итератор. И как така ще клонираме низове, да не пишем руби, я връщайте някакви references!

Не можем *винаги* да връщаме references, защото искаме да връщаме и числа, конвертирани към низове, а има доста такива... Няма къде да ги държим. Но може да използваме [`std::borrow::Cow`](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html), за да държим reference когато можем и own-ната стойност, когато не можем:

``` rust
use std::borrow::Cow;

pub struct FizzBuzzer {
    labels: [String; 3],
}

impl FizzBuzzer {
    pub fn new(labels: [String; 3]) -> Self {
        FizzBuzzer { labels }
    }

    /// Връщаме нова структура `FizzBuzzerIter`, която има връзка с оригиналния FizzBuzzer и
    /// каквото още ѝ трябва, за да може да връща резултати.
    pub fn iter(&self) -> FizzBuzzerIter {
        todo!()
    }
}

pub struct FizzBuzzerIter {
    fizzbuzzer: &FizzBuzzer,
    // каквито други полета ви трябват
}
```

Забележете, че `FizzBuzzerIter` има reference към някакъв `FizzBuzzer`. Ако се опитате да компилирате този код досега, ще получите грешка, защото **не сме сложили lifetime анотации**, които са нужни -- вие трябва да добавите каквито трябва. Свободни сте да добавите каквито други полета искате в тази структура също.

``` rust
impl Iterator for FizzBuzzerIter {
    type Item = Cow<str>;

    /// Очакваме всяко извикване на тази функция да връща следващото естествено число, започващо от
    /// 1:
    ///
    /// - Ако числото се дели на 3, връщаме `Cow::Borrowed`, държащо reference към `labels[0]`
    /// - Ако числото се дели на 5, връщаме `Cow::Borrowed`, държащо reference към `labels[1]`
    /// - Ако числото се дели на 15, връщаме `Cow::Borrowed`, държащо reference към `labels[2]`
    /// - Иначе, връщаме Cow::Owned, държащо числото, конвертирано до `String`
    ///
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
```

Ще трябва да прочетете документацията на [`std::borrow::Cow`](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html), за да разберете как се използва. По-досадното е, че ще трябва и да разберете какви lifetime анотации да сложите, за да може кода да се компилира.

Ето ви пълния тест, който ще използваме, за да оценим задачата: [rust-homework/challenge_02/test_full.rs](https://github.com/fmi/rust-homework/blob/c9c542dd1150c78eb1c60cea4bf21c075e318280/challenge_02/test_full.rs). Необичайно е да пуснем пълния тест, но в случая предизвикателството е да компилирате кода -- ако успеете да го компилирате с минаващ тест, ще си заслужите точките.

Съветваме ви да започнете от това да връщате `String` от итератора, за по-лесно -- `test_basic` би трябвало да минава с това даже, стига да закоментирате останалите. Така ще имплементирате итератора на сравнително спокойствие. После можете да смените `type Item` обратно на `Cow` и да видите как да преминете към него.