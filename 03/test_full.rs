// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::{Instant, Duration};

#[derive(Clone)]
struct TestWriter {
    storage: Rc<RefCell<Vec<u8>>>,
}

impl TestWriter {
    fn new() -> Self {
        TestWriter { storage: Rc::new(RefCell::new(Vec::new())) }
    }

    fn into_inner(self) -> Vec<u8> {
        Rc::try_unwrap(self.storage).ok().unwrap().into_inner()
    }

    fn into_string(self) -> String {
        String::from_utf8(self.into_inner()).unwrap()
    }
}

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.storage.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.storage.borrow_mut().flush()
    }
}

struct ErroringMockIO {}

impl Write for ErroringMockIO {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "Write Error!"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "Flush Error!"))
    }
}

#[test]
fn test_basic_push() {
    let mut logger = BufferedLogger::new(TestWriter::new(), 100);
    let now = Instant::now();

    logger.push(now + Duration::from_millis(1), "Warning!");
    logger.push(now + Duration::from_millis(2), "!");
    logger.push(now + Duration::from_millis(3), "info");
    logger.push(now + Duration::from_millis(4), "asdf");

    assert_eq!(
        logger.buffered_entries().as_ref(),
        ["Warning!", "!", "info", "asdf"],
    )
}

#[test]
fn test_basic_log() {
    let mut logger = BufferedLogger::new(TestWriter::new(), 100);
    logger.log("Some warning");

    assert_eq!(
        logger.buffered_entries().as_ref(),
        [format!("Some warning")]
    )
}

#[test]
fn test_flushing_the_buffer() {
    let out = TestWriter::new();
    {
        let mut logger = BufferedLogger::new(out.clone(), 100);
        let now = Instant::now();

        logger.push(now + Duration::from_millis(1), "Some warning");
        logger.push(now + Duration::from_millis(2), "Some other warning");

        logger.flush();

        assert_eq!(logger.buffered_entries().len(), 0);
    }

    assert_eq!(out.into_string(), vec![
        "Some warning\n",
        "Some other warning\n",
    ].join(""));
}

#[test]
fn test_reordering_logs_in_buffer() {
    let mut logger = BufferedLogger::new(TestWriter::new(), 100);
    let now = Instant::now();

    logger.push(now + Duration::from_millis(4), "Fourth");
    logger.push(now + Duration::from_millis(2), "Second");
    logger.push(now + Duration::from_millis(3), "Third");
    logger.push(now + Duration::from_millis(1), "First");

    assert_eq!(logger.buffered_entries(), vec![
        "First",
        "Second",
        "Third",
        "Fourth",
    ]);
}

#[test]
fn test_reordering_logs_in_io() {
    let out = TestWriter::new();
    {
        let mut logger = BufferedLogger::new(out.clone(), 100);
        let now = Instant::now();

        logger.push(now + Duration::from_millis(4), "Fourth");
        logger.push(now + Duration::from_millis(2), "Second");
        logger.push(now + Duration::from_millis(3), "Third");
        logger.push(now + Duration::from_millis(1), "First");

        logger.flush();
    }

    assert_eq!(out.into_string(), vec![
        "First\n",
        "Second\n",
        "Third\n",
        "Fourth\n",
    ].join(""));
}

#[test]
fn test_cloning_a_logger_shares_a_buffer() {
    let out = TestWriter::new();
    let mut first_logger = BufferedLogger::new(out, 100);
    let mut second_logger = first_logger.clone();
    let mut third_logger = second_logger.clone();
    let now = Instant::now();

    first_logger.push(now + Duration::from_millis(2), "Second");
    third_logger.push(now + Duration::from_millis(1), "First");
    second_logger.push(now + Duration::from_millis(3), "Third");

    assert_eq!(first_logger.buffered_entries(), vec![
        "First", "Second", "Third"
    ]);
}

#[test]
fn test_cloning_a_logger_shares_their_io() {
    let out = TestWriter::new();
    {
        let mut first_logger = BufferedLogger::new(out.clone(), 100);
        let mut second_logger = first_logger.clone();
        let mut third_logger = second_logger.clone();
        let now = Instant::now();

        first_logger.push(now + Duration::from_millis(2), "Second");
        third_logger.push(now + Duration::from_millis(1), "First");
        second_logger.push(now + Duration::from_millis(3), "Third");

        first_logger.flush()
    }

    assert_eq!(out.into_string(), vec![
        "First\n",
        "Second\n",
        "Third\n",
    ].join(""));
}

#[test]
fn test_automatic_flushing_when_buffer_limit_is_reached() {
    let out = TestWriter::new();
    let now = Instant::now();
    {
        let mut logger = BufferedLogger::new(out.clone(), 3);
        logger.push(now + Duration::from_millis(1), "One");
        logger.push(now + Duration::from_millis(2), "Two");

        assert_eq!(logger.buffered_entries().len(), 2);

        logger.push(now + Duration::from_millis(3), "Three");
        assert_eq!(logger.buffered_entries().len(), 0);

        logger.push(now + Duration::from_millis(4), "One");
        assert_eq!(logger.buffered_entries().len(), 1);
    }

    assert_eq!(out.into_string(), "One\nTwo\nThree\n");
}

