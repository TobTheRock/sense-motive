use approx::assert_relative_eq;
use sense_motive::{
    signal_utils::{diff, error_l2, error_l2_norm, generate_cos_signal, generate_sparse_signal},
    ModelBuilder, Transformation,
};

const N: usize = 128; // original length
const M: usize = 64; // compressed length
const K: usize = 4; // sparsity

const TOL_ERR: f64 = 0.1;
// TODO add setting the seed to the API
// const SEED: u64 = 42;

#[test]
fn reconstruct_with_bernoulli() {
    let model = ModelBuilder::new()
        // .with_algorithm(Algorithm::MatchingPursuit(MatchingPursuitSolver {
        //     max_iter: 1000,
        //     tolerance: 0.1,
        // }))
        .with_transformation(Transformation::None)
        .build(M, N);

    let original = generate_sparse_signal(N, K);

    let compressed = model.compress(&original);
    let decompressed = model.decompress(&compressed);

    dbg!(diff(&original, &decompressed));
    assert_relative_eq!(error_l2(&original, &decompressed), 0.0, epsilon = TOL_ERR);
}

#[test]
fn reconstruct_with_bernoulli_and_dct() {
    let model = ModelBuilder::new()
        // .with_algorithm(Algorithm::MatchingPursuit(MatchingPursuitSolver {
        //     max_iter: 1000,
        //     tolerance: 0.1,
        // }))
        .with_transformation(Transformation::Dct1dInverse)
        .build(M, N);

    let original = generate_cos_signal(N, K);

    let compressed = model.compress(&original);
    let decompressed = model.decompress(&compressed);

    dbg!(diff(&original, &decompressed));

    assert_relative_eq!(
        error_l2_norm(&original, &decompressed),
        0.0,
        epsilon = TOL_ERR
    )
}
