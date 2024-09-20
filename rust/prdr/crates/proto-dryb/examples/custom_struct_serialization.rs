use proto_dryb::{Deserialize, DeserializeError, Endianness, Serialize, SerializeError};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SimpleStruct {
    x: i32,
    y: i32,
}

fn main() {
    let simple = SimpleStruct { x: 42, y: -42 };
    let endian = Endianness::Little;
    let mut buffer = [0; 1024];

    let bytes_written = simple.serialize(&mut buffer, endian).unwrap();
    let (deserialized, bytes_read) = SimpleStruct::deserialize(&buffer, endian).unwrap();

    assert_eq!(simple, deserialized);
    assert_eq!(bytes_written, bytes_read);

    println!(
        "Roundtrip successful for {:?} with {:?} endianness",
        simple, endian
    );
}
