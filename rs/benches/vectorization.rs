#![feature(test)]
extern crate test;
extern crate rand;

extern crate nalgebra as na;

use na::{Dynamic, Matrix, MatrixMN, MatrixVec};

const DIMENSION_LENGTH: usize = 1_000_000;

/// Generate a one-dimensional matrix to use in tests.
fn new_test_vector() -> Matrix<f64, Dynamic, Dynamic, MatrixVec<f64, Dynamic, Dynamic>>
{
    MatrixMN::from_fn_generic(Dynamic::new(1), Dynamic::new(DIMENSION_LENGTH), rand_fn)
}

/// Generate a new float in the range `0.0...1.0`.
fn rand_fn(_: usize, _: usize) -> f64 { 
    rand::random::<f64>()
}

#[bench]
fn vectorization(b: &mut test::Bencher) {
    let vec1 = new_test_vector();
    let vec2 = new_test_vector();

    b.iter(|| {
        let _ret = vec1.dot(&vec2) + (1 as f64);
    })
}

#[bench]
fn without_vectorization(b: &mut test::Bencher) {
    let vec1 = new_test_vector();
    let vec2 = new_test_vector();

    let fn_loop = || -> f64 {
        let mut c: f64 = 0.0;
        
        for n in 0..DIMENSION_LENGTH {
            c += vec1[n] * vec2[n];
        }

        c + (1 as f64)
    };

    b.iter(|| {
        let _ret = fn_loop();
    })
}