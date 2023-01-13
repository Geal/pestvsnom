# pest VS nom benchmarks

This repository contains the benchmarks used in [the comparison between pest and nom](https://github.com/pest-parser/pest/blob/4cdbfc767b05e0e21b3d1d7fb211139f8a6b93c4/README.md#sheer-performance).

All the files are in `benches/`:

- `benches/pest.rs` official pest parser
- `benches/faster_pest.rs` a pest parser generated by the [faster-pest](https://github.com/mubelotix/faster-pest) generator
- `benches/nom.rs` is the [test JSON parser](https://github.com/Geal/nom/blob/b5d11591056b6acf93834dc26f863aff6559c8ac/benches/json.rs) for **nom 4**. This outdated version is remains because it greatly outperforms future versions.
- `benches/nom7.rs` is the [example JSON parser](https://github.com/rust-bakery/nom/blob/main/examples/json.rs) for **nom 7**
- `benches/serde_json.rs`
- `benches/ujson4c.rs`

Only benchmarks with names sharing the same suffix are comparable.

Benchmarks suffixed `_shallow` recognize the AST without building a Rust data structure.

## Results

```
$ cargo bench
   Compiling pestvsnom v0.1.0 (/home/mubelotix/projects/pestvsnom)
    Finished bench [optimized] target(s) in 3.19s
     Running unittests src/lib.rs (target/release/deps/pestvsnom-757e2dbb8943e632)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/faster_pest.rs (target/release/deps/faster_pest-8d0810fb064854ab)

running 4 tests
test faster_pest_canada         ... bench:  14,991,354 ns/iter (+/- 502,108)
test faster_pest_canada_shallow ... bench:   4,205,600 ns/iter (+/- 167,251)
test faster_pest_data           ... bench:      10,283 ns/iter (+/- 295)
test faster_pest_data_shallow   ... bench:       3,417 ns/iter (+/- 210)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 8.84s

     Running benches/nom.rs (target/release/deps/nom-77e22cfc4fff022c)

running 2 tests
test old_nom_canada ... bench:  20,508,526 ns/iter (+/- 404,666)
test old_nom_data   ... bench:      17,131 ns/iter (+/- 722)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out; finished in 6.42s

     Running benches/nom7.rs (target/release/deps/nom7-0e056333b986f195)

running 2 tests
test nom_canada ... bench:  26,446,832 ns/iter (+/- 518,255)
test nom_data   ... bench:      21,709 ns/iter (+/- 423)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out; finished in 8.23s

     Running benches/pest.rs (target/release/deps/pest-0f795f43e6e0c3fb)

running 4 tests
test pest_canada_collect ... bench:  35,478,226 ns/iter (+/- 640,722)
test pest_canada_shallow ... bench:  29,646,016 ns/iter (+/- 453,319)
test pest_data_collect   ... bench:      28,732 ns/iter (+/- 3,105)
test pest_data_shallow   ... bench:      20,634 ns/iter (+/- 2,239)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 19.95s

     Running benches/serde_json.rs (target/release/deps/serde_json-833ccbe6872c9094)

running 2 tests
test serde_json_canada ... bench:  14,315,484 ns/iter (+/- 355,358)
test serde_json_data   ... bench:       9,806 ns/iter (+/- 429)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out; finished in 4.66s

     Running benches/ujson4c.rs (target/release/deps/ujson4c-4fc918f0c864dcf6)

running 2 tests
test ujson4c_canada ... bench:   7,749,558 ns/iter (+/- 175,905)
test ujson4c_data   ... bench:      13,839 ns/iter (+/- 442)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out; finished in 8.89s
```

From these results (run on a late `Intel® Core™ i7-6700HQ CPU @ 2.60GHz`, with `rustc 1.68.0-nightly (3020239de 2023-01-09)`), we can see that it takes pest as much time to recognize the AST as it takes nom to both recognize it *and* build the data structure. Nom outperforms pest code generated by the official generator. On the other hand, pest code generated by the faster-pest generator beats both pest, nom, and ujson4c on small files. Indeed, it parses the `canada.json` file at 705% the speed of pest and 136% the speed of nom.
