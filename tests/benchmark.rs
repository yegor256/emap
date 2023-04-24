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
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

macro_rules! measure {
    ($title:expr, $ret:expr, $total:expr, $e:expr) => {{
        let start = Instant::now();
        let mut sum = 0;
        for _ in 0..$total {
            sum += $e;
        }
        std::hint::black_box(sum);
        let e = start.elapsed();
        $ret.insert($title, e);
    }};
}

fn benchmark(total: usize) -> HashMap<&'static str, Duration> {
    let mut ret = HashMap::new();
    measure!("std::Vec", ret, total, {
        let mut sum = 0;
        let mut v = Vec::with_capacity(total);
        for i in 0..total {
            v.push(&"hello!");
            if !std::hint::black_box(v[i]).is_empty() {
                sum += 1;
            }
        }
        for v in v.into_iter() {
            if !v.is_empty() {
                sum += 1;
            }
        }
        std::hint::black_box(sum)
    });
    measure!("emap::Map", ret, total, {
        let mut sum = 0;
        let mut v = Map::with_capacity(total);
        for i in 0..total {
            v.insert(i, &"hello!");
            if !std::hint::black_box(v[i]).is_empty() {
                sum += 1;
            }
        }
        for v in v.into_values() {
            if !v.is_empty() {
                sum += 1;
            }
        }
        std::hint::black_box(sum)
    });
    ret
}

#[test]
pub fn benchmark_and_print() {
    let times = benchmark(1000);
    let ours = times.get("emap::Map").unwrap();
    for (m, d) in &times {
        println!(
            "{m} -> {:?} ({:.2}x)",
            d,
            d.as_nanos() as f64 / ours.as_nanos() as f64
        );
        if d == ours {
            continue;
        }
        assert!(d.cmp(ours).is_gt());
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let times = benchmark(args.get(1).unwrap().parse::<usize>().unwrap());
    let mut lines = vec![];
    for (m, d) in &times {
        lines.push(format!("{m}\t{}", d.as_nanos()));
    }
    lines.sort();
    for t in lines {
        println!("{t}");
    }
}
