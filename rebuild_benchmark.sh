#!/bin/bash

# SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
# SPDX-License-Identifier: MIT

set -x
set -e

rm -rf src/bin
mkdir -p src/bin
cp tests/benchmark.rs src/bin/benchmark.rs

sed -E -i 's/\[dev-dependencies\]//g' Cargo.toml

caps="4 16 256 4096"
cycles=10000
first=$(echo "${caps}" | cut -f1 -d' ')

rm -rf target/benchmark
mkdir -p target/benchmark
SECONDS=0
for cap in ${caps}; do
    sed -E -i "s/CAP: usize = [0-9]+/CAP: usize = ${cap}/g" src/bin/benchmark.rs
    cargo build --release
    ./target/release/benchmark "${cycles}" > "target/benchmark/${cap}.out"
done

{
    echo -n '| |'
    for cap in ${caps}; do
        echo -n " ${cap} |"
    done
    echo ''
    echo -n '| --- |'
    for cap in ${caps}; do
        echo -n " --: |"
    done
    echo ''
    while read -r script; do
        echo -n "| \`${script}\` |"
        for cap in ${caps}; do
            dv=$(grep "${script}" "target/benchmark/${cap}.out" | cut -f 2)
            dm=$(grep "${script}" "target/benchmark/${cap}.out" | cut -f 3)
            perl -e "printf(\"%.02f\", ${dv} / ${dm});"
            echo -n ' |'
        done
        echo ''
    done < <(cut -f 1 "target/benchmark/${first}.out")
    echo ''
    echo "The experiment was performed on $(date +%d-%m-%Y)."
    echo " There were ${cycles} repetition cycles."
    echo " The entire benchmark took ${SECONDS}s."
} > target/benchmark/table.md

perl -e '
    my $readme;
    my $file = "README.md";
    open(my $r, "<", $file);
    { local $/; $readme = <$r>; }
    close($r);
    my $sep = "<!-- benchmark -->";
    my @p = split(/\Q$sep\E/, $readme);
    my $table = "target/benchmark/table.md";
    open(my $t, "<", $table);
    { local $/; $table = <$t>; }
    close($t);
    $p[1] = "\n" . $table . "\n";
    my $new = join($sep, @p);
    open(my $w, ">", $file);
    print $w join($sep, @p);
    close($w);
'

git restore Cargo.toml
rm -rf src/bin
cat target/benchmark/table.md
