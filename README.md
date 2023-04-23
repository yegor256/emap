[![cargo](https://github.com/yegor256/emap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/emap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/emap.svg)](https://crates.io/crates/emap)
[![codecov](https://codecov.io/gh/yegor256/emap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/emap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/emap)](https://hitsofcode.com/view/github/yegor256/emap)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/emap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/emap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/emap)](https://docs.rs/emap/latest/emap/)

It is an alternative implementation of a map in Rust, which works much faster under the following conditions:

  * Keys are `usize`
  * Keys used sequentially (e.g., the 5th key is inserted only when 0..4th are in the map)
  * Values implement `Copy`

See the [benchmarking results](#benchmark) below.

First, add this to `Cargo.toml`:

```toml
[dependencies]
emap = "0.0.0"
```

Then, use it like a standard hash map... well, almost:

```rust
use emap::Map;
let mut m : Map<&str, 100> = Map::new(); // allocation on stack
m.insert(1, "foo");
m.insert(42, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `100`. This is 
the total size of the map, which is allocated on stack when `::new()` is called. 
If more than 100 keys will be added to the map, it will panic.

Read [the API documentation](https://docs.rs/emap/latest/emap/). 
The struct
[`emap::Map`](https://docs.rs/emap/latest/emap/struct.Map.html) is designed as closely similar to 
[`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `emap::Map` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions 
([`benchmark.rs`](https://github.com/yegor256/emap/blob/master/tests/benchmark.rs)) 
to them and measured how fast they performed. In the following table, 
the numbers over 1.0 indicate performance gain, 
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 1 | 10 | 100 | 1000 | 10000 |
| --- | --: | --: | --: | --: | --: |
| `emap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `hashbrown::HashMap` | 28.50 | 15.35 | 10.73 | 10.99 | 11.23 |
| `indexmap::IndexMap` | 22.50 | 31.04 | 25.78 | 26.91 | 28.09 |
| `linear_map::LinearMap` | 4.50 | 5.26 | 32.47 | 251.73 | 2K |
| `linked_hash_map::LinkedHashMap` | 34.00 | 60.74 | 36.57 | 37.04 | 38.15 |
| `litemap::LiteMap` | 6.50 | 7.74 | 13.02 | 62.06 | 975.18 |
| `nohash_hasher::BuildNoHashHasher` | 15.00 | 18.57 | 9.65 | 10.00 | 10.50 |
| `rustc_hash::FxHashMap` | 16.00 | 14.78 | 9.94 | 10.80 | 11.90 |
| `std::collections::BTreeMap` | 25.50 | 28.00 | 27.11 | 52.21 | 56.15 |
| `std::collections::HashMap` | 56.00 | 29.48 | 27.26 | 26.63 | 26.56 |
| `tinymap::array_map::ArrayMap` | 2.50 | 17.83 | 128.34 | 1K | 12K |

The experiment was performed on 23-04-2023.
 There were 100 repetition cycles.
 The entire benchmark took 142s.

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
