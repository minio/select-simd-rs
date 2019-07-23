# select-simd-rs

This is a Rust package for high performance execution of select statements on large CSV objects.

## Works in progress

This package is in early development and does not yet cover a wide range of operations.

## Design Goals

`select-simd-rs` has been designed with the following goals in mind:

- zero copy behavior
- low memory footprint
- leverage SIMD instructions
- support for streaming (chunking)
- support large objects (> 4 GB)
- embedded assembly

## Performance

### Single core

bench_query_scan ... bench:      50,571 ns/iter (+/- 763) --> 24.5 GB/s
