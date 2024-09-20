use proto_dryb::{Deserialize, Endianness, Serialize};

fn main() {
    let foo = Some(vec![
        Some(String::from("Hello, world!")),
        None,
        Some(String::from("🦀 Rust 💻")),
    ]);
    let endian = Endianness::Little;
    let mut buffer = [0; 1024];

    foo.serialize(&mut buffer, endian).unwrap();

    let (deserialized, _) = Option::<Vec<Option<String>>>::deserialize(&buffer, endian).unwrap();

    assert_eq!(foo, deserialized);

    println!(
        "Roundtrip successful for {:?} with {:?} endianness",
        foo, endian
    );
}
