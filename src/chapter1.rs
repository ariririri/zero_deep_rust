extern crate ndarray;
extern crate image;
extern crate num_complex;

use ndarray::{arr2, arr3, s};
use ndarray::Array;
use std::ops::{Add, Sub, Mul};

pub fn chapter1() {
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    println!("{:?}", a);

    get_sample();
    loop_sample();
    zip_sample();

    let a = arr2(&[[1., 2.],
                [3., 4.]]);

    println!("slice {:?}", a.slice(s![.., 0..1]));
    matrix_sample();

    view();
}

fn get_sample(){
    let a = arr2(&[[1., 2.],
                   [3., 4.]]);
    
    assert!(
        a.get([0, 1]) == Some(&2.) &&
        a.get((0, 2)) == None &&
        a[(0, 1)] == 2. &&
        a[[0, 1]] == 2.
    );

}

fn loop_sample(){
    let mut a = Array::zeros((10, 10));
    for mut row in a.genrows_mut() {
        println!("{:}", row);
        row.fill(1.);
        println!("{:}", row);
    }
    println!("{:}", a);
    let a = arr3(&[[[ 50,  1,  2],
                [ 3,  4,  5]],
               [[ 6,  7,  8],
                [ 9, 10, 11]]]);
    println!("{:?}", a.iter().next().unwrap());

}

fn zip_sample() {
    use ndarray::Zip;
     // 1. Loop over the rows of a 2D array
    let mut a = Array::zeros((10, 10));
    for mut row in a.genrows_mut() {
        row.fill(1.);
    }

    let mut b = Array::zeros(a.rows());
    println!("a is {:?}", a.rows());
    
    Zip::from(a.genrows())
        .and(&mut b)
        .apply(|a_row, b_elt| {
            *b_elt = a_row[a.cols() - 1] + a_row[0];
        });
    println!("b {:?}", b);

    let mut c = Array::zeros(10);
    Zip::from(&mut c)
        .and(&b)
        .apply(|c_elt, &b_elt| {
            println!("*c {:?}", *c_elt);
            println!("c {:?}", c_elt);
            *c_elt = b_elt + 1.;
        });
    println!("c {:?}", c);

    use ndarray::Array2;
    type M = Array2<f64>;
    let mut a = M::zeros((12, 8));
    let b = M::from_elem(a.dim(), 1.);
    let c = M::from_elem(a.dim(), 2.);
    let d = M::from_elem(a.dim(), 3.);


    Zip::from(&mut a)
        .and(&b)
        .and(&c)
        .and(&d)
        .apply(|w, &x, &y, &z| {
            *w += x + y * z;
        });
    println!("{:?}", a);

}

fn matrix_sample() {
    let a = arr2(&[[1., 2.],
                [3., 4.]]);
    let b = arr2(&[[5., 6.],
                [3., 4.]]);
    let c = a.add(&b);
    let d = c.sub(&b);
    let e = d.dot(&b);
    let f = e.mul(&b); // 要素積
    println!("f {}", f);

}


fn view() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).0;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
