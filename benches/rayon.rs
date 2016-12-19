
#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use(s)]
extern crate ndarray;
use ndarray::prelude::*;

extern crate rayon;
use rayon::prelude::*;

const EXP_N: usize = 128;

#[bench]
fn map_exp_regular(bench: &mut Bencher)
{
    let mut a = Array2::<f64>::zeros((EXP_N, EXP_N));
    a.swap_axes(0, 1);
    bench.iter(|| {
        a.mapv_inplace(|x| x.exp());
    });
}

#[bench]
fn rayon_exp_regular(bench: &mut Bencher)
{
    let mut a = Array2::<f64>::zeros((EXP_N, EXP_N));
    a.swap_axes(0, 1);
    bench.iter(|| {
        a.view_mut().into_par_iter().for_each(|x| *x = x.exp());
    });
}

const FASTEXP: usize = 900;

#[inline]
fn fastexp(x: f64) -> f64 {
    let x = 1. + x/1024.;
    x.powi(1024)
}

#[bench]
fn map_fastexp_regular(bench: &mut Bencher)
{
    let mut a = Array2::<f64>::zeros((FASTEXP, FASTEXP));
    let mut a = a.slice_mut(s![.., ..-1]);
    bench.iter(|| {
        a.mapv_inplace(|x| fastexp(x))
    });
}

#[bench]
fn rayon_fastexp_regular(bench: &mut Bencher)
{
    let mut a = Array2::<f64>::zeros((FASTEXP, FASTEXP));
    let mut a = a.slice_mut(s![.., ..-1]);
    bench.iter(|| {
        a.view_mut().into_par_iter().for_each(|x| *x = fastexp(*x));
    });
}