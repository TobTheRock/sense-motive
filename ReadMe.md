# Sense Motive

A library which aims to implement certain  [Compressed Sensing](https://en.wikipedia.org/wiki/Compressed_sensing) algorithms

## TODO
- [] Basic Structure, implement OMP

## Concepts
Input vector uncompressed: N -> input compressed M -> output vector decompressed: N
Sensing Matrix: MxN compression, decompression, subgaussian distributed (normal, brenoulli distributed...)
Transformation Matrix: NxN decompression, e.g. DCT, fourier

UncompressedVec(Vec<T>) -> compress(SensingMatrix<T>) -> CompressedVec<T>
CompressedVec -> decompress(Algorithm, Sensing, Transform)
