use solution::*;

// За тестване че някакъв резултат пасва на някакъв pattern:
macro_rules! assert_match {
    ($expr:expr, $pat:pat) => {
        if let $pat = $expr {
            // all good
        } else {
            assert!(false, "Expression {:?} does not match the pattern {:?}", $expr, stringify!($pat));
        }
    }
}

#[test]
fn test_word_not_in_alphabet_on_construction() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";

    assert_match!(Game::new(english_letters, "oopsie-daisy"), Err(GameError::NotInAlphabet('-')));
}

#[test]
fn test_word_not_in_alphabet_on_construction_cyrrilic() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let bulgarian_letters = "абвгдежзийклмнопрстуфхцчшщъьюя";

    assert_match!(Game::new(english_letters, "опа"), Err(GameError::NotInAlphabet('о')));
    assert_match!(Game::new(bulgarian_letters, "oops"), Err(GameError::NotInAlphabet('o')));
    assert_match!(Game::new(bulgarian_letters, "смайлифейс😄"), Err(GameError::NotInAlphabet('😄')));
}

#[test]
fn test_word_not_in_alphabet_on_guess() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";

    let mut game = Game::new(english_letters, "boop").unwrap();
    assert_match!(game.guess_word("'oop"), Err(GameError::NotInAlphabet('\'')));
}

#[test]
fn test_word_not_in_alphabet_on_guess_cyrillic() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let bulgarian_letters = "абвгдежзийклмнопрстуфхцчшщъьюя";

    let mut game = Game::new(english_letters, "boop").unwrap();
    assert_match!(game.guess_word("хопа"), Err(GameError::NotInAlphabet('х')));

    let mut game = Game::new(bulgarian_letters, "хелп").unwrap();
    assert_match!(game.guess_word("хеlп"), Err(GameError::NotInAlphabet('l')));
}

#[test]
fn test_word_display() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "bop").unwrap();

    assert_eq!(game.guess_word("ops").unwrap().to_string(), "(O)(P)>S<");
    assert_eq!(game.guess_word("pob").unwrap().to_string(), "(P)[O](B)");
    assert_eq!(game.guess_word("bop").unwrap().to_string(), "[B][O][P]");
}

#[test]
fn test_word_display_with_repetitions() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "boop").unwrap();

    assert_eq!(game.guess_word("oops").unwrap().to_string(), "(O)[O](P)>S<");
    assert_eq!(game.guess_word("poob").unwrap().to_string(), "(P)[O][O](B)");
    assert_eq!(game.guess_word("boop").unwrap().to_string(), "[B][O][O][P]");
}

#[test]
fn test_word_display_bulgarian() {
    let bulgarian_letters = "абвгдежзийклмнопрстуфхцчшщъьюя";
    let mut game = Game::new(bulgarian_letters, "стол").unwrap();

    assert_eq!(game.guess_word("лале").unwrap().to_string(), "(Л)>А<(Л)>Е<");
    assert_eq!(game.guess_word("атол").unwrap().to_string(), ">А<[Т][О][Л]");
    assert_eq!(game.guess_word("стол").unwrap().to_string(), "[С][Т][О][Л]");
}

#[test]
fn test_word_display_german() {
    let german_letters = "abcdefghijklmnopqrstuvwxyzäöüß";
    let mut game = Game::new(german_letters, "süß").unwrap();

    assert_eq!(game.to_string(), "|_||_||_|");
    assert_eq!(game.guess_word("füß").unwrap().to_string(), ">F<[Ü][SS]");
}

#[test]
fn test_wrong_length() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "boop").unwrap();

    assert_match!(game.guess_word("boorish"), Err(GameError::WrongLength { expected: 4, actual: 7 }));
    assert_match!(game.guess_word("bop"),     Err(GameError::WrongLength { expected: 4, actual: 3 }));
    assert_match!(game.guess_word(" bop "),   Err(GameError::WrongLength { expected: 4, actual: 5 }));
    assert_match!(game.guess_word(""),        Err(GameError::WrongLength { expected: 4, actual: 0 }));
}

#[test]
fn test_game_display() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "cape").unwrap();

    assert_eq!(game.to_string(), "|_||_||_||_|");
    game.guess_word("peak").unwrap();
    assert_eq!(game.to_string(), "|_||_||_||_|\n(P)(E)(A)>K<");
    game.guess_word("cape").unwrap();
    assert_eq!(game.to_string(), "|_||_||_||_|\n(P)(E)(A)>K<\n[C][A][P][E]");
}

#[test]
fn test_game_display_cyrillic() {
    let bulgarian_letters = "абвгдежзийклмнопрстуфхцчшщъьюя";
    let mut game = Game::new(bulgarian_letters, "аре").unwrap();

    assert_eq!(game.to_string(), "|_||_||_|");
    game.guess_word("опа").unwrap();
    assert_eq!(game.to_string(), "|_||_||_|\n>О<>П<(А)");
    game.guess_word("абе").unwrap();
    assert_eq!(game.to_string(), "|_||_||_|\n>О<>П<(А)\n[А]>Б<[Е]");
    game.guess_word("аре").unwrap();
    assert_eq!(game.to_string(), "|_||_||_|\n>О<>П<(А)\n[А]>Б<[Е]\n[А][Р][Е]");
}

#[test]
fn test_game_display_german() {
    let german_letters = "abcdefghijklmnopqrstuvwxyzäöüß";
    let mut game = Game::new(german_letters, "süß").unwrap();

    assert_eq!(game.to_string(), "|_||_||_|");
    game.guess_word("füß").unwrap();
    assert_eq!(game.to_string(), "|_||_||_|\n>F<[Ü][SS]");
}

#[test]
fn test_game_state_1() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "abc").unwrap();

    assert_match!(game.status, GameStatus::InProgress);
    game.guess_word("abc").unwrap();
    assert_match!(game.status, GameStatus::Won);
    assert_match!(game.guess_word("abc"), Err(GameError::GameIsOver(GameStatus::Won)));
}

#[test]
fn test_game_state_2() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "abc").unwrap();

    assert_match!(game.status, GameStatus::InProgress);
    for _ in 0..5 {
        assert_match!(game.status, GameStatus::InProgress);
        game.guess_word("bca").unwrap();
    }
    assert_match!(game.status, GameStatus::Lost);
    assert_match!(game.guess_word("abc"), Err(GameError::GameIsOver(GameStatus::Lost)));
}

#[test]
fn test_game_state_3() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut game = Game::new(english_letters, "abc").unwrap();

    assert_eq!(game.attempts, 0);

    for attempt_count in 1..=5 {
        game.guess_word("bca").unwrap();
        assert_eq!(game.attempts, attempt_count);
    }
}
