use criterion::{
    black_box, criterion_group, criterion_main, Bencher, Criterion, ParameterizedBenchmark,
    Throughput,
};
use rand::Rng;

fn do_encode_bench(b: &mut Bencher, &size: &usize) {
    let data = {
        let mut v: Vec<u8> = vec![0; size];
        rand::thread_rng().fill(v.as_mut_slice());
        v
    };
    b.iter(|| {
        black_box(&base32::encode(&data));
    });
}

fn do_decode_bench(b: &mut Bencher, &size: &usize) {
    let data = {
        let mut v: Vec<u8> = vec![0; size];
        rand::thread_rng().fill(v.as_mut_slice());
        base32::encode(v)
    };
    b.iter(|| {
        black_box(&base32::decode(&data).unwrap());
    });
}

fn encode_benchmarks(byte_sizes: &[usize]) -> ParameterizedBenchmark<usize> {
    ParameterizedBenchmark::new("encode", do_encode_bench, byte_sizes.iter().cloned())
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(3))
        .throughput(|&size| Throughput::Bytes(size as u64))
}

fn decode_benchmarks(byte_sizes: &[usize]) -> ParameterizedBenchmark<usize> {
    ParameterizedBenchmark::new("decode", do_decode_bench, byte_sizes.iter().cloned())
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(3))
        .throughput(|&size| Throughput::Bytes(size as u64))
}

const SMALL_BYTE_SIZES: [usize; 5] = [3, 50, 100, 500, 3 * 1024];
const LARGE_BYTE_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

fn benchmark(c: &mut Criterion) {
    c.bench("bench_small_input", encode_benchmarks(&SMALL_BYTE_SIZES));
    c.bench(
        "bench_large_input",
        encode_benchmarks(&LARGE_BYTE_SIZES).sample_size(10),
    );

    c.bench("bench_small_input", decode_benchmarks(&SMALL_BYTE_SIZES));
    c.bench(
        "bench_large_input",
        decode_benchmarks(&LARGE_BYTE_SIZES).sample_size(10),
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
