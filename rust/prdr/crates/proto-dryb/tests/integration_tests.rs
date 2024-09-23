use proto_dryb::{Deserialize, Endianness, Serialize};

#[test]
fn test_primitives() {
    let endianness = [Endianness::Little, Endianness::Big];
    for &endian in &endianness {
        // u8 and i8 (endianness doesn't matter)
        test_roundtrip(42u8, endian);
        test_roundtrip(-42i8, endian);

        // u16 and i16
        test_roundtrip(4242u16, endian);
        test_roundtrip(-4242i16, endian);

        // u32 and i32
        test_roundtrip(424242u32, endian);
        test_roundtrip(-424242i32, endian);

        // u64 and i64
        test_roundtrip(4242424242u64, endian);
        test_roundtrip(-4242424242i64, endian);

        // f32 and f64
        test_roundtrip(3.14159f32, endian);
        test_roundtrip(3.14159265359f64, endian);

        // bool
        test_roundtrip(true, endian);
        test_roundtrip(false, endian);
    }
}

#[test]
fn test_option() {
    let endianness = [Endianness::Little, Endianness::Big];
    for &endian in &endianness {
        test_roundtrip(Some(42u32), endian);
        test_roundtrip(None::<u32>, endian);
    }
}

#[test]
fn test_vec() {
    let endianness = [Endianness::Little, Endianness::Big];
    for &endian in &endianness {
        test_roundtrip(vec![1u8, 2u8, 3u8], endian);
        test_roundtrip(vec![1u32, 2u32, 3u32], endian);
        test_roundtrip(Vec::<u32>::new(), endian);
    }
}

#[test]
fn test_string() {
    let endianness = [Endianness::Little, Endianness::Big];
    for &endian in &endianness {
        test_roundtrip("Hello, world!".to_string(), endian);
        test_roundtrip("".to_string(), endian);
        test_roundtrip("🦀 Rust 💻".to_string(), endian);
    }
}

#[test]
fn test_array() {
    let arr = [1, 2, 3, 4, 5];
    test_roundtrip(arr, Endianness::Little);
    test_roundtrip(arr, Endianness::Big);
}

#[test]
fn test_array_of_arrays() {
    let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    test_roundtrip(arr, Endianness::Little);
    test_roundtrip(arr, Endianness::Big);
}

#[test]
fn test_empty_array() {
    let empty_arr: [u32; 0] = [];
    test_roundtrip(empty_arr, Endianness::Little);
    test_roundtrip(empty_arr, Endianness::Big);
}

fn test_roundtrip<T: Serialize + Deserialize + PartialEq + std::fmt::Debug>(
    value: T,
    endian: Endianness,
) {
    let mut buffer = [0u8; 1024];
    let serialized_size = value
        .serialize(&mut buffer, endian)
        .expect("Serialization failed");
    let (deserialized_value, deserialized_size) =
        T::deserialize(&buffer[..serialized_size], endian).expect("Deserialization failed");

    assert_eq!(
        value, deserialized_value,
        "Roundtrip failed for {:?} with {:?} endianness",
        value, endian
    );
    assert_eq!(
        serialized_size, deserialized_size,
        "Serialized and deserialized sizes don't match for {:?} with {:?} endianness",
        value, endian
    );
}
