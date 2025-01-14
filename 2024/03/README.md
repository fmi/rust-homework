Ще напишем програма, която работи с логически изрази от вида

```
A & !B & (C | D)
```

Изразът ще съдържа следните символи:

* атом - еднобуквено име на променлива. Може да е всичко освен whitespace символи и специалните символи, използвани отдолу
* оператор:
  * `&` - бинарната операция конюнкция
  * `|` - бинарната операции дизюнкция
  * `!` - унарната операция отрицание
* `(` `)` - скоби, указващи приоритет

Правила за парсене.  
Следват се нормалните правила за групиране и приоритет, с едно изключение. В нашия израз операциите конюнкция и дизюнкция ще имат еднакъв приоритет. Групирането между тях се случва просто от ляво надясно.

```
A & B & C & D  =  ((A & B) & C) & D
A & B | C & D  =  ((A & B) | C) & D   // а не (A & B) | (C & D)
```

Целта е изразът да се разпарси до стойност от типа `Expr`.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(char),
    Not(Box<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
}
```

За да направим нещата по-интересни (и някой би казал по-оптимални), ако има няколко поредни операции конюнкция или дизюнкция, те се обединяват в един списък.  
Т.е. `A & B & C` става `And(['А', 'B', 'C'])`.

Някой може да се запита, не можем ли да ползваме по-опримално представяне и на вложени отрицания? Може, но това не е толкова интересно, затова няма да го правим.


Напишете парсър за описания израз. Парсъра ще получава символите от израза един по един чрез извикване на съответния метод, така че ще трябва да си пази някакво междинно състояние.  
Ако подадения символ води до невалиден израз, трябва да се върне `ParseError`. Например невалидно е да има два поредни бинарни оператора (`А & &`) или два поредни атома (`A & B B`).

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedExpr,
    UnexpectedUnaryOp,
    UnexpectedBinOp,
    UnexpectedParen,
    UnexpectedEnd,
}

/// Парсър за прост израз, който не съдържа скоби
pub struct SimpleExprParser {
    // ...
}

impl SimpleExprParser {
    pub fn new() -> SimpleExprParser {
        todo!();
    }

    /// Приема атом.
    ///
    /// `c` ще бъде валиден символ за атом.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_atom(&mut self, c: char) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема символ за операция.
    ///
    /// `op` ще бъде едно от '&', '|', '!'.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_op(&mut self, op: char) -> Result<(), ParseError> {
        todo!()
    }

    /// Завършва парсването и връща построения израз.
    pub fn finish(self) -> Result<Expr, ParseError> {
        todo!()
    }
}
```

И след това още един парсър, който приема и скоби.

```rust
/// Парсър за пълния израз
pub struct ExprParser {
    // ...
}

impl ExprParser {
    pub fn new() -> ExprParser {
        todo!();
    }

    /// Приема атом.
    ///
    /// `c` ще бъде валиден символ за атом.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_atom(&mut self, c: char) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема символ за операция.
    ///
    /// `op` ще бъде едно от '&', '|', '!'.
    /// В противен случай можете да panic-нете (няма да се тества)
    pub fn push_op(&mut self, op: char) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема отваряща скоба.
    pub fn open_paren(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    /// Приема затваряща скоба.
    pub fn close_paren(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    /// Завършва парсването и връща построения израз.
    pub fn finish(self) -> Result<Expr, ParseError> {
        todo!()
    }
}
```

След като имаме разпарсен израз, можем да свършим някаква работа с него, а именно да го оценим.  
Напишете функция `eval`, която приема израз и два масива от атоми - такива, които се оценяват до истина и такива, които се оценяват до лъжа. Можете да приемете, че двата масива нямат общи елементи (не е нужно да правите проверка). Не е нужно всички използвани атоми от израза да присъстват в масивите.

Ако израза може да се оцени напълно, трябва да се върне `True` или `False`. Иначе трябва да се върне израз `Expr`, където всички срещания на атоми с известна стойност за заместени с тази стойност и опростени доколкото може:

- `A & True  = A;     True  & A = A`
- `A & False = False; False & A = False`
- `A | True  = True;  True  | A = True`
- `A | False = A;     False | A = A`

В резутата не трябва да се връщат `Expr::And` или `Expr::Or` с 0 или 1 елемента - те трябва да се опростят.  
Не трябва, обаче, да се прави разкриване на скоби - "!(A & B)" си остава "!(A & B)".

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    True,
    False,
    Expr(Expr),
}

pub fn eval(expr: &Expr, truthy: &[char], falsy: &[char]) -> Value {
    todo!()
}
```

Забележка: за по-лесно тестване можете да си направите функция `parse`, която приема низ и го подава символ по символ на съответния метод `push_atom/push_op/open_paren/close_paren`. Но трябва да имплементирате отделните методи, защото тестовете ще проверяват тях.

Вижте и придизвикателството за още една идея, която може да ви улесни тестването - <https://fmi.rust-lang.bg/challenges/3>.