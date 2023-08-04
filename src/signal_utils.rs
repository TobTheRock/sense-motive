use std::f64::consts::PI;

use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    Rng,
};

pub fn diff(original: &Vec<f64>, decompressed: &Vec<f64>) -> Vec<f64> {
    original
        .iter()
        .zip(decompressed.iter())
        .map(|(a, b)| (a - b))
        .collect()
}

pub fn error_l2(original: &Vec<f64>, decompressed: &Vec<f64>) -> f64 {
    original
        .iter()
        .zip(decompressed.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum()
}
pub fn error_l2_norm(original: &Vec<f64>, decompressed: &Vec<f64>) -> f64 {
    error_l2(original, decompressed) / (original.len() as f64)
}

pub fn generate_cos_signal(len: usize, sparsity: usize) -> Vec<f64> {
    let frequencies = random_values(sparsity, 0.0, 10.0);
    let amplitudes = random_values(sparsity, 0.0, 1.0);
    let params: Vec<(f64, f64)> = frequencies.zip(amplitudes).collect();
    (0..len)
        .map(|i| i as f64 / len as f64)
        .map(|i| {
            params
                .iter()
                .map(|(f, a)| (a * (i * 2.0 * PI * f).cos()))
                .sum()
        })
        .collect()
}

pub fn generate_sparse_signal(len: usize, sparsity: usize) -> Vec<f64> {
    let indices = random_values(sparsity, 0, len);
    let amplitudes = random_values(sparsity, 0.0, 1.0);

    let mut signal = vec![0.0; len];
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
