# CodeIdentifier

Задачата е да напишете тип, който "обвива" програмен идентификатор. Примерно, имена на променливи. Различните езици си имат различни стилове за идентификация, някои се очаква да са `under_scored`, някои ще са `camelCased`, тук-таме лиспаджиите `kebab-case`-ват неща. Ние ще очакваме да започнете от стандартния rust-ки стил за идентификатор: underscored, и да генерирате всички други. Примерно:

``` rust
let code_identifier = CodeIdentifier::new("some_var").unwrap();

code_identifier.camelcase()  // => "someVar"
code_identifier.underscore() // => "some_var"
```

## Валидност

Очакваме да проверите, че подаденото наистина е валиден идентификатор, по следните правила:

- Първия символ трябва да е задължително буква.
- Всеки следващ символ може да е буква, цифра, или `_`.

Тоест, това са валидни идентификатори: `some_var`, `abc1`.
Невалидни идентификатори са, например `1abc`, `@@@`.

Дефинираме "буква" като: това, за което `char::is_alphabetic` върне истина.
Дефинираме "цифра" като: това, за което `char::is_numeric` върне истина.

## Нормализация на входа

Ако ви подадем низ с интервали, табове, или нови редове преди или след идентификатора, изчистете ги. Тоест, тези две са еквивалентни:

``` rust
let code_identifier = CodeIdentifier::new(" some_var ");
let code_identifier = CodeIdentifier::new("some_var");
```

Ако ви подадем вход с разнообразен casing (големи/малки букви), нормализирайте го:

``` rust
let code_identifier = CodeIdentifier::new("SoMe_vAr").unwrap();
code_identifier.underscore()          // => "some_var"
code_identifier.screaming_snakecase() // => "SOME_VAR"
code_identifier.camelcase()           // => "someVar"
```

Ето структурата на кода, която очакваме:

``` rust
/// Необходимо е CodeIdentifier да имплементира Debug, за да можем да го
/// използваме в тестови assertion-и.
///
#[derive(Debug)]
pub struct CodeIdentifier {
    // Каквито полета ви трябват
}

impl CodeIdentifier {
    /// Функцията ще върне Option<CodeIdentifier>, което ще бъде:
    /// - None: ако входа не е валиден идентификатор. Вижте горе за това
    ///   какво значи "валиден идентификатор".
    /// - Some(code_identifier): Ако входа е валиден.
    ///
    pub fn new(identifier: &str) -> Option<Self> {
        unimplemented!()
    }

    /// Конвертира идентификатора до camelcased вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "someVar"
    ///
    pub fn camelcase(&self) -> String {
        unimplemented!()
    }

    /// Конвертира идентификатора до titlecased вариант (camelcased с първа заглавна буква).
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "SomeVar"
    ///
    pub fn titlecase(&self) -> String {
        unimplemented!()
    }

    /// Конвертира идентификатора до kebabcased вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "some-var"
    ///
    pub fn kebabcase(&self) -> String {
        unimplemented!()
    }

    /// Конвертира идентификатора до underscored вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "some_var"
    ///
    /// - Примерен вход: "Some_Var"
    /// - Примерен изход: "some_var"
    ///
    pub fn underscore(&self) -> String {
        unimplemented!()
    }

    /// Конвертира идентификатора до screaming-snakecased вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "SOME_VAR"
    ///
    pub fn screaming_snakecase(&self) -> String {
        unimplemented!()
    }
}
```

<hr>

Внимавайте всички типове и методи, които ни трябват, да бъдат маркирани като `pub`, за да могат тестовете ни да ги викат. Прочетете и общия [guide за писане на домашни](https://fmi.rust-lang.bg/tasks/guide).
