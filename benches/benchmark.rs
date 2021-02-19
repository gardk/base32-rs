use std::time::Duration;

use base32::STANDARD;
use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, Bencher, BenchmarkGroup,
    BenchmarkId, Criterion, Throughput,
};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, SmallRng},
    SeedableRng,
};

fn do_encode_benchmark(b: &mut Bencher<'_>, input_data: &[u8]) {
    b.iter(|| black_box(STANDARD.encode(&input_data)));
}

fn do_encode_to_slice_benchmark(b: &mut Bencher<'_>, input_data: &[u8]) {
    let encoded_size = STANDARD.encoded_size(input_data.len()).unwrap();
    let mut output = vec![0; encoded_size];

    b.iter(|| {
        assert_eq!(
            STANDARD.encode_to_slice(&mut output, &input_data),
            encoded_size
        )
    });
}

fn do_decode_benchmark(b: &mut Bencher<'_>, input_data: &str) {
    b.iter(|| black_box(STANDARD.decode(input_data)))
}

fn do_decode_to_slice_benchmark(b: &mut Bencher<'_>, input_data: &[u8]) {
    let decoded_size = STANDARD.decoded_size(input_data.len()).unwrap();
    let mut output = vec![0; decoded_size];

    b.iter(|| black_box(STANDARD.decode_to_slice(&mut output, input_data).unwrap()));
}

fn generate_random_vec(size: usize) -> Vec<u8> {
    Uniform::new_inclusive(u8::MIN, u8::MAX)
        .sample_iter(SmallRng::from_entropy())
        .take(size)
        .collect()
}

const SMALL_INPUT_SIZES: [usize; 5] = [3, 50, 100, 500, 3 * 1024];
const LARGE_INPUT_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

fn encode_benchmarks(group: &mut BenchmarkGroup<'_, WallTime>, input_sizes: &[usize]) {
    for input_bytes in input_sizes {
        let input_data = generate_random_vec(*input_bytes);

        group
            .throughput(Throughput::Bytes(*input_bytes as u64))
            .bench_with_input(
                BenchmarkId::new("encode", input_bytes),
                input_data.as_slice(),
                do_encode_benchmark,
            )
            .bench_with_input(
                BenchmarkId::new("encode_to_slice", input_bytes),
                input_data.as_slice(),
                do_encode_to_slice_benchmark,
            );
    }
}

fn decode_benchmarks(group: &mut BenchmarkGroup<'_, WallTime>, input_sizes: &[usize]) {
    for input_bytes in input_sizes {
        let input_data = STANDARD.encode(generate_random_vec(*input_bytes));

        group
            .throughput(Throughput::Bytes(*input_bytes as u64))
            .bench_with_input(
                BenchmarkId::new("decode", input_bytes),
                input_data.as_str(),
                do_decode_benchmark,
            )
            .bench_with_input(
                BenchmarkId::new("decode_to_slice", input_bytes),
                input_data.as_bytes(),
                do_decode_to_slice_benchmark,
            );
    }
}

fn benchmarks(c: &mut Criterion) {
    encode_benchmarks(
        c.benchmark_group("encode_small_input")
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(3)),
        &SMALL_INPUT_SIZES,
    );
    encode_benchmarks(
        c.benchmark_group("encode_large_input")
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(3))
            .sample_size(10),
        &LARGE_INPUT_SIZES,
    );

    decode_benchmarks(
        c.benchmark_group("decode_small_input")
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(3)),
        &SMALL_INPUT_SIZES,
    );
    decode_benchmarks(
        c.benchmark_group("decode_large_input")
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(3))
            .sample_size(10),
        &LARGE_INPUT_SIZES,
    );
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
