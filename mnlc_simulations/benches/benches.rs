#[macro_use]

extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use believer::ParityCheckMatrix;
use believer::{Decoder, QuantumErasureDecoder, GF4Stabilizers};
use believer::Pauli::{I, X, Z};

use std::time::Duration;
use std::collections::HashMap;



fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rank functions");
    group.sample_size(10);




    let stabilizers = GF4Stabilizers::from_sparse_paulis(
            vec![
                vec![(Z, 0), (Z, 1)],
                vec![(Z, 1), (Z, 2)],
                vec![(Z, 3), (Z, 4)],
                vec![(Z, 4), (Z, 5)],
                vec![(Z, 6), (Z, 7)],
                vec![(Z, 7), (Z, 8)],
                vec![(X, 0), (X, 1), (X, 2), (X, 3), (X, 4), (X, 5)],
                vec![(X, 3), (X, 4), (X, 5), (X, 6), (X, 7), (X, 8)],
            ],
            9,
        );

    let m = stabilizers.merge();

    let mut m_tmp = m.tmp_rank_pcm();
    let mut v_tmp = Vec::with_capacity(18);

    let mut decoder = QuantumErasureDecoder::new(stabilizers, 0.1);

    group.bench_function("Test decode", |b| b.iter(||  decoder.decode(&vec![0, 1, 3, 4])));

    group.bench_function("Test simul", |b| b.iter(||  decoder.simulate_until_n_events_are_found(20)));

    group.bench_function("Test rank", |b| b.iter(||  m.rank()));

    group.bench_function("Test rank_mut", |b| b.iter(||  m.rank_mut(&mut m_tmp, &mut v_tmp)));

    println!("rate:{}",decoder.simulate_until_n_events_are_found(20).get_failure_rate());

    

    group.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
