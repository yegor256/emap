// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// In order to run this single test from the command line:
// $ cargo test --test benchmark -- --nocapture

use emap::Map;
use std::env;
use std::time::{Duration, Instant};

const CAP: usize = 10;

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
    ($title:expr, $ret:expr, $total:expr, $eV:expr, $eM:expr) => {{
        $ret.push((
            $title,
            measure!($total, { $eV(&mut Vec::with_capacity(CAP)) }),
            measure!($total, { $eM(&mut Map::with_capacity(CAP)) }),
        ));
    }};
}

fn benchmark(total: usize) -> Vec<(&'static str, Duration, Duration)> {
    let mut ret = vec![];
    compare!(
        "for _ in 0..CAP { M.push(&42); for s in M.into_values() { s > 0; }; }",
        ret,
        total,
        |v: &mut Vec<_>| {
            for _ in 0..CAP {
                v.push(&42);
                for s in v.into_iter() {
                    std::hint::black_box(*s > &0);
                }
            }
        },
        |v: &mut Map<_>| {
            for _ in 0..CAP {
                v.push(&42);
                for s in v.into_values() {
                    std::hint::black_box(*s > 0);
                }
            }
        }
    );
    compare!(
        "for _ in 0..CAP { M.push(&42); } for i in CAP-1..0 { M.remove(&i); }",
        ret,
        total,
        |v: &mut Vec<_>| {
            for _ in 0..CAP {
                v.push(&42);
            }
            for i in CAP - 1..0 {
                std::hint::black_box(v.remove(i));
            }
        },
        |v: &mut Map<_>| {
            for _ in 0..CAP {
                v.push(&42);
            }
            for i in CAP - 1..0 {
                std::hint::black_box(v.remove(&i));
            }
        }
    );
    compare!(
        "for _ in 0..CAP { M.push(&\"Hello, world!\"); }",
        ret,
        total,
        |v: &mut Vec<_>| {
            for _ in 0..CAP {
                std::hint::black_box(v.push(&"Hello, world!"));
            }
        },
        |v: &mut Map<_>| {
            for _ in 0..CAP {
                std::hint::black_box(v.push(&"Hello, world!"));
            }
        }
    );
    ret
}

#[test]
pub fn benchmark_and_print() {
    let times = benchmark(1000);
    for (m, dv, dm) in times {
        println!("{m} -> {:.2}x", dv.as_nanos() as f64 / dm.as_nanos() as f64);
        assert!(dv.cmp(&dm).is_gt());
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let times = benchmark(args.get(1).unwrap().parse::<usize>().unwrap());
    let mut lines = vec![];
    for (m, dv, dm) in times {
        lines.push(format!("{m}\t{}\t{}", dv.as_nanos(), dm.as_nanos()));
    }
    lines.sort();
    for t in lines {
        println!("{t}");
    }
}
