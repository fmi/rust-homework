# Bigint display

Едно нещо, което не имплементирахме за домашно 2 беше `Display` trait, защото би трябвало да е сравнително прост и си имаше достатъчно други неща за имплементация. Но сега ще искаме да имплементирате даже 2 различни начина за display-ване на един `Bigint`.

Първо, дефиниция и парсене:

```rust
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bigint {
    pub digits: Vec<u8>,
}

impl FromStr for Bigint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());

        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit as u8);
            } else {
                return Err("Invalid input!");
            }
        }

        Ok(Bigint { digits })
    }
}
```

Даваме ви някаква готова имплементация, защото вече ги писахте, така че няма какво да ви караме да я мислите. Все пак, чувствайте се свободни да си я напишете както на вас ви харесва, стига да не чупи външния интерфейс, т.е. типовете, които връща.

Забележете, че този `Bigint` няма знак -- не е особено предизвикателство да го напечатате, така че нека просто го изпуснем за опростяване.

Това, което очакваме да имплементирате е, първо, един `Display`:

``` rust
use std::fmt::{self, Write};

impl fmt::Display for Bigint {
    /// Форматира число като за хора, по най-простия начин -- просто показва цифрите една след
    /// друга:
    ///
    ///   let bigint = Bigint::from_str("100000").unwrap();
    ///   println!("{}", bigint);
    ///   // => 100000
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
```

Един проблем на този вид печатане е че може да е малко тегаво да разберете колко голямо е това число. Би било по-удобно да го напечатате с групички по 3 цифри, хиляди, милиони, и т.н. Но има само една имплементация за `Display` на един тип, нали?

Е, можем да решим този проблем сравнително лесно -- като си имплементираме нов тип, който държи reference към стария, но се форматира по различен начин:

``` rust
pub struct Delimited<'a> {
    bigint: &'a Bigint,
}

impl Bigint {
    pub fn delimited(&self) -> Delimited {
        Delimited { bigint: self }
    }
}

impl<'a> fmt::Display for Delimited<'a> {
    /// Форматира Bigint по малко по-човешки начин -- със запетайки на всеки 3 цифри:
    ///
    ///   let bigint = Bigint::from_str("100000").unwrap();
    ///   println!("{}", bigint.delimited());
    ///   // => 100,000
    ///
    ///   let bigint = Bigint::from_str("100000000").unwrap();
    ///   println!("{}", bigint.delimited());
    ///   // => 100,000,000
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
```

Както и друг път, полетата, които не са `pub` може да ги променяте и преименувате както решите. Единственото, което ще правим в нашите тестове, е да конструираме `Bigint`-ове с `Bigint::from_str` и да ги конвертираме до низове, използвайки `format!` макроса. Вие напишете конкретните имплементации на `Display` за двата типа, както са описани. Мислете за edge cases.
