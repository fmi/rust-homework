use solution::*;

#[test]
fn test_basic() {
    let english_letters = "abcdefghijklmnopqrstuvwxyz";

    // Конструираме по два различни начина, just in case -- няма причина да не работи и с двата.
    assert!(Game::new(english_letters, "!!!").is_err());
    let mut game = Game::new(&String::from(english_letters), "abc").unwrap();

    assert!(matches!(game.status, GameStatus::InProgress));
    assert_eq!(game.attempts, 0);
    assert_eq!(game.to_string(), "|_||_||_|");
    assert_eq!(game.guess_word("abc").unwrap().to_string(), "[A][B][C]");
}
