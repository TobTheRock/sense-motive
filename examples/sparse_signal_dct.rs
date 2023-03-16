use std::f64::consts::PI;

use plotly::{common::Mode, Plot, Scatter};
use sense_motive::Model;

const N: usize = 16;
const M: usize = 8;

// const N_

fn main() {
    let mut model = Model::<M, N>::new();
    // model.sensing_matrix().transform_matrix(); // TODO add enums to select algorithm sensing matrix etc.

    // let original = generate_signal(&[0.25, 1.0]);
    let original = generate_sparse_signal(&[1, 8]);
    let compressed = model.compress(&original);
    let decompressed = model.decompress(&compressed);

    let mut plot = Plot::new();
    let trace = Scatter::new((0..N - 1).collect(), original.clone()).mode(Mode::Lines);
    plot.add_trace(trace);

    // let trace = Scatter::new((0..M - 1).collect(), compressed).mode(Mode::Lines);
    // plot.add_trace(trace);

    let trace = Scatter::new((0..N - 1).collect(), decompressed.clone()).mode(Mode::Lines);
    plot.add_trace(trace);

    plot.show();

    let error: f64 = original
        .iter()
        .zip(decompressed.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum();
    println!("Error {}", error);
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

fn generate_sparse_signal(indices: &[usize]) -> Vec<f64> {
    let mut signal = vec![0.0; N];

    for &i in indices {
        signal[i] = 1.0;
    }

    signal
}
