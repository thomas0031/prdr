use proto_dryb::{Deserialize, DeserializeError, Endianness, Serialize, SerializeError};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum CustomEnum {
    A,
    B,
    C,
}

fn main() {
    let custom_enums = vec![CustomEnum::A, CustomEnum::B, CustomEnum::C];
    let mut buffer = [0u8; 1024];
    let endian = Endianness::Little;

    let bytes_written = custom_enums.serialize(&mut buffer, endian).unwrap();
    let (deserialized, bytes_read) = Vec::<CustomEnum>::deserialize(&buffer, endian).unwrap();

    assert_eq!(custom_enums, deserialized);
    assert_eq!(bytes_written, bytes_read);

    println!(
        "Roundtrip successful for {:?} with {:?} endianness",
        custom_enums, endian
    );
}
