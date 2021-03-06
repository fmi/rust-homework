# Logging

В тази задача, ще имплементирате структури, които да се използват за logging (писане на информация за протичането на програмата, в някой файл, или на стандартния изход). Трите структури са:

- `BufferedLogger`, който събира съобщения, и, когато броя им надмине определена граница, ги записва в подадения "файл" (тип, който имплементира `Write`), подредени по timestamp.
- `MultiLogger`, който ще ви позволи да log-вате в няколко "файла" наведнъж -- примерно, в истински файл, в syslog, и на стандартния изход.
- `ScopedLogger`, който ще prefix-не всеки logger с определен низ, позволяващ ви различни структури да си държат reference към споделен logger с различен tag, който да ги идентифицира.

За да може да се композират, първо ще си дефинирате споделен trait.

## `Logger` trait

Очакваме дефиниция, подобна на това:

``` rust
use std::time::Instant;
use std::io;

pub trait Logger {
    /// Метод, който добавя нов запис за логване. Първия аргумент, от тип `std::time::Instant`, е
    /// момента във времето, който се асоциира със събитието. Обикновено ще се ползва метода `log`
    /// директно, който запълва този параметър, но метода `push` ще е удобен за тестване на
    /// логиката.
    ///
    /// Втория аргумент е низа, който ще се логва.
    ///
    fn push(&mut self, time: Instant, text: &str);

    /// Метод който ще работи като `push`, с тази разлика, че директно използва `Instant::now()` за
    /// да вземе текущ timestamp.
    ///
    fn log(&mut self, text: &str);

    /// Метод, който записва нещата от вътрешния буфер към някакъв външен носител -- файл, сокет,
    /// стандартния изход. В случай на имплементация, която няма нужда от този метод, винаги може да
    /// се имплементира като просто `Ok(())`.
    ///
    fn try_flush(&mut self) -> io::Result<()>;

    /// Метод, който прави същото като по-горния, но не връща грешка. Вижте по-долу за бележки за
    /// Error handling-а, който очакваме.
    ///
    fn flush(&mut self);
}
```

Имайте предвид, че някои от тези методи вероятно могат да получат default-ни имплементации директно в trait-а. Стига това да се компилира с базовия тест, това би било ок.

## Error handling

Какво може да направим, ако един Logger се опита да пише във файл и това не проработи? Може файла да изчезне, защото е на network drive и интернета пада, може да бъде изтрит, може харддиска да се счупи.

Бихме могли да изискваме от всяка IO операция да връща грешка, но това ще направи употребата на logger-а ужасно неудобна. Бихме могли да викаме `unwrap`, но ако нещо се случи с log-ването, това не значи, че останалата част от кода не работи -- вероятно не е добра идея да спираме цялата програма, понеже log-ването се е счупило.

Следния подход ще свърши работа за целите ни: Метода `try_flush` ще връща всякакви IO грешки, които се случат, и ще тестваме за това. Метода `flush` няма да връща никаква грешка, но няма да спира и програмата -- вместо това, ще напечата грешката на стандартния изход за грешки, използвайки (например) макроса [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html). Където метод **не** връща `io` грешка, очакваме да се справяте с грешките по този начин.

## `BufferedLogger`

``` rust
use std::io::Write;

pub struct BufferedLogger<W: Write> {
    /// Каквито полета ви трябват
}

impl<W: Write> BufferedLogger<W> {
    /// Конструира структура, която ще пази записи в буфер с размер `buffer_size`, и ще ги записва
    /// в подадената структура от тип, който имплементира `Write`;
    ///
    pub fn new(out: W, buffer_size: usize) -> Self {
        unimplemented!()
    }

    /// Връща списък от записите, които са буферирани в момента. Записите се очаква да бъдат
    /// подредени по времето, в което са log-нати, от най-ранни до най-късни.
    ///
    pub fn buffered_entries(&self) -> Vec<String> {
        unimplemented!()
    }
}

/// Вижте по-долу за бележки за клонирането
impl<W: Write> Clone for BufferedLogger<W> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl<W: Write> Logger for BufferedLogger<W> {
    /// Подходящи имплементации на Logger методите
}
```

Това е най-базовия logger който ще напишете, и другите два ще използват него като основа. Всяко викане на `push` или `log` ще добавя нов запис в запазения буфер.

``` rust
let mut buffered_logger = BufferedLogger::new(Vec::new(), 100);
let now = Instant::now();

buffered_logger.push(now + Duration::from_millis(2), "Test2");
buffered_logger.push(now + Duration::from_millis(1), "Test1");

assert_eq!(buffered_logger.buffered_entries(), vec!["Test1", "Test2"]);
```

Забележете, че записите са подредени в ред на подаденото време.

## Flush-ване

