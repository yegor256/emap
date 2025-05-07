# The Fastest Map with `usize` Keys and Fixed Capacity

[![cargo](https://github.com/yegor256/emap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/emap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/emap.svg)](https://crates.io/crates/emap)
[![codecov](https://codecov.io/gh/yegor256/emap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/emap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/emap)](https://hitsofcode.com/view/github/yegor256/emap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/emap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/emap)](https://docs.rs/emap/latest/emap/)

The [`emap::Map`][Map] is the fastest possible [associative array] in Rust,
  with `usize` keys.
It's by the order of magnitude faster than the standard
  [`std::collections::HashMap`][HashMap].
It's also faster than [`std::vec::Vec`][Vec].
However, the following restrictions apply:

* you must know the total capacity upfront
* you must account for a memory overhead of `2 * usize` per element
* you must use `next_key()` method, of _O(1)_, to find the next available key

First, add this to `Cargo.toml`:

```toml
[dependencies]
emap = "0.0.13"
```

Then, use it like a standard hash map... well, almost:

```rust
use emap::Map;
let mut m : Map<&str> = Map::with_capacity_init(100); // allocation on heap
m.insert(m.next_key(), "foo");
m.insert(m.next_key(), "bar");
assert_eq!(2, m.len());
```

If more than 100 keys will be added to the map, it will panic.
The map doesn't increase its size automatically, like [`Vec`][Vec] does
(this is one of the reasons why we are faster).

Read [the API documentation](https://docs.rs/emap/latest/emap/).
The struct [`emap::Map`][Map] is designed as closely similar to
[`std::collections::HashMap`][HashMap] as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `emap::Map` with
`Vec`, changing the total capacity `CAP` of them (horizontal axis).
We applied the same interactions
([`benchmark.rs`][benchmark])
to them both and measured how fast they performed. In the following table,
the numbers over 1.0 indicate performance gain of `Map` against `Vec`,
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 4 | 16 | 256 | 4096 |
| --- | --: | --: | --: | --: |
| `i ∈ 0..CAP {M.insert(i, &"Hello, world!")}` |1.08 |1.89 |2.35 |2.14 |
| `i ∈ 0..CAP {M.insert(i, &"大家好"); s ∈ M.values() {sum += s.len()}}` |1.20 |0.88 |0.34 |0.51 |
| `i ∈ 0..CAP {M.insert(i, &42); s ∈ M.into_values() {sum += s}}` |1.45 |0.93 |0.51 |0.52 |
| `i ∈ 0..CAP {M.insert(i, &42); s ∈ M.keys() {sum += s}}` |1.09 |0.58 |0.34 |0.51 |
| `i ∈ 0..CAP {M.insert(i, &42); s ∈ M.values() {sum += s}}` |1.03 |0.57 |0.51 |0.51 |
| `i ∈ 0..CAP {M.insert(i, &42)}; M.clear(); M.len();` |1.27 |1.84 |6.71 |7.61 |
| `i ∈ 0..CAP {M.insert(i, &42)}; i ∈ CAP-1..0 {M.remove(&i)}` |1.13 |2.13 |2.01 |2.15 |

The experiment was performed on 21-08-2023.
 There were 10000 repetition cycles.
 The entire benchmark took 423s.

<!-- benchmark -->

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
cargo test -vv
```

If everything goes well, fork repository, make changes,
send us a
[pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also,
run `cargo fmt` and `cargo clippy`.

Also, before you start making changes, run benchmarks:

```bash
cargo bench
```

Then, after the changes you make, run it again. Compare the results.
If your changes degrade performance, think twice before submitting
a pull request.

[Map]: https://docs.rs/emap/0.0.13/emap/struct.Map.html
[HashMap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[benchmark]: https://github.com/yegor256/emap/blob/master/tests/benchmark.rs
[associative array]: https://en.wikipedia.org/wiki/Associative_array
