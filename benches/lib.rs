#![feature(test)]

extern crate test;
use test::Bencher;
use test::black_box;

use zummi::{zummi, zummi_broke, zummi_naive};

#[bench]
fn zummi_bench(b: &mut Bencher) {
    b.iter(|| {
        let mix = zummi("stinkender fisch");
        black_box(mix);
    });
}

#[bench]
fn zummi_broke_bench(b: &mut Bencher) {
    b.iter(|| {
        let mix = zummi_broke("stinkender fisch");
        black_box(mix);
    });
}

#[bench]
fn zummi_naive_bench(b: &mut Bencher) {
    b.iter(|| {
        let mix = zummi_naive("stinkender fisch");
        black_box(mix);
    });
}

