[![cargo](https://github.com/yegor256/emap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/emap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/emap.svg)](https://crates.io/crates/emap)
[![codecov](https://codecov.io/gh/yegor256/emap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/emap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/emap)](https://hitsofcode.com/view/github/yegor256/emap)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/emap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/emap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/emap)](https://docs.rs/emap/latest/emap/)

It is an alternative on-heap implementation of a map with keys of type `usize`
and a fixed capacity. It works much faster than a standard `HashMap` 
because it allocates memory for all keys at once and then the cost
of `get()` is _O(1)_. Obviously, with this design, the cost of `iter()` increases because the iterator
has to jump through empty keys. However, there
is a supplementary function `next_key()`, which returns the next available key in the map. 
It is recommended to use it in order to ensure sequential order of the keys, which
will guarantee _O(1)_ cost of `next()` in iterators.

If `usize` keys are placed sequentially, the only true competitor of ours is 
[`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
We beat it too, see the [benchmarking results](#benchmark) below.

First, add this to `Cargo.toml`:

```toml
[dependencies]
emap = "0.0.6"
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
The map doesn't increase its size automatically, like `Vec` does 
(this is one of the reasons why we are faster).

Read [the API documentation](https://docs.rs/emap/latest/emap/). 
The struct
[`emap::Map`](https://docs.rs/emap/latest/emap/struct.Map.html) is designed as closely similar to 
[`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `emap::Map` with
`Vec`, changing the total capacity `CAP` of them (horizontal axis).
We applied the same interactions 
([`benchmark.rs`](https://github.com/yegor256/emap/blob/master/tests/benchmark.rs)) 
to them both and measured how fast they performed. In the following table, 
the numbers over 1.0 indicate performance gain of `Map` against `Vec`, 
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 4 | 16 | 256 | 4096 |
| --- | --: | --: | --: | --: |
| `i ∈ 0..CAP {M.insert(i, &"Hello, world!")}` |1.09 |1.97 |2.18 |2.15 |
| `i ∈ 0..CAP {M.insert(i, &"大家好"); s ∈ M.values() {sum += s.len()}}` |1.09 |0.59 |0.51 |0.35 |
| `i ∈ 0..CAP {M.insert(i, &42); s ∈ M.into_values() {sum += s}}` |1.10 |0.78 |0.34 |0.35 |
| `i ∈ 0..CAP {M.insert(i, &42); s ∈ M.keys() {sum += s}}` |0.95 |0.50 |0.48 |0.52 |
| `i ∈ 0..CAP {M.insert(i, &42); s ∈ M.values() {sum += s}}` |1.20 |0.57 |0.51 |0.52 |
| `i ∈ 0..CAP {M.insert(i, &42)}; M.clear(); M.len();` |1.08 |1.85 |6.47 |7.53 |
| `i ∈ 0..CAP {M.insert(i, &42)}; i ∈ CAP-1..0 {M.remove(&i)}` |1.21 |1.91 |2.48 |2.18 |

The experiment was performed on 26-04-2023.
 There were 10000 repetition cycles.
 The entire benchmark took 485s.

<!-- benchmark -->

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
$ cargo test -vv
```

If everything goes well, fork repository, make changes, 
send us a [pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also, 
run `cargo fmt` and `cargo clippy`.
