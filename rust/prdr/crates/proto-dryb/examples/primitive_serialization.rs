use proto_dryb::{Deserialize, Endianness, Serialize};

fn main() {
    let number = 42069;
    let endian = Endianness::Little;
    let mut buffer = [0; 1024];

    number.serialize(&mut buffer, endian).unwrap();

    let (deserialized, _) = i32::deserialize(&buffer, endian).unwrap();

    assert_eq!(number, deserialized);

    println!("Roundtrip successful for {} with {:?} endianness", number, endian);
}
