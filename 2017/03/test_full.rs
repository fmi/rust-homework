// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

macro_rules! assert_substring {
    ($expected:expr, $actual:expr) => {
        assert!($actual.contains($expected), "Expected {:?} to contain {:?}", $actual, $expected);
    }
}

macro_rules! assert_not_substring {
    ($expected:expr, $actual:expr) => {
        assert!(!$actual.contains($expected), "Expected {:?} to NOT contain {:?}", $actual, $expected);
    }
}

macro_rules! assert_match {
    ($pattern:pat, $actual:expr) => {
        if let $pattern = $actual {
            assert!(true);
        } else {
            assert!(false, "Expected {} to match {:?}", stringify!($pattern), $actual);
        }
    }
}

#[test]
fn test_game_basic() {
    let g = Game::new("funyarinpa", 10).unwrap();

    assert_eq!(10, g.attempts_remaining);
    assert_eq!(0, g.attempted_letters.len());
    assert_eq!(0, g.attempted_words.len());
}

#[test]
fn test_game_error() {
    assert_match!(Some(GameError::InvalidSolution(_)), Game::new("foo bar", 10).err());
    assert_match!(Some(GameError::InvalidSolution(_)), Game::new("foo_bar", 10).err());
    assert_match!(Some(GameError::InvalidSolution(_)), Game::new("", 10).err());
    assert_match!(Some(GameError::InvalidSolution(_)), Game::new("_ _ _", 10).err());
}

#[test]
fn test_game_cyrillic() {
    let mut g = Game::new("клаксон", 10).unwrap();

    let _ = g.guess_letter('к');

    assert_eq!(1, g.attempted_letters.len());
    assert_substring!("к _ _ к _ _ _", format!("{}", g));

    let _ = g.guess_word("класкон");

    assert_eq!(1, g.attempted_letters.len());
    assert_eq!(1, g.attempted_words.len());
    assert_substring!("к _ _ к _ _ _", format!("{}", g));

    let _ = g.guess_word("клаксон");

    assert_substring!("клаксон", format!("{}", g));
}

#[test]
fn test_game_display() {
    let g = Game::new("foo", 10).unwrap();
    assert_substring!("_ _ _", format!("{}", g));
    assert_not_substring!("_ _ _ _", format!("{}", g));

    let g = Game::new("foobar", 10).unwrap();
    assert_substring!("_ _ _ _ _ _", format!("{}", g));
    assert_not_substring!("_ _ _ _ _ _ _", format!("{}", g));
}

#[test]
fn test_game_guess_basic() {
    let mut g = Game::new("foo", 10).unwrap();

    assert_substring!("_ _ _", format!("{}", g));
    assert_eq!(10, g.attempts_remaining);

    assert_eq!(true, g.guess_letter('o').unwrap());
    assert_substring!("_ o o", format!("{}", g));
    assert_eq!(10, g.attempts_remaining);

    assert_match!(GameError::BadGuess(_), g.guess_letter('o').unwrap_err());
    assert_substring!("_ o o", format!("{}", g));
    assert_eq!(10, g.attempts_remaining);

    assert_eq!(false, g.guess_letter('z').unwrap());
    assert_substring!("_ o o", format!("{}", g));
    assert_eq!(9, g.attempts_remaining);
}

#[test]
fn test_game_guess_word() {
    let mut g = Game::new("foobar", 2).unwrap();

    assert_substring!("_ _ _ _ _ _", format!("{}", g));
    assert_eq!(2, g.attempts_remaining);

    assert_eq!(false, g.guess_word("barfoo").unwrap());
    assert_substring!("_ _ _ _ _ _", format!("{}", g));
    assert_eq!(1, g.attempts_remaining);

    assert_match!(GameError::BadGuess(_), g.guess_word("barfoo").unwrap_err());
    assert_substring!("_ _ _ _ _ _", format!("{}", g));
    assert_eq!(1, g.attempts_remaining);

    assert_eq!(true, g.guess_word("foobar").unwrap());
    assert_substring!("foobar", format!("{}", g));
    assert_eq!(1, g.attempts_remaining);
}

#[test]
fn test_game_guess_state_lose() {
    let mut g = Game::new("xyzzy", 2).unwrap();
    let _ = g.guess_letter('a');

    assert_eq!(1, g.attempts_remaining);
    assert_eq!(false, g.is_over());

    let _ = g.guess_letter('b');

    assert_eq!(0, g.attempts_remaining);
    assert_eq!(true, g.is_over());
    assert_substring!("lost", g.to_string());
    assert_not_substring!("won", g.to_string());
    assert_substring!("xyzzy", g.to_string());
}

