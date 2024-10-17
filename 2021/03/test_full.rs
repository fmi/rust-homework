// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::io::{self, Read, BufRead};

macro_rules! timeout {
    ($time:expr, $body:block) => {
        use std::panic::catch_unwind;

        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            if let Err(e) = catch_unwind(|| $body) {
                sender.send(Err(e)).unwrap();
                return;
            }

            match sender.send(Ok(())) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });

        if let Err(any) = receiver.recv_timeout(std::time::Duration::from_millis($time)).unwrap() {
            panic!("{}", any.downcast_ref::<String>().unwrap());
        }
    }
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West
];

fn all_links<'a>(dungeon: &'a Dungeon, room: &'a Room) -> Vec<&'a str> {
    ALL_DIRECTIONS.iter().
        flat_map(|dir| dungeon.get_next_room(&room.name, *dir).unwrap()).
        map(|r| r.name.as_str()).
        collect::<Vec<&'a str>>()
}

struct ErroringReader {}

impl Read for ErroringReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "read error!"))
    }
}

impl BufRead for ErroringReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(io::ErrorKind::Other, "fill_buf error!"))
    }

    fn consume(&mut self, _amt: usize) { }
}

#[test]
fn test_adding_rooms_1() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.add_room("Laboratory").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
    assert_eq!(dungeon.get_room("Laboratory").unwrap().name, "Laboratory");
}

#[test]
fn test_adding_rooms_2() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway 1").unwrap();
    dungeon.add_room("Side Closet").unwrap();
    dungeon.add_room("Hallway 2").unwrap();
    dungeon.add_room("Treasure Room").unwrap();

    dungeon.set_link("Entrance", Direction::East, "Hallway 1").unwrap();
    dungeon.set_link("Hallway 1", Direction::East, "Side Closet").unwrap();
    dungeon.set_link("Hallway 1", Direction::North, "Hallway 2").unwrap();
    dungeon.set_link("Hallway 2", Direction::South, "Side Closet").unwrap();
    dungeon.set_link("Hallway 2", Direction::West, "Treasure Room").unwrap();
    dungeon.set_link("Side Closet", Direction::South, "Entrance").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway 1").unwrap().name, "Hallway 1");
    assert_eq!(dungeon.get_room("Side Closet").unwrap().name, "Side Closet");
    assert_eq!(dungeon.get_room("Hallway 2").unwrap().name, "Hallway 2");
    assert_eq!(dungeon.get_room("Treasure Room").unwrap().name, "Treasure Room");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway 1");
    assert_eq!(dungeon.get_next_room("Hallway 1", Direction::West).unwrap().unwrap().name, "Entrance");

    assert_eq!(dungeon.get_next_room("Hallway 1", Direction::North).unwrap().unwrap().name, "Hallway 2");
    // Overwrite "South" link:
    assert_eq!(dungeon.get_next_room("Hallway 2", Direction::South).unwrap().unwrap().name, "Side Closet");

    assert_eq!(dungeon.get_next_room("Side Closet", Direction::South).unwrap().unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::North).unwrap().unwrap().name, "Side Closet");
}

#[test]
fn test_room_links() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();

    assert_eq!(all_links(&dungeon, dungeon.get_room("Entrance").unwrap()), Vec::<&str>::new());

    dungeon.add_room("Hallway 1").unwrap();
    dungeon.set_link("Entrance", Direction::North, "Hallway 1").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1"]
    );

    dungeon.add_room("Hallway 2").unwrap();
    dungeon.set_link("Entrance", Direction::South, "Hallway 2").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1", "Hallway 2"]
    );

    dungeon.add_room("Hallway 3").unwrap();
    dungeon.set_link("Entrance", Direction::East, "Hallway 3").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1", "Hallway 2", "Hallway 3"]
    );

    dungeon.add_room("Hallway 4").unwrap();
    dungeon.set_link("Entrance", Direction::West, "Hallway 4").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1", "Hallway 2", "Hallway 3", "Hallway 4"]
    );
}

#[test]
fn test_overwriting_a_room_link() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();

    assert_eq!(all_links(&dungeon, dungeon.get_room("Entrance").unwrap()), Vec::<&str>::new());

    dungeon.add_room("Hallway 1").unwrap();
    dungeon.set_link("Entrance", Direction::North, "Hallway 1").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1"]
    );

    dungeon.add_room("Hallway 2").unwrap();
    dungeon.set_link("Entrance", Direction::North, "Hallway 2").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 2"]
    );
}

#[test]
fn test_cyrillic_room_names() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Антре").unwrap();
    dungeon.add_room("Хол").unwrap();
    dungeon.set_link("Антре", Direction::North, "Хол").unwrap();

    assert_eq!(dungeon.get_room("Антре").unwrap().name, "Антре");
    assert_eq!(dungeon.get_room("Хол").unwrap().name, "Хол");
    assert!(matches!(dungeon.get_room("Кухня"), Err(Errors::UnknownRoom(_))));

    assert_eq!(dungeon.get_next_room("Антре", Direction::North).unwrap().unwrap().name, "Хол");
    assert_eq!(dungeon.get_next_room("Хол", Direction::South).unwrap().unwrap().name, "Антре");
}

#[test]
fn test_room_errors() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    assert!(matches!(dungeon.add_room("Entrance"), Err(Errors::DuplicateRoom(_))));
    assert!(matches!(dungeon.get_room("Exit"), Err(Errors::UnknownRoom(_))));
}

