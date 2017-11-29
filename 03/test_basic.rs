// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

macro_rules! assert_substring {
    ($expected:expr, $actual:expr) => {
        assert!($actual.contains($expected), "Expected {:?} to contain {:?}", $actual, $expected);
    }
}

macro_rules! assert_match {
    ($expected:pat, $actual:expr) => {
        if let $expected = $actual {
            assert!(true);
        } else {
            assert!(false, "Expected to match pattern on {:?}", $actual);
        }
    }
}

#[test]
fn test_game_basic() {
    let mut g = Game::new("foobar", 2).unwrap();

    assert!(!g.is_over());

    assert_eq!(2, g.attempts_remaining);
    assert_eq!(0, g.attempted_letters.len());
    assert_eq!(0, g.attempted_words.len());

    assert_substring!("_ _ _ _ _ _", format!("{}", g));

    assert!(g.guess_letter('o').unwrap());
    assert_eq!(1, g.attempted_letters.len());
    assert_eq!(0, g.attempted_words.len());
    assert_substring!("_ o o _ _ _", format!("{}", g));

    assert!(g.guess_word("foobar").unwrap());
    assert_eq!(1, g.attempted_letters.len());
    assert_eq!(1, g.attempted_words.len());
    assert_substring!("foobar", format!("{}", g));
    assert_substring!("won", format!("{}", g));

    assert!(g.is_over());
}

#[test]
fn test_commands_basic() {
    assert_match!(Ok(Command::Help), "help".parse::<Command>());
    assert_match!(Ok(Command::Info), "info".parse::<Command>());
    assert_match!(Ok(Command::Quit), "quit".parse::<Command>());
    assert_match!(Ok(Command::TryLetter(_)), "try letter x".parse::<Command>());
    assert_match!(Ok(Command::TryWord(_)), "try word xyzzy".parse::<Command>());
}

#[test]
fn test_errors_basic() {
    assert_match!(GameError::ParseError(_), GameError::ParseError(String::from("error!")));
    assert_match!(GameError::BadGuess(_), GameError::BadGuess(String::from("error!")));
    assert_match!(GameError::InvalidSolution(_), GameError::InvalidSolution(String::from("error!")));
    assert_match!(GameError::GameOver, GameError::GameOver);
}
