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
bench_query_parse ... bench:       6,886 ns/iter (+/- 46) = 192854 MB/s
bench_query_scan  ... bench:      51,347 ns/iter (+/- 1,040) = 25863 MB/s
```