#[test]
fn test_game_guess_state_won() {
    let mut g = Game::new("zzzz", 2).unwrap();

    let _ = g.guess_letter('a');

    assert_eq!(1, g.attempts_remaining);
    assert_eq!(false, g.is_over());

    let _ = g.guess_letter('z');

    assert_eq!(1, g.attempts_remaining);
    assert_eq!(true, g.is_over());
    assert_substring!("won", g.to_string());
    assert_not_substring!("lost", g.to_string());
    assert_substring!("zzzz", g.to_string());
}

#[test]
fn test_game_over_guesses() {
    let mut g = Game::new("foo", 10).unwrap();
    let _ = g.guess_word("foo");

    assert!(g.is_over());

    assert_match!(Err(GameError::GameOver), g.guess_letter('f'));
    assert_match!(Err(GameError::GameOver), g.guess_letter('b'));
    assert_match!(Err(GameError::GameOver), g.guess_word("foo"));
    assert_match!(Err(GameError::GameOver), g.guess_word("bar"));
}

#[test]
fn test_command_parsing_full_words() {
    assert_match!(Ok(Command::Quit), "quit".parse::<Command>());
    assert_match!(Ok(Command::Quit), "Quit".parse::<Command>());
    assert_match!(Ok(Command::Quit), "QUIT".parse::<Command>());
    assert_match!(Ok(Command::Quit), "quit quit".parse::<Command>());
    assert_match!(Ok(Command::Quit), "quit anything else".parse::<Command>());
    assert_match!(Ok(Command::Quit), "quit with the rage of an angry god".parse::<Command>());

    assert_match!(Err(GameError::ParseError(_)), "flip".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "uqit".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "RAGE QUIT".parse::<Command>());

    assert_match!(Ok(Command::Info), "info".parse::<Command>());
    assert_match!(Ok(Command::Info), "Info".parse::<Command>());
    assert_match!(Ok(Command::Info), "INFO".parse::<Command>());

    assert_match!(Ok(Command::TryLetter('x')), "try letter x".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "try letter xy".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "try letter".parse::<Command>());
    assert_match!(Ok(Command::TryLetter('z')), "Try Letter z".parse::<Command>());

    assert_match!(Ok(Command::TryWord(_)), "try word x".parse::<Command>());
    assert_match!(Ok(Command::TryWord(_)), "try word xyzzy".parse::<Command>());

    assert_match!(Err(GameError::ParseError(_)), "tryword xyzzy".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "try word".parse::<Command>());
}

#[test]
fn test_command_parsing_special() {
    assert_match!(Ok(Command::TryLetter('я')), "try letter я".parse::<Command>());
    assert_match!(Ok(Command::TryWord(_)), "Try Word Язовец".parse::<Command>());

    assert_match!(Err(GameError::ParseError(_)), "".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "    ".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "___".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "word".parse::<Command>());
}

#[test]
fn test_command_parsing_partial_words() {
    assert_match!(Ok(Command::Quit), "q".parse::<Command>());
    assert_match!(Ok(Command::Quit), "Q".parse::<Command>());

    assert_match!(Err(GameError::ParseError(_)), "1q".parse::<Command>());

    assert_match!(Ok(Command::Help), "help".parse::<Command>());
    assert_match!(Ok(Command::Help), "H".parse::<Command>());
    assert_match!(Ok(Command::Help), "h".parse::<Command>());

    assert_match!(Err(GameError::ParseError(_)), "~h".parse::<Command>());

    assert_match!(Ok(Command::Info), "i".parse::<Command>());
    assert_match!(Ok(Command::Info), "I".parse::<Command>());
    assert_match!(Ok(Command::Info), " i ".parse::<Command>());

    assert_match!(Err(GameError::ParseError(_)), "_i".parse::<Command>());
    assert_match!(Ok(Command::TryLetter('c')), "t l c".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "t t t".parse::<Command>());
    assert_match!(Ok(Command::TryWord(_)), "t w c".parse::<Command>());
    assert_match!(Err(GameError::ParseError(_)), "w t w".parse::<Command>());
}

#[test]
fn test_command_parsing_spacing() {
    assert_match!(Ok(Command::Quit), " quit".parse::<Command>());
    assert_match!(Ok(Command::Quit), "q ".parse::<Command>());
}

#[test]
fn test_command_parsing_cyrillic() {
    assert_match!(Ok(Command::TryLetter('я')), "try letter я".parse::<Command>());
    assert_match!(Ok(Command::TryWord(_)), "try word бабаяга".parse::<Command>());
}

#[test]
fn test_command_parsing_extra_stuff() {
    assert_match!(Ok(Command::TryLetter('a')), "try letter a b c".parse::<Command>());

    match "try word foo bar baz".parse::<Command>() {
        Ok(Command::TryWord(word)) => assert_eq!(word, "foo"),
        Ok(command) => assert!(false, "Failed to parse: {:?}", command),
        Err(e) => assert!(false, "Failed to parse: {:?}", e),
    }

    assert_match!(Ok(Command::TryLetter('a')), "tr le a".parse::<Command>());
}
