За тестване на домашното ще е нужно да си построите литерали от типа `Expr`.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(char),
    Not(Box<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
}
```

Но това не е много удобно, защото синтаксиса става малко тежък - вложените енумерации, `Box`, `Vec`, много скоби.  
`Expr::Not(Box::new(Expr::Atom('A')))`

От всичкия синтактичен шум става трудно да се прочете какъв е изразът. Едно макро би помогнало. (В случая и няколко свободни функции биха помогнали, но е добър пример, ако човек иска да се поупражни макроси.)

Напишете декларативно макро чрез `macro_rules!`, което конструира литерали от типа `Expr`.  
Макрото трябва да поддържа следния синтаксис:

```rust
expr!(atom('A')),
expr!(not(atom('A'))),
expr!(and(atom('A'), atom('B'), atom('C'))),
expr!(or(atom('A'), atom('B'), atom('C'))),
expr!(not(and(atom('A'), atom('B')))),
expr!(not(or(atom('A'), atom('B')))),
expr!(or(and(atom('A'), atom('B')), and(atom('C'), atom('D')))),
// и т.н.
```
