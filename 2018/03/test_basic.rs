use std::time::{Instant, Duration};
// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

#[test]
fn test_buffered_logger() {
    let mut logger = BufferedLogger::new(Vec::new(), 100);
    let now = Instant::now();

    logger.push(now + Duration::from_millis(2), "Test2");
    logger.push(now + Duration::from_millis(1), "Test1");

    assert_eq!(logger.buffered_entries(), vec!["Test1", "Test2"]);

    let cloned_logger = logger.clone();
    assert_eq!(cloned_logger.buffered_entries(), vec!["Test1", "Test2"]);

    logger.try_flush().unwrap();
    logger.flush();
    logger.log("Test");

    assert_eq!(logger.buffered_entries().len(), 1);
}

#[test]
fn test_multi_logger() {
    let logger1 = BufferedLogger::new(Vec::new(), 100);
    let logger2 = BufferedLogger::new(Vec::new(), 100);
    let now = Instant::now();

    let mut logger = MultiLogger::new();
    logger.log_to(logger1.clone());
    logger.push(now + Duration::from_millis(1), "Test1");

    logger.log_to(ScopedLogger::new("Second", logger2.clone()));
    logger.push(now + Duration::from_millis(2), "Test2");
    logger.push(now + Duration::from_millis(3), "Test3");

    assert_eq!(logger1.buffered_entries(), vec!["Test1", "Test2", "Test3"]);
    assert_eq!(logger2.buffered_entries(), vec!["[Second] Test2", "[Second] Test3"]);

    logger.try_flush().unwrap();
    logger.flush();
}

#[test]
fn test_scoped_logger() {
    let base = BufferedLogger::new(Vec::new(), 100);
    let mut logger = ScopedLogger::new("Rust", ScopedLogger::new("FMI", base.clone()));
    logger.log("Test");

    assert_eq!(base.buffered_entries(), vec!["[FMI] [Rust] Test"]);
}