Когато извикаме метода `try_flush` или метода `flush`, очакваме всички записи в буфера да бъдат записани в подадения тип, който имплементира `Write` (подредени по време), и да бъдат премахнати от буфера. Тоест:

``` rust
logger.flush();
assert_eq!(logger.buffered_entries().len(), 0);
```

В случай, че при викане на `push` или `log` броя записи в буфера надхвърли подадения `buffer_size`, очакваме да се викне `flush` след като се сложи новия запис в буфера:

``` rust
let mut logger = BufferedLogger::new(Vec::new(), 3);
logger.log("Test");
logger.log("Test");
assert_eq!(logger.buffered_entries().len(), 2);
logger.log("Test");
assert_eq!(logger.buffered_entries().len(), 0);
```

## Писане във "файл"

Важно е да се погрижете, че **всеки** ред ще завършва със символ за нов ред, `\n`. Ако log-нем низовете `Foo` и `Bar`, и flush-нем, очакваме съдържанието на "файла" да е `Foo\nBar\n`. Ще тестваме тези неща, така че ако допуснете грешка тук, ще губите точки. Силно ви съветваме да си помислите как да изтествате четенето и писането във "файл" (иначе казано, в "нещо, което имплементира Write"), за да сте сигурни, че сте го имплементирали правилно.

## Клониране

`BufferedLogger` може да бъде клониран, и очакваме клонингите да **споделят** общ буфер с оригинала. Тоест, ако имаме два logger-а, които са клонирани, и викнем `log` на единия, и после `buffered_entries` на другия, трябва да "видим" същата стойност в буфера.

За целта може да използвате `Rc`, за да споделят два клонинга общ буфер и общ "файл", и `RefCell`, за да можете да извиквате методи, които имат нужда от mutable reference към това, което споделяте. (Има и други начини, бъдете свободни да ги имплементирате, ако ги намерите.) Разгледайте внимателно документацията на двата типа и/или си припомнете лекциите за свързани списъци.

## `MultiLogger`

Този logger ще бъде инстанциран и след това ще може да приеме няколко logger-а, към които ще делегира виканията на методите си:

``` rust
let logger1 = BufferedLogger::new(Vec::new(), 100);
let logger2 = BufferedLogger::new(Vec::new(), 100);
let mut logger = MultiLogger::new();
logger.log_to(logger1.clone());
logger.log_to(logger2.clone());
```

При викане на метод от `Logger` интерфейса, ще минете през "вложените" logger-и, каквито и те да
са, и ще им подавате нужните методи.

``` rust
pub struct MultiLogger {
    /// Каквито полета решите, че ви трябват
}

impl MultiLogger {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn log_to<L: Logger + 'static>(&mut self, logger: L) {
        unimplemented!()
    }
}

impl Logger for MultiLogger {
    /// Подходящи имплементации на Logger методите
}
```

Нещо важно, което трябва да имате предвид -- logger-ите могат да бъдат **от различни конкретни типове**:

``` rust
let logger1 = BufferedLogger::new(Vec::new(), 100);
let logger2 = BufferedLogger::new(Vec::new(), 100);

let mut logger = MultiLogger::new();
logger.log_to(logger1.clone());
logger.log_to(ScopedLogger::new("Second", logger2.clone()));
```

За да постигнете това, съветваме ви да съхранявате всеки конкретен logger в trait object -- `Box<dyn Logger>`.

## `ScopedLogger`

Последния logger ще приеме един вложен logger и низ, който да се използва като "етикет". Ще работи горе-долу така:

``` rust
let base = BufferedLogger::new(Vec::new(), 100);
let logger = ScopedLogger::new("FMI", base);
logger.log("Test");

assert_eq!(base.buffered_entries(), vec!["[FMI] Test"]);
```

Очаквана дефиниция:

``` rust
pub struct ScopedLogger<L: Logger> {
    /// Каквито полета решите, че ви трябват
}

impl<L: Logger> ScopedLogger<L> {
    pub fn new(tag: &str, base_logger: L) -> Self {
        unimplemented!()
    }
}

impl<L: Logger> Logger for ScopedLogger<L> {
    /// Подходящи имплементации на Logger методите
}
```

Забележете, че "tag"-а, който подаваме, се обвива в квадратни скоби, и има един интервал преди истинския текст. Нищо не ви пречи и да вложите няколко такива. Всеки нов "обвиващ" logger добавя своя tag в началото на подадения низ. Това може да доведе до малко странно изглеждащи логове, но би трябвало да е сравнително лесно за имплементация:

``` rust
let base = BufferedLogger::new(Vec::new(), 100);
let mut logger = ScopedLogger::new("Rust", ScopedLogger::new("FMI", base.clone()));
logger.log("Test");

assert_eq!(base.buffered_entries(), vec!["[FMI] [Rust] Test"]);
```
