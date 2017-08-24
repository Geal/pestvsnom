# pest VS nom benchmarks

This repository contains the benchmarks used in [the comparison between pest and nom](https://github.com/pest-parser/pest/blob/4cdbfc767b05e0e21b3d1d7fb211139f8a6b93c4/README.md#sheer-performance).

All the files are in `benches/`:

- `benches/pest.rs` is the "pest" benchmark (fastest one, recognizes the AST but does not transform to numbers, strings, vectors, etc)
- `benches/full_pest.rs` is the "pest (custom AST)" benchmark (parses the values)
- `benches/nom.rs` is the [test JSON parser](https://github.com/Geal/nom/blob/b5d11591056b6acf93834dc26f863aff6559c8ac/benches/json.rs) for nom
- `benches/nom_f64.rs` is a version of the nom parser where numbers are parsed as `f64` instead of `f32`

## Results

```
$ cargo bench
   Compiling pestvsnom v0.1.0 (file:///Users/geal/dev/rust/projects/pestvsnom)
    Finished release [optimized] target(s) in 3.73 secs
     Running target/release/deps/pestvsnom-97707cfcfc27f95f

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/full_pest-d19d1d9a2d0599f4

running 1 test
test full_pest ... bench:  85,736,045 ns/iter (+/- 6,323,669)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out

     Running target/release/deps/nom-8768745f161d04c9

running 1 test
test nom_f32 ... bench: 299,732,664 ns/iter (+/- 10,568,900)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out

     Running target/release/deps/nom_f64-1730d2858dff8973

running 1 test
test nom_f64 ... bench:  63,901,183 ns/iter (+/- 3,180,677)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out

     Running target/release/deps/pest-46b9649508cd945f

running 1 test
test pest ... bench:  37,862,233 ns/iter (+/- 2,293,381)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out
```

From these results (run on a late 2013 Macbook Pro, CPU 2,3 GHz Intel Core i7, with `rustc 1.21.0-nightly (b75d1f0ce 2017-08-02)`), we can see that the "nom_f64" and "full_pest" benchmarks are in the same range, the "pest" parser is, as expected, faster, but the original nom parser is way slower.

As it turns out, the main cost comes from converting to a `f32` in the [`FromStr` implementation for `f32`](https://github.com/rust-lang/rust/blob/e9c74bc42db1f811820bd829f99c023d3d007628/src/libcore/num/dec2flt/mod.rs).
There might be an interesting investigation (and possibly an optimization of float parsing?) to do there.
