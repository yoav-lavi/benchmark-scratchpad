# Benchmark Scratchpad

A quick scratchpad for benchmarking Rust code

## Prerequisites

- [cargo-make](https://github.com/sagiegurari/cargo-make) - `cargo install --force cargo-make`

## Instructions

1. Edit [benches/benchmark.rs](benches/benchmark.rs)
2. Run `cargo make bench`

## `benchmarks!`

The `benchmarks!` macro accepts input with the following structure:

```rust
benchmarks! {
  benchmark_name { /* benchmark impl */ }
  other_benchmark_name { /* other benchmark impl */ }
}
```

The [criterion.rs](https://github.com/bheisler/criterion.rs) `black_box` function is included and usable within the macro.

`criterion::black_box` stops the compiler from constant-folding away the whole function and replacing it with a constant.

You can you the keyword `@skip` before a benchmark for it not to be run:

```rust
benchmarks! {
  @skip benchmark_name { /* benchmark impl */ }
}
```

## Tasks

- `cargo make bench` - runs the benchmark via `cargo-criterion`
- `cargo make bench-verbose` - runs the benchmark via `cargo-criterion` with verbose output
- `cargo make reset` - resets the criterion data for earlier benchmarks
- `cargo make new` - returns [benches/benchmark.rs](benches/benchmark.rs) to the initial state
- `cargo make save [FILENAME]` - saves the contents of [benches/benchmark.rs](benches/benchmark.rs) to `saved/[FILENAME].rs`

## Example

```rust 
mod internals;

benchmarks! {
    collect_vec_string {
        let vector = black_box(vec!["hello", "world"]);
        let _output = vector
            .iter()
            .map(|item| item.to_uppercase())
            .collect::<Vec<String>>()
            .join("");
    }

    collect_string {
        let vector = black_box(vec!["hello", "world"]);
        let _output = vector
            .iter()
            .map(|item| item.to_uppercase())
            .collect::<String>();
    }
}
```


## Uses 

- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [cargo-criterion](https://github.com/bheisler/cargo-criterion)
- [criterion.rs](https://github.com/bheisler/criterion.rs)
