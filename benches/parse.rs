extern crate core_affinity;
#[macro_use]
extern crate criterion;

#[cfg(feature = "jemallocator")]
extern crate jemallocator;
#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use criterion::{BatchSize, Criterion, ParameterizedBenchmark, Throughput};
#[cfg(feature = "bench-serder")]
use serde_json;
use simd_json;
#[cfg(feature = "simd_json-rust")]
use simd_json_rust::build_parsed_json;
use std::fs::File;
use std::io::Read;

macro_rules! bench_file {
    ($name:ident) => {
        fn $name(c: &mut Criterion) {
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(core_ids[0]);

            let mut vec = Vec::new();
            File::open(concat!("data/", stringify!($name), ".json"))
                .unwrap()
                .read_to_end(&mut vec)
                .unwrap();

            let b = ParameterizedBenchmark::new(
                "simd_json",
                |b, data| {
                    b.iter_batched(
                        || data.clone(),
                        |mut bytes| {
                            simd_json::to_borrowed_value(&mut bytes).unwrap();
                        },
                        BatchSize::SmallInput,
                    )
                },
                vec![vec],
            );
            let b = b.with_function("simd_json-owned", |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut bytes| {
                        simd_json::to_owned_value(&mut bytes).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            });
            #[cfg(feature = "simd_json-rust")]
            let b = b.with_function("simd_json_cpp", move |b, data| {
                b.iter_batched(
                    || String::from_utf8(data.to_vec()).unwrap(),
                    |bytes| {
                        let _ = build_parsed_json(&bytes, true);
                    },
                    BatchSize::SmallInput,
                )
            });
            #[cfg(feature = "bench-serde")]
            let b = b.with_function("serde_json", move |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut bytes| {
                        let _: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            });
            c.bench(
                stringify!($name),
                b.throughput(|data| Throughput::Bytes(data.len() as u64)),
            );
        }
    };
}

bench_file!(apache_builds);
bench_file!(canada);
bench_file!(citm_catalog);
bench_file!(log);
bench_file!(twitter);

criterion_group!(benches, apache_builds, canada, citm_catalog, log, twitter);
criterion_main!(benches);
