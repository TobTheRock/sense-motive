use std::f64::consts::PI;

use plotly::{
    common::{Marker, MarkerSymbol, Mode},
    Plot, Scatter,
};
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    Rng,
};
use sense_motive::{Model, ModelBuilder, Transformation};

const N: usize = 64;
const M: usize = 32;
const K: usize = 8;

const TRANSFORM: Transformation = Transformation::Fourier1dInverse;

fn main() {
    let model = ModelBuilder::new()
        .with_transformation(TRANSFORM)
        .build(M, N);

    let original = generate_cos_signal(K);
    // let original = match TRANSFORM {
    //     Transformation::None => generate_sparse_signal(K),
    //     Transformation::Dct1dInverse => generate_cos_signal(K),
    //     Transformation::Dct1d => todo!(),
    //     Transformation::Fourier1dInverse => todo!(),
    //     Transformation::Fourier1d => todo!(),
    // };

    let compressed = model.compress(&original);
    let decompressed = model.decompress(&compressed);

    let mut plot = Plot::new();
    let trace = Scatter::new((0..N - 1).collect(), original.clone())
        .name("Original")
        .mode(Mode::LinesMarkers)
        .marker(Marker::new().symbol(MarkerSymbol::Cross));
    plot.add_trace(trace);

    // TODO 2nd trace of compressed
    // let trace = Scatter::new((0..M - 1).collect(), compressed).mode(Mode::Lines);
    // plot.add_trace(trace);

    let trace = Scatter::new((0..N - 1).collect(), decompressed.clone())
        .name("Decompressed")
        .mode(Mode::LinesMarkersText)
        .marker(Marker::new().symbol(MarkerSymbol::Circle));
    plot.add_trace(trace);

    println!(
        "Error (normalized to N) {}",
        error_l2(&original, &decompressed) / (N as f64)
    );
    plot.show();
}

fn error_l2(original: &Vec<f64>, decompressed: &Vec<f64>) -> f64 {
    original
        .iter()
        .zip(decompressed.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum()
}

fn generate_cos_signal(sparsity: usize) -> Vec<f64> {
    let frequencies = random_values(sparsity, 0.0, 10.0);
    let amplitudes = random_values(sparsity, 0.0, 1.0);
    let params: Vec<(f64, f64)> = frequencies.zip(amplitudes).collect();

    (0..N)
        .map(|i| i as f64 / N as f64)
        .map(|i| {
            params
                .iter()
                .map(|(f, a)| (a * (i * 2.0 * PI * f).cos()))
                .sum()
        })
        .collect()
}

fn generate_sparse_signal(sparsity: usize) -> Vec<f64> {
    let indices = random_values(sparsity, 0, N);
    let amplitudes = random_values(sparsity, 0.0, 1.0);

    let mut signal = vec![0.0; N];
    for (i, a) in indices.zip(amplitudes) {
        signal[i] = a;
    }

    signal
}

fn random_values<T>(n: usize, min: T, max: T) -> impl Iterator<Item = T>
where
    T: SampleUniform,
{
    rand::thread_rng()
        .sample_iter(Uniform::new(min, max))
        .take(n)
}
