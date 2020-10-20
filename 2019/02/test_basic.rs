// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_basic_packets() {
    let source = b"hello";
    let (packet, remainder) = Packet::from_source(source, 100);

    assert_eq!(packet.payload().len(), source.len());
    assert_eq!(remainder, b"");
    assert!(packet.serialize().len() > 0);

    if let Err(_) = Packet::deserialize(&packet.serialize()) {
        assert!(false, "Couldn't deserialize serialized packet");
    }
}

#[test]
fn test_basic_iteration() {
    let source = String::from("hello");
    let packets = source.to_packets(100).collect::<Vec<Packet>>();
    assert!(packets.len() > 0);

    let data = source.to_packet_data(100);
    assert!(data.len() > 0);

    if let Err(_) = String::from_packet_data(&data) {
        assert!(false, "Couldn't deserialize serialized packet data");
    }
}
