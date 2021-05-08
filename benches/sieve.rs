#![feature(test)]
extern crate test;

use cpl_rust::math::eratosthenes::Eratosthenes;
use cpl_rust::math::linear_sieve::LinearSieve;
use test::Bencher;

const MAX: usize = 1_000_000;

#[bench]
fn bench_eratosthenes_sieve(b: &mut Bencher) {
    b.iter(|| Eratosthenes::new(MAX));
}

#[bench]
fn bench_linear_sieve(b: &mut Bencher) {
    b.iter(|| LinearSieve::new(MAX));
}
