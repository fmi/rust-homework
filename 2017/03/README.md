# Hangman

First things first: вижте [guide](https://fmi.rust-lang.bg/tasks/guide)-а за предаване на домашни! Дори да сте го гледали преди, може да има нови неща в него. Бъдете *сигурни*, че решението ви поне се компилира с [базовия тест](https://github.com/fmi/rust-homework/blob/master/03/test_basic.rs), иначе ще получите 0 точки.

Тази задача е игра на бесеница. Ще имплементирате няколко компонента от играта:

- Инициализиране на игра от дума и брой опити
- Конвертиране на текущото състояние на играта до низ, показваем на потребителя.
- Парсене на команда от потребителя.
- Краен резултат, победа или загуба.

## Грешки

Като за начало, очакваме да дефинирате следния тип, с всички възможни грешки, които може да връщате в играта:

``` rust
#[derive(Debug)]
pub enum GameError {
    ParseError(String),
    BadGuess(String),
    InvalidSolution(String),
    GameOver,
}

impl Display for GameError {
    /// Имплементацията на този метод може да върне какъвто низ искате, но типа `GameError` трябва
    /// да имплементира trait-а. Чувствайте се свободни да бъдете креативни със съобщенията, или не.
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ...
    }
}
```

По-долу ще ви уточним кои методи какви типове грешки се очаква да връщат. Съветваме си да имплементирате `From` trait-а, за да конвертирате лесно от стандартни грешки към тези типове грешки. Не е нужно да го направите, и може би не си заслужава за *всички* видове грешки.

Забележете, че някои от грешките приемат низове, които могат да се използват за вариране на начина, по който се показват грешките. В идеалния случай, вие сами бихте могли да си изберете структурата и типовете данни, които да съхранявате във вашите собствени грешки, но понеже трябва да тестваме тия неща, този път просто ги имплементирайте както сме ги описали.

## Превръщане на низ в команда

Очакваме да дефинирате следния тип:

``` rust
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    TryLetter(char),
    TryWord(String),
    Info,
    Help,
    Quit,
}

impl FromStr for Command {
    type Err = GameError;

    /// Този метод ще приеме string slice, и ще върне нова команда, която му съответства. Правилата
    /// за това кои низове се превръщат в какви команди са по-долу.
    ///
    /// В случай на грешка, винаги ще се върне `GameError::ParseError`. С какъвто низ искате -- било то
    /// само входния низ, или пълно съобщение за грешка.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ...
    }
}
```

Командите, както виждате, са 5. Най-просто казано, командите се конвертират по следния начин:

- От входа "help" получаваме командата `Command::Help`
- От входа "info" получаваме командата `Command::Info`
- От входа "quit" получаваме командата `Command::Quit`
- От входа "try letter x" получаваме командата `Command::TryLetter('x')`
- От входа "try word abc" получаваме командата `Command::TryWord(String::from("abc"))`

Ще се наложи да обработите входа и да прецените коя команда с какви параметри ще извадите. Ще дадем доста свобода на входа, за доброто на потребителя:

- Малки или големи букви нямат значение. Командите `help`, `Help` и `HELP` са еквивалентни
- Начален и краен whitespace няма значение. Низа `  try letter x   ` е еквивалентен на `try letter x`
- Може командите да бъдат съкратени, примерно `t l x` и `tr le x` са еквивалентни на `try letter x`.
- Всичко след правилната команда се игнорира. Тоест, `help i need somebody` се парси просто до `Command::Help`, a `try word one two three` се парси до `Command::TryWord(String::from("one"))`
- Няма да тестваме с повече от един интервал между компонентите. Чувствайте се свободни да разбивате входа по само един интервал. (Макар че hint: типа `&str` си има вече метод за тая цел)
- Няма да тестваме за валидност на символа или думата. По-долу, когато инициализирате играта, ще проверявате дали решението е валидно, но тук няма нужда (макар че ако искате, няма да попречи).

Ето и някои ограничения, обаче:

- В командата `try letter x`, въвеждането на повече от един символ за `x` е грешка. Тоест, `try letter xy` е грешка.
- Каквото и да е освен тези команди, е грешка. `foo try letter x` е невалидна команда.
- Празен низ е грешка.

## Инициализиране на нова игра

Състоянието на играта ще се съхранява като структура от тип `Game`. Ето публичните атрибути, които очакваме:

``` rust
use std::collections::HashSet;

pub struct Game {
    /// Букви, които вече са били пробвани
    pub attempted_letters: HashSet<char>,

    /// Думи, които вече са били пробвани
    pub attempted_words: HashSet<String>,

    /// Брой на оставащите опити.
    pub attempts_remaining: u32,

    // ...
}
```

Чувствайте се свободни да вкарате каквито още атрибути решите. Вероятно ще ви трябва поне едно поле за правилната дума, в каквато форма изберете. Може би и още няколко полета ще ви бъдат полезни.

Ето как създаваме нова игра:

``` rust
impl Game {
    /// Първия аргумент е думата, която ще е правилния отговор на играта. Вижте по-долу за
    /// ограниченията, които трябва да спазва.
    ///
    /// Втория аргумент е броя опити, които има играча да познае думата. Когато ударят 0, играта
    /// минава в състояние на "загуба". Запишете тази стойност в `self.attempts_remaining`.
    ///
    pub fn new(solution: &str, attempts: u32) -> Result<Self, GameError> {
        // ...
    }
}
```

Забележете, че метода не връща игра, а връща резултат! Възможно е една игра да бъде конструирана невалидно. Може да се спори, че в такава ситуация е ок да се panic-не, но за целите на упражнението, ще върнем грешка от тип `GameError::InvalidSolution`. Това се случва ако:

- `solution` е празен низ
- `solution` съдържа не-азбучни символи -- тези, за които `char::is_alphabetic()` връща `false`.

Иначе, инициализирате играта с началното ѝ състояние и я връщате като резултат.

## Познаване на букви или думи

Предоставяме следните методи, с които може да се опитаме да познаем буква или дума, променяйки състоянието на играта:

``` rust
impl Game {
    /// Приема символ, проверява дали този символ присъства в решението:
    ///
    ///   - Ако да, връща `Ok(true)`
    ///   - Ако не, връща `Ok(false)` и намалява `self.attempts_remaining` с 1.
    ///
    /// Ако символа вече е бил пробван, връща `Err(GameError::BadGuess)` с каквото съобщение пожелаете.
    ///
    /// Ако `self.attempts_remaining` е 0, играта е приключила със загуба. По-долу ще видите как
    /// това става ясно за потребителя.
    ///
    /// Ако думата е позната (всичките ѝ символи са "разкрити"), играта е приключила с победа.
    /// По-долу ще видите как това става ясно за потребителя.
    ///
    pub fn guess_letter(&mut self, guess: char) -> Result<bool, GameError> {
        // ...
    }

    /// Приема дума, проверява дали тази дума присъства в решението:
    ///
    ///   - Ако да, връща `Ok(true)`
    ///   - Ако не, връща `Ok(false)` и намалява `self.attempts_remaining` с 1.
    ///
    /// Ако думата вече е бил пробвана, връща `Err(GameError::BadGuess)` с каквото съобщение пожелаете.
    ///
    /// Ако `self.attempts_remaining` е 0, играта е приключила със загуба. По-долу ще видите как
    /// това става ясно за потребителя.
    ///
    /// Ако думата е позната, играта е приключила с победа. По-долу ще видите как това става ясно
    /// за потребителя.
    ///
    pub fn guess_word(&mut self, guess: &str) -> Result<bool, GameError> {
        // ...
    }

    /// Връща `true`, ако играта е приключила, по един или друг начин. Иначе връща `false`.
    ///
    pub fn is_over(&self) -> bool {
        // ...
    }
}
```

## Показване на текстова репрезентация на играта

Очакваме да имплементирате trait-а `Display` за играта, който да я конвертира до текст. Това би изглеждало така:

``` rust
use std::fmt::{self, Display, Write};

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ...
    }
}
```

Играта се рендерира различно в зависимост от това дали е спечелена, загубена, или в момента продължава.

В случай на победа, очакваме да върнете низ, който включва думата "won" и думата-решение. Примерно, ако `solution` е било "баба", може да върнете низа:

```
You won! ^_^
The word was: баба
```

В кода, това би изглеждало така: `"You won! ^_^\nThe word was: баба"`. Напълно е ок да напишете нещо друго, стига то да включва низа "won" и (в случая) низа "баба".

В случай на загуба, очакваме да върнете низ, който включва думата "lost" и думата-решение. Примерно, ако `solution` е било "дядо", може да върнете низа:

```
You lost! :/
The word was: дядо
```

Отново, свободни сте да импровизирате с истинското съдържание, стига да включва думата "lost" и решението.

В случай, че играта в момента продължава, очакваме да извадите низ, който показва буквите, които вече са "разкрити" (познати правилно), и показва останалите като "_". Като остава по един интервал между всеки един от тези символи.

Тоест, ако решението е "крокодил", и сме пробвали буквата "к" и буквата "о", може да извадите това:

```
Attempts remaining: 10
Guess: к _ о к о _ _ _
```

Единственото, за което ще проверяваме, ще е форматираната дума с правилния брой подчертавки и познати букви. Чувствайте се свободни да импровизирате за останалия текст, включително, ако искате, да покажете някакво бесило, което да се рисува на процент опити :).

## Не забравяйте!

Вижте [guide](https://fmi.rust-lang.bg/tasks/guide)-а за предаване на домашни! Дори да сте го гледали преди, може да има нови неща в него. Бъдете *сигурни*, че решението ви поне се компилира с [базовия тест](https://github.com/fmi/rust-homework/blob/master/03/test_basic.rs), иначе ще получите 0 точки.
