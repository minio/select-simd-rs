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

```
bench_scan  ... bench:  62,231,899 ns/iter (+/- 425,512) = 10865 MB/s
bench_parse ... bench:  90,598,073 ns/iter (+/- 650,543) = 7463 MB/s
bench_eval  ... bench:  37,424,982 ns/iter (+/- 455,449) = 18067 MB/s
```