#[test]
fn test_io_error() {
    timeout!(2000, {
        let dungeon = Dungeon::from_reader(ErroringReader {});
        assert!(matches!(dungeon, Err(Errors::IoError(_))));
    });
}

const TEST_INPUT_1: &str = "
## Rooms
- Entrance
- Hallway

## Links
- Entrance -> East -> Hallway
- Hallway -> West -> Entrance
";

#[test]
fn test_parsing_rooms() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Entrance");
}

const TEST_INPUT_2: &str = "
## Rooms

## Links
";

const TEST_INPUT_3: &str = "
## Rooms
- Pantry
- Kitchen

## Links
";

#[test]
fn test_parsing_no_rooms_or_links() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_2.trim().as_bytes()).unwrap();
    assert!(matches!(dungeon.get_room("Entrance"), Err(Errors::UnknownRoom(_))));

    let dungeon = Dungeon::from_reader(TEST_INPUT_3.trim().as_bytes()).unwrap();
    assert_eq!(all_links(&dungeon, dungeon.get_room("Pantry").unwrap()), Vec::<&str>::new());
}

const TEST_INPUT_4: &str = "
## Chambers

## Links
";

const TEST_INPUT_5: &str = "
## Rooms

## Neighbours
";

const TEST_INPUT_6: &str = "
## Rooms
- Entrance
- Treasure Room

## Links
- Closet -> North -> Bathroom
";

const TEST_INPUT_7: &str = "
## Rooms
- Entrance
- Treasure Room

## Links
- Entrance -> North-west -> Treasure Room
";

#[test]
fn test_invalid_parsing() {
    assert!(matches!(Dungeon::from_reader("".as_bytes()), Err(Errors::LineParseError { line_number: 0 })));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_4.trim().as_bytes()), Err(Errors::LineParseError { line_number: 1 })));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_5.trim().as_bytes()), Err(Errors::LineParseError { line_number: 3 })));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_6.trim().as_bytes()), Err(Errors::UnknownRoom(_))));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_7.trim().as_bytes()), Err(Errors::DirectionParseError(_))));
}

const TEST_INPUT_8: &str = "
## Rooms
- Вход
- Хол

## Links
- Вход -> West -> Хол
";

#[test]
fn test_parsing_cyrillic_rooms() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_8.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Вход").unwrap().name, "Вход");
    assert_eq!(dungeon.get_room("Хол").unwrap().name, "Хол");
    assert!(matches!(dungeon.get_room("Кухня"), Err(Errors::UnknownRoom(_))));

    assert_eq!(dungeon.get_next_room("Вход", Direction::West).unwrap().unwrap().name, "Хол");
    assert_eq!(dungeon.get_next_room("Хол", Direction::East).unwrap().unwrap().name, "Вход");
}

#[test]
fn test_finding_a_direct_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Treasure Room").unwrap();
        dungeon.set_link("Entrance", Direction::West, "Treasure Room").unwrap();

        let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();
        assert_eq!(path.into_iter().map(|p| &p.name).collect::<Vec<_>>(), ["Entrance", "Treasure Room"]);
    });
}

#[test]
fn test_finding_an_indirect_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Hallway 1").unwrap();
        dungeon.add_room("Side Closet").unwrap();
        dungeon.add_room("Hallway 2").unwrap();
        dungeon.add_room("Treasure Room").unwrap();

        dungeon.set_link("Entrance", Direction::East, "Hallway 1").unwrap();
        dungeon.set_link("Hallway 1", Direction::East, "Side Closet").unwrap();
        dungeon.set_link("Hallway 1", Direction::North, "Hallway 2").unwrap();
        dungeon.set_link("Hallway 2", Direction::South, "Side Closet").unwrap();
        dungeon.set_link("Hallway 2", Direction::West, "Treasure Room").unwrap();
        dungeon.set_link("Side Closet", Direction::South, "Entrance").unwrap();

        let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();

        let mut path_iter = path.iter();
        let mut first = path_iter.next().unwrap();
        while let Some(second) = path_iter.next() {
            let first_neighbours = all_links(&dungeon, &first);
            let second_neighbours = all_links(&dungeon, &second);

            assert!(first_neighbours.contains(&second.name.as_str()));
            assert!(second_neighbours.contains(&first.name.as_str()));

            first = second;
        }

        assert_eq!(path[0].name, "Entrance");
        assert_eq!(path[path.len() - 1].name, "Treasure Room");
    });
}

#[test]
fn test_finding_a_reflexive_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Treasure Room").unwrap();

        let path = dungeon.find_path("Entrance", "Entrance").unwrap().unwrap();
        assert_eq!(path[0].name, "Entrance");
        assert_eq!(path.len(), 1);

        let path = dungeon.find_path("Treasure Room", "Treasure Room").unwrap().unwrap();
        assert_eq!(path[0].name, "Treasure Room");
        assert_eq!(path.len(), 1);
    });
}

#[test]
fn test_finding_no_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Treasure Room").unwrap();

        let path = dungeon.find_path("Entrance", "Treasure Room");
        assert!(path.unwrap().is_none());

        let path = dungeon.find_path("Entrance", "Mystery Room");
        assert!(path.is_err());
        let path = dungeon.find_path("Mystery Room", "Treasure Room");
        assert!(path.is_err());
    });
}
