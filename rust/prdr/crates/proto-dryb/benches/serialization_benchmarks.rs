use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proto_dryb::{Deserialize, DeserializeError, Endianness, Serialize, SerializeError};

// Custom struct for benchmarking
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

// Custom enum for benchmarking
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum CustomEnum {
    A(u32),
    B(String),
    C { x: f64, y: f64 },
}

fn criterion_benchmark(c: &mut Criterion) {
    // Benchmark u8
    c.bench_function("serialize u8", |b| {
        let value: u8 = 42;
        let mut buffer = [0u8; 1];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark i32
    c.bench_function("serialize i32", |b| {
        let value: i32 = -1234567;
        let mut buffer = [0u8; 4];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark f64
    c.bench_function("serialize f64", |b| {
        let value: f64 = 3.14159265359;
        let mut buffer = [0u8; 8];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark String
    c.bench_function("serialize String", |b| {
        let value = "Hello, world!".to_string();
        let mut buffer = [0u8; 100];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark Vec<i32>
    c.bench_function("serialize Vec<i32>", |b| {
        let value: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut buffer = [0u8; 100];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark Option<String>
    c.bench_function("serialize Option<String>", |b| {
        let value: Option<String> = Some("Optional value".to_string());
        let mut buffer = [0u8; 100];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark array
    c.bench_function("serialize [u32; 5]", |b| {
        let value: [u32; 5] = [1, 2, 3, 4, 5];
        let mut buffer = [0u8; 100];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark custom struct
    c.bench_function("serialize Point", |b| {
        let value = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mut buffer = [0u8; 100];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Benchmark custom enum
    c.bench_function("serialize CustomEnum", |b| {
        let value = CustomEnum::C { x: 3.14, y: 2.718 };
        let mut buffer = [0u8; 100];
        b.iter(|| {
            black_box(value.serialize(&mut buffer, Endianness::Little)).unwrap();
        })
    });

    // Deserialization benchmarks
    c.bench_function("deserialize u8", |b| {
        let value: u8 = 42;
        let mut buffer = [0u8; 1];
        value.serialize(&mut buffer, Endianness::Little).unwrap();
        b.iter(|| {
            black_box(u8::deserialize(&buffer, Endianness::Little)).unwrap();
        })
    });

    c.bench_function("deserialize String", |b| {
        let value = "Hello, world!".to_string();
        let mut buffer = [0u8; 100];
        let len = value.serialize(&mut buffer, Endianness::Little).unwrap();
        b.iter(|| {
            black_box(String::deserialize(&buffer[..len], Endianness::Little)).unwrap();
        })
    });

    c.bench_function("deserialize Vec<i32>", |b| {
        let value: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut buffer = [0u8; 100];
        let len = value.serialize(&mut buffer, Endianness::Little).unwrap();
        b.iter(|| {
            black_box(Vec::<i32>::deserialize(&buffer[..len], Endianness::Little)).unwrap();
        })
    });

    c.bench_function("deserialize Point", |b| {
        let value = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mut buffer = [0u8; 100];
        let len = value.serialize(&mut buffer, Endianness::Little).unwrap();
        b.iter(|| {
            black_box(Point::deserialize(&buffer[..len], Endianness::Little)).unwrap();
        })
    });

    c.bench_function("deserialize CustomEnum", |b| {
        let value = CustomEnum::C { x: 3.14, y: 2.718 };
        let mut buffer = [0u8; 100];
        let len = value.serialize(&mut buffer, Endianness::Little).unwrap();
        b.iter(|| {
            black_box(CustomEnum::deserialize(&buffer[..len], Endianness::Little)).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

