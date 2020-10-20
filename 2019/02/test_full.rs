// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

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
fn test_construct_packet_no_remainder() {
    let (packet, remainder) = Packet::from_source(b"test packet", 20);
    assert_eq!(packet.payload(), b"test packet");
    assert_eq!(remainder, b"");

    let (packet, remainder) = Packet::from_source("баба".as_bytes(), "баба".len() as u8);
    assert_eq!(packet.payload(), "баба".as_bytes());
    assert_eq!(remainder, b"");
}

#[test]
fn test_construct_packet_with_remainder() {
    let (packet, remainder) = Packet::from_source(b"test packet", 5);
    assert_eq!(packet.payload(), b"test ");
    assert_eq!(remainder, b"packet");

    let (packet, remainder) = Packet::from_source(&remainder, 5);
    assert_eq!(packet.payload(), b"packe");
    assert_eq!(remainder, b"t");
}

#[test]
fn test_construct_packet_with_remainder_cyrillic() {
    let (packet, remainder) = Packet::from_source("дядо".as_bytes(), "дя".len() as u8);
    assert_eq!(packet.payload(), "дя".as_bytes());
    assert_eq!(remainder, "до".as_bytes());

    let (packet, remainder) = Packet::from_source(&remainder, 100);
    assert_eq!(packet.payload(), "до".as_bytes());
    assert_eq!(remainder, b"");
}

#[test]
#[should_panic]
fn test_zero_size() {
    let (_, _) = Packet::from_source(b"xyz", 0);
}

#[test]
fn test_construct_packet_from_unicode() {
    let source = String::from("раз-два");
    let (packet, remainder) = Packet::from_source(source.as_bytes(), 6);
    assert_eq!(packet.payload(), "раз".as_bytes());
    assert_eq!(remainder, "-два".as_bytes());

    let source = String::from("раз-два");
    let (packet, remainder) = Packet::from_source(source.as_bytes(), 1);
    assert_eq!(packet.payload().get(0).unwrap(), "р".as_bytes().get(0).unwrap());
    assert_eq!(remainder.get(0).unwrap(), "р".as_bytes().get(1).unwrap());
}

#[test]
fn test_serialize_packet() {
    let (packet, _) = Packet::from_source(b"xyz", 100);

    let data = packet.serialize();
    assert_eq!(data[0], 1); // version
    assert_eq!(data[1], 3); // size

    assert_eq!(&data[2..=4], b"xyz"); // payload

    let checksum = b'x' as u32 + b'y' as u32 + b'z' as u32;
    assert_eq!(&data[5..=8], checksum.to_be_bytes()); // checksum
}

#[test]
fn test_deserialize_packet() {
    let checksum = b'x' as u32 + b'y' as u32 + b'z' as u32;
    let mut data = vec![1, 3];
    data.extend_from_slice(b"xyz");
    data.extend_from_slice(&checksum.to_be_bytes());

    let (packet, _) = Packet::from_source(b"xyz", 100);
    assert_eq!(Packet::deserialize(&data).unwrap(), (packet, b"" as &[u8]));

    let (packet, _) = Packet::from_source(b"xyz", 100);
    data.extend_from_slice(b"_extra_stuff");
    assert_eq!(Packet::deserialize(&data).unwrap(), (packet, b"_extra_stuff" as &[u8]));
}

#[test]
fn test_deserialize_unicode_packet() {
    let payload = "хитро";
    let checksum = payload.as_bytes().into_iter().map(|b| *b as u32).sum::<u32>();
    let mut data = vec![1, payload.len() as u8]; // version, size

    data.extend_from_slice(payload.as_bytes());
    data.extend_from_slice(&checksum.to_be_bytes());

    let (packet, _) = Packet::from_source(payload.as_bytes(), 100);
    assert_eq!(Packet::deserialize(&data).unwrap(), (packet, b"" as &[u8]));

    let (packet, _) = Packet::from_source(payload.as_bytes(), 100);
    data.extend_from_slice("и още нещо".as_bytes());
    assert_eq!(Packet::deserialize(&data).unwrap(), (packet, "и още нещо".as_bytes()));
}

#[test]
fn test_deserialize_invalid_packet() {
    let checksum = b'x' as u32 + b'y' as u32 + b'z' as u32;
    let mut data = vec![1, 3];
    data.extend_from_slice(b"xyz");
    data.extend_from_slice(&checksum.to_be_bytes());

    // Wrong version:
    data[0] = 2;
    assert_match!(Packet::deserialize(&data), Err(PacketError::UnknownProtocolVersion));
    data[0] = 1;
    assert!(Packet::deserialize(&data).is_ok());

    // Wrong size:
    data[1] = 100;
    assert_match!(Packet::deserialize(&data), Err(PacketError::InvalidPacket));
    data[1] = 4;
    assert_match!(Packet::deserialize(&data), Err(PacketError::InvalidPacket));
    data[1] = 2; // Note: shorter, so invalid checksum
    assert_match!(Packet::deserialize(&data), Err(PacketError::InvalidChecksum));
    data[1] = 3;
    assert!(Packet::deserialize(&data).is_ok());

    // Wrong checksum:
    for checksum_byte in 5..=8 {
        data[checksum_byte] += 1;
        assert_match!(Packet::deserialize(&data), Err(PacketError::InvalidChecksum));
        data[checksum_byte] -= 1;
        assert!(Packet::deserialize(&data).is_ok());
    }
}

#[test]
fn test_iterating_packets() {
    timeout!(1000, {
        let source = String::from("foo bar baz");
        let serializer = source.to_packets(4);
        let packets: Vec<Packet> = serializer.collect();

        assert_eq!(packets.len(), 3);
        assert_eq!(packets[0].payload(), b"foo ");
        assert_eq!(packets[1].payload(), b"bar ");
        assert_eq!(packets[2].payload(), b"baz");
    });
}

#[test]
fn test_iterating_packets_for_zero_size_string() {
    timeout!(1000, {
        let source = String::new();
        let serializer = source.to_packets(4);
        let packets: Vec<Packet> = serializer.collect();

        assert_eq!(packets.len(), 0);
    });
}

#[test]
fn test_consuming_packets() {
    timeout!(1000, {
        let source = String::from("foo bar baz");
        let serializer = source.to_packets(4);

        let packets: Vec<u8> = serializer.flat_map(|p| p.serialize()).collect();
        let result = String::from_packet_data(&packets).unwrap();

        assert_eq!(result.as_str(), "foo bar baz");
    });
}

#[test]
fn test_full_roundtrip() {
    timeout!(1000, {
        let source = String::from("foo bar baz");
        let packet_data = source.to_packet_data(10);
        let destination = String::from_packet_data(&packet_data).unwrap();

        assert_eq!(source, destination);
    });
}

#[test]
fn test_full_roundtrip_for_zero_size_string() {
    timeout!(1000, {
        let source = String::new();
        let packet_data = source.to_packet_data(10);
        let destination = String::from_packet_data(&packet_data).unwrap();

        assert_eq!(source, destination);
    });
}

#[test]
fn test_invalid_packet_combination() {
    timeout!(1000, {
        let source = String::from("сюрприз");
        let mut packets = source.to_packets(3).collect::<Vec<Packet>>();
        // Delete a packet in the middle, end up with bad utf8
        packets.swap_remove(1);
        let packet_data = packets.into_iter().flat_map(|packet| packet.serialize()).collect::<Vec<u8>>();

        assert_match!(String::from_packet_data(&packet_data), Err(PacketError::CorruptedMessage));
    });
}
