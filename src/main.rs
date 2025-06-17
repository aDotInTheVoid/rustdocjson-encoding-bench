use std::error::Error;
use std::fs;

use criterion::Criterion;
use rustdoc_types::Crate;

fn cbor_to_bytes(krate: &Crate) -> Vec<u8> {
    let mut cbor_bytes = Vec::new();
    ciborium::into_writer(krate, &mut cbor_bytes).unwrap();
    cbor_bytes
}

fn benches(c: &mut Criterion, krate: &Crate) {
    let json_bytes = serde_json::to_vec(&krate).unwrap();
    let postcard_bytes = postcard::to_allocvec(&krate).unwrap();
    let cbor_bytes = cbor_to_bytes(krate);

    println!("json size:     {:>9}", json_bytes.len());
    println!("postcard size: {:>9}", postcard_bytes.len());
    println!("cbor size:     {:>9}", cbor_bytes.len());

    c.bench_function("deserialize_json", |b| {
        b.iter_with_large_drop(|| {
            serde_json::from_slice::<rustdoc_types::Crate>(&json_bytes).unwrap()
        })
    });

    c.bench_function("serialize_json", |b| {
        b.iter(|| serde_json::to_vec(&krate).unwrap())
    });

    c.bench_function("deserialize_postcard", |b| {
        b.iter_with_large_drop(|| {
            postcard::from_bytes::<rustdoc_types::Crate>(&postcard_bytes).unwrap()
        })
    });

    c.bench_function("serialize_postcard", |b| {
        b.iter(|| postcard::to_allocvec(&krate).unwrap())
    });

    c.bench_function("deserialize_cbor", |b| {
        b.iter_with_large_drop(|| ciborium::from_reader::<Crate, _>(&cbor_bytes[..]).unwrap())
    });

    c.bench_function("serialize_cbor", |b| {
        b.iter(|| postcard::to_allocvec(&krate).unwrap())
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = fs::read("./corpus/aws-sdk-ec2.json")?;
    let krate: rustdoc_types::Crate = serde_json::from_slice(&bytes)?;
    assert_eq!(krate.format_version, rustdoc_types::FORMAT_VERSION);

    let mut c = criterion::Criterion::default().configure_from_args();

    benches(&mut c, &krate);

    c.final_summary();

    Ok(())
}
