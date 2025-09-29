// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use emap::Map;
use intmap::IntMap;
use std::env;
use std::time::{Duration, Instant};

const CAP: usize = 10;
type FastIntMap<'a> = IntMap<u64, &'a i32>;
type FastIntMapStr<'a> = IntMap<u64, &'a str>;

macro_rules! measure {
    ($total:expr, $e:expr) => {{
        let start = Instant::now();
        for _ in 0..$total {
            std::hint::black_box($e);
        }
        start.elapsed()
    }};
}

macro_rules! compare {
    ($title:expr, $ret:expr, $total:expr, $eI:expr, $eM:expr) => {{
        $ret.push((
            $title,
            measure!($total, { $eI(std::hint::black_box(&mut IntMap::with_capacity(CAP))) }),
            measure!($total, { $eM(std::hint::black_box(&mut Map::with_capacity_none(CAP))) }),
        ));
    }};
}

#[allow(clippy::too_many_lines)]
fn benchmark(total: usize) -> Vec<(&'static str, Duration, Duration)> {
    let mut ret = vec![];
    compare!(
        "i ∈ 0..CAP {M.insert(i, &42); s ∈ M.values() {sum += s}}",
        ret,
        total,
        |mi: &mut FastIntMap| {
            let mut sum = 0;
            for i in 0..CAP as u64 {
                mi.insert(i, &42);
                for s in mi.values() {
                    sum += *s;
                }
            }
            sum
        },
        |m: &mut Map<_>| {
            let mut sum = 0;
            for i in 0..CAP {
                unsafe { m.insert_unchecked(i, &42) };
                for s in m.values() {
                    sum += *s;
                }
            }
            sum
        }
    );
    compare!(
        "i ∈ 0..CAP {M.insert(i, &\"大家好\"); s ∈ M.values() {sum += s.len()}}",
        ret,
        total,
        |mi: &mut FastIntMapStr| {
            let mut sum = 0;
            for i in 0..CAP as u64 {
                mi.insert(i, "大家好");
                for s in mi.values() {
                    sum += s.len();
                }
            }
            sum
        },
        |m: &mut Map<_>| {
            let mut sum = 0;
            for i in 0..CAP {
                unsafe { m.insert_unchecked(i, "大家好") };
                for s in m.values() {
                    sum += s.len();
                }
            }
            sum
        }
    );
    compare!(
        "i ∈ 0..CAP {M.insert(i, &42); s ∈ M.keys() {sum += s}}",
        ret,
        total,
        |mi: &mut FastIntMap| {
            let mut sum = 0;
            for i in 0..CAP as u64 {
                mi.insert(i, &42);
                for k in mi.keys() {
                    sum += k;
                }
            }
            sum
        },
        |m: &mut Map<_>| {
            let mut sum = 0;
            for i in 0..CAP {
                unsafe { m.insert_unchecked(i, &42) };
                for k in m.keys() {
                    sum += k;
                }
            }
            sum
        }
    );
    compare!(
        "i ∈ 0..CAP {M.insert(i, &42)}; i ∈ CAP-1..0 {M.remove(&i)}",
        ret,
        total,
        |mi: &mut FastIntMap| {
            for i in 0..CAP as u64 {
                mi.insert(i, &42);
            }
            for i in 0..CAP as u64 {
                mi.remove(i);
            }
        },
        |m: &mut Map<_>| {
            for i in 0..CAP {
                unsafe { m.insert_unchecked(i, &42) };
            }
            for i in 0..CAP {
                unsafe { m.remove_unchecked(i) };
            }
        }
    );
    compare!(
        "i ∈ 0..CAP {M.insert(i, &42)}; M.clear(); M.len();",
        ret,
        total,
        |mi: &mut FastIntMap| {
            for i in 0..CAP as u64 {
                mi.insert(i, &42);
            }
            mi.clear();
            mi.len()
        },
        |m: &mut Map<_>| {
            for i in 0..CAP {
                unsafe { m.insert_unchecked(i, &42) };
            }
            m.clear();
            m.len()
        }
    );
    compare!(
        "i ∈ 0..CAP {M.insert(i, &\"Hello, world!\")}",
        ret,
        total,
        |mi: &mut FastIntMapStr| {
            for i in 0..CAP as u64 {
                mi.insert(i, "Hello, world!");
            }
        },
        |m: &mut Map<_>| {
            for i in 0..CAP {
                unsafe { m.insert_unchecked(i, "Hello, world!") };
            }
        }
    );
    ret
}

/// Run it from command line:
///
/// ```text
/// $ cargo test --release benchmark_and_print -- --nocapture
/// ```
#[test]
pub fn benchmark_and_print() {
    let times = benchmark(1000);
    for (m, di, dm) in times {
        println!("{m} -> {:.2}x", di.as_nanos() as f64 / dm.as_nanos() as f64);
        assert!(di.as_nanos() > 0);
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let times = benchmark(args.get(1).unwrap().parse::<usize>().unwrap());
    let mut lines = vec![];
    for (m, di, dm) in times {
        lines.push(format!("{m}\t{}\t{}", di.as_nanos(), dm.as_nanos()));
    }
    lines.sort();
    for t in lines {
        println!("{t}");
    }
}
