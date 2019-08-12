#![allow(
    clippy::many_single_char_names,
    clippy::deref_addrof,
    clippy::unreadable_literal,
    clippy::many_single_char_names
)]

extern crate ndarray;
extern crate image;
extern crate num_complex;

use ndarray::arr1;
//use ndarray::Array;
//use std::ops::{Add, Sub, Mul};

mod chapter1;
mod chapter2;

fn main(){
    let mode = 2;
    if mode == 1 {
        c1();
    } else if mode == 2 {
        c2();
    }
}

fn c1(){
    chapter1::chapter1();
}

fn c2(){
    let x = arr1(&[1., 0.]);
    chapter2::all(&x);
    let x = arr1(&[0., 0.]);
    chapter2::all(&x);
    let x = arr1(&[0., 1.]);
    chapter2::all(&x);
    let x = arr1(&[1., 1.]);
    chapter2::all(&x);
}

