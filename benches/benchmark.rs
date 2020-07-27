use std::time::Duration;

use base32::STANDARD;
use criterion::{
    criterion_group, criterion_main, Bencher, Criterion, ParameterizedBenchmark, Throughput,
};
use rand::Rng;

fn bench_encode(b: &mut Bencher, &size: &usize) {
    let input = {
        let mut v: Vec<u8> = vec![0; size];
        rand::thread_rng().fill(v.as_mut_slice());
        v
    };
    b.iter(|| base32::encode(&input));
}

fn bench_encode_to_slice(b: &mut Bencher, &size: &usize) {
    let input = {
        let mut v: Vec<u8> = vec![0; size];
        rand::thread_rng().fill(v.as_mut_slice());
        v
    };
    let mut output = vec![0; STANDARD.encoded_size(input.len())];

    b.iter(|| STANDARD.encode_to_slice(&mut output, &input));
}

fn bench_decode(b: &mut Bencher, &size: &usize) {
    let input = {
        let mut v: Vec<u8> = vec![0; size];
        rand::thread_rng().fill(v.as_mut_slice());
        base32::encode(v)
    };
    b.iter(|| base32::decode(&input).unwrap());
}

fn bench_decode_to_slice(b: &mut Bencher, &size: &usize) {
    let input = {
        let mut v: Vec<u8> = vec![0; size];
        rand::thread_rng().fill(v.as_mut_slice());
        base32::encode(v)
    };
    let mut output = vec![0; STANDARD.decoded_size(input.len())];

    b.iter(|| {
        STANDARD
            .decode_to_slice(&mut output, input.as_bytes())
            .unwrap()
    });
}

fn encode_benchmarks(byte_sizes: &[usize]) -> ParameterizedBenchmark<usize> {
    ParameterizedBenchmark::new("encode", bench_encode, byte_sizes.iter().cloned())
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(3))
        .throughput(|&size| Throughput::Bytes(size as u64))
        .with_function("encode_to_slice", bench_encode_to_slice)
}

fn decode_benchmarks(byte_sizes: &[usize]) -> ParameterizedBenchmark<usize> {
    ParameterizedBenchmark::new("decode", bench_decode, byte_sizes.iter().cloned())
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(3))
        .throughput(|&size| Throughput::Bytes(size as u64))
        .with_function("decode_to_slice", bench_decode_to_slice)
}

const SMALL_BYTE_SIZES: [usize; 5] = [3, 50, 100, 500, 3072];
const LARGE_BYTE_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

fn benchmarks(c: &mut Criterion) {
    c.bench("small_input", encode_benchmarks(&SMALL_BYTE_SIZES));
    c.bench(
        "large_input",
        encode_benchmarks(&LARGE_BYTE_SIZES).sample_size(10),
    );

    c.bench("small_input", decode_benchmarks(&SMALL_BYTE_SIZES));
    c.bench(
        "large_input",
        decode_benchmarks(&LARGE_BYTE_SIZES).sample_size(10),
    );
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