#[test]
fn test_automatic_flushing_when_zero_buffer_limit() {
    let out = TestWriter::new();
    let now = Instant::now();
    {
        let mut logger = BufferedLogger::new(out.clone(), 0);
        logger.push(now + Duration::from_millis(1), "One");
        assert_eq!(logger.buffered_entries().len(), 0);

        logger.push(now + Duration::from_millis(2), "Two");
        assert_eq!(logger.buffered_entries().len(), 0);

        logger.push(now + Duration::from_millis(3), "Three");
        assert_eq!(logger.buffered_entries().len(), 0);
    }

    assert_eq!(out.into_string(), "One\nTwo\nThree\n");
}

#[test]
fn test_multilogger_logs_to_several_ios() {
    let out1 = TestWriter::new();
    let out2 = TestWriter::new();
    let now = Instant::now();

    {
        let logger1 = BufferedLogger::new(out1.clone(), 100);
        let logger2 = BufferedLogger::new(out2.clone(), 100);
        let mut multi = MultiLogger::new();
        multi.log_to(logger1);
        multi.log_to(logger2);

        multi.push(now + Duration::from_millis(1), "One");
        multi.push(now + Duration::from_millis(2), "Two");

        multi.flush();
    }

    assert_eq!(out1.into_string(), "One\nTwo\n");
    assert_eq!(out2.into_string(), "One\nTwo\n");
}

#[test]
fn test_logger_combinations() {
    let out = TestWriter::new();
    {
        let base1 = BufferedLogger::new(out.clone(), 100);
        let scoped1 = ScopedLogger::new("Base1", base1.clone());

        let base2 = BufferedLogger::new(out.clone(), 100);
        let scoped2 = ScopedLogger::new("Base2", base2.clone());

        let mut multi = MultiLogger::new();
        multi.log_to(scoped1);
        multi.log_to(scoped2);
        let mut outer = ScopedLogger::new("Multi", multi);

        outer.log("Test entry");

        assert_eq!(base1.buffered_entries(), vec!["[Base1] [Multi] Test entry"]);
        assert_eq!(base2.buffered_entries(), vec!["[Base2] [Multi] Test entry"]);

        outer.flush();
    }

    assert_eq!(out.into_string(), "[Base1] [Multi] Test entry\n[Base2] [Multi] Test entry\n");
}

#[test]
fn test_multilogger_logs_and_flushes_when_needed() {
    let out1 = TestWriter::new();
    let out2 = TestWriter::new();
    let now = Instant::now();

    {
        let logger1 = BufferedLogger::new(out1.clone(), 3);
        let logger2 = BufferedLogger::new(out2.clone(), 3);
        let mut multi = MultiLogger::new();
        multi.log_to(logger1.clone());
        multi.push(now + Duration::from_millis(1), "One");

        multi.log_to(logger2.clone());
        multi.push(now + Duration::from_millis(2), "Two");
        multi.push(now + Duration::from_millis(3), "Three");

        assert_eq!(logger1.buffered_entries().len(), 0);
        assert_eq!(logger2.buffered_entries().len(), 2);
    }
}

#[test]
fn test_scoped_logger() {
    let out = TestWriter::new();
    let now = Instant::now();

    {
        let base = BufferedLogger::new(out.clone(), 100);
        let mut first_logger = ScopedLogger::new("First", base.clone());
        let mut second_logger = ScopedLogger::new("Second", base.clone());

        first_logger.push(now + Duration::from_millis(1), "One");
        second_logger.push(now + Duration::from_millis(2), "Two");

        assert_eq!(base.buffered_entries(), vec!["[First] One", "[Second] Two"]);

        second_logger.push(now + Duration::from_millis(3), "Three");
        first_logger.push(now + Duration::from_millis(4), "Four");

        first_logger.flush();
        second_logger.flush();
    }

    assert_eq!(out.into_string(), "[First] One\n[Second] Two\n[Second] Three\n[First] Four\n");
}

#[test]
fn test_scoped_logger_with_a_string_tag() {
    let out = TestWriter::new();

    {
        let base = BufferedLogger::new(out.clone(), 100);
        let mut logger = ScopedLogger::new(&String::from("First"), base.clone());
        logger.log("Test");
        logger.try_flush().unwrap();
    }

    assert_eq!(out.into_string(), "[First] Test\n");
}

#[test]
fn test_erroring_io() {
    let out = ErroringMockIO {};

    let mut logger = BufferedLogger::new(out, 2);
    logger.log("One");

    if let Ok(_) = logger.try_flush() {
        assert!(false, "Expected try_flush with an erroring IO to return an error")
    }
    logger.flush(); // Should work, no errors

    logger.log("Two");
    logger.log("Something");
    // Should flush successfully, no errors.
}
