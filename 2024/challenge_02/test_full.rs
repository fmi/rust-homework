#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(char),
    Not(Box<Expr>),
    And(Vec<Expr>),
    Or(Vec<Expr>),
}

#[allow(dead_code)]
#[macro_use]
mod submod {
    // HACK - include the solution code, so the macro is defined
    // in the same crate and is thus visible to the testing code.
    //
    // I forgot to tell you to add `#[macro_use]`.
    include!("../src/lib.rs");
}

#[test]
fn test_macro() {
    expr!(atom('A'));
    expr!(not(atom('A')));
    expr!(and(atom('A'), atom('B'), atom('C')));
    expr!(or(atom('A'), atom('B'), atom('C')));

    expr!(not(and(atom('A'), atom('B'))));
    expr!(not(or(atom('A'), atom('B'))));
    expr!(or(and(atom('A'), atom('B')), and(atom('C'), atom('D'))));

    expr!(or(
        atom('X'),
        and(atom('A'), atom('B')),
        not(and(atom('C'), atom('D'))),
        not(not(not(atom('Y'))))
    ));
}
