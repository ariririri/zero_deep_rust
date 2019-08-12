extern crate ndarray;
extern crate image;
extern crate num_complex;

use ndarray::arr1;
use ndarray::Array1;

fn over(x: f64, theta: f64) -> f64 {
    return if x >= theta {
        1.0
    } else {
        0.0
    }
}

pub fn and(x: &Array1<f64>) -> f64{
    let weights = arr1(&[0.5, 0.5]);
    let theta = 0.7;
    println!("{:?}", weights);
    return over(x.dot(&weights), theta)
}

pub fn or(x: &Array1<f64>) -> f64{
    let weights = arr1(&[0.5, 0.5]);
    let theta = 0.3;
    println!("{:?}", weights);
    return over(x.dot(&weights), theta)
}

pub fn nand(x: &Array1<f64>) -> f64{
    let weights = arr1(&[-0.5, -0.5]);
    let theta = -0.7;
    println!("{:?}", weights);
    return over(x.dot(&weights), theta)
}

pub fn xor(x: &Array1<f64>) -> f64{
    let s1 = or(&x);
    let s2 = nand(&x);
    let s = arr1(&[s1, s2]);
    return and(&s);
}

pub fn all(x: &Array1<f64>) {
    println!("input {:8.4}", x);
    println!("and {:8.4}", and(&x));
    println!("or {:8.4}", or(&x));
    println!("nand {:8.4}", nand(&x));
    println!("xor {:8.4}", xor(&x));
}