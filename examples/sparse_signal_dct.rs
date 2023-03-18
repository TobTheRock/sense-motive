use std::f64::consts::PI;

use plotly::{common::Mode, Plot, Scatter};
use rand::{distributions::Uniform, Rng};
use sense_motive::{Model, ModelBuilder, Transformation};

const N: usize = 256;
const M: usize = 64;
const K: usize = 12;

const TRANSFORM: Transformation = Transformation::None;

fn main() {
    let model = ModelBuilder::new()
        .with_transformation(TRANSFORM)
        .build::<M, N>();
    // model.sensing_matrix().transform_matrix(); // TODO add enums to select algorithm sensing matrix etc.

    // let original = generate_signal(&[0.25, 1.0]);
    let original = generate_sparse_signal(K);
    let compressed = model.compress(&original);
    let decompressed = model.decompress(&compressed);

    let mut plot = Plot::new();
    let trace = Scatter::new((0..N - 1).collect(), original.clone()).mode(Mode::Lines);
    plot.add_trace(trace);

    // let trace = Scatter::new((0..M - 1).collect(), compressed).mode(Mode::Lines);
    // plot.add_trace(trace);

    let trace = Scatter::new((0..N - 1).collect(), decompressed.clone()).mode(Mode::Lines);
    plot.add_trace(trace);

    println!("Error {}", error_l2(&original, &decompressed));
    plot.show();
}

fn error_l2(original: &Vec<f64>, decompressed: &Vec<f64>) -> f64 {
    original
        .iter()
        .zip(decompressed.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum()
}

// TODO amplitudes
fn generate_signal(frequencies: &[f64]) -> Vec<f64> {
    let mut signal = vec![0.0; N];

    for (i, val) in signal.iter_mut().enumerate() {
        *val = frequencies
            .iter()
            .map(|&f| ((i as f64 * 2.0 * PI * f).cos()))
            .sum();
    }

    signal
}

fn generate_sparse_signal(sparsity: usize) -> Vec<f64> {
    let indices = rand::thread_rng()
        .sample_iter(Uniform::new(0, N))
        .take(sparsity);
    let amplitudes = rand::thread_rng()
        .sample_iter(Uniform::new(0.0, 1.0))
        .take(sparsity);

    let mut signal = vec![0.0; N];
    for (i, a) in indices.zip(amplitudes) {
        signal[i] = a;
    }

    signal
}
