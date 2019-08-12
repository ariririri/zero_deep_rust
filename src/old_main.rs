use ndarray::arr2;
use ndarray::prelude::*;

//fn main() {
//    let a = arr2(&[[1.0, 2.0, 3.0],
//                   [4.0, 5.0, 6.0]]);
//    let b = arr2(&[[1.0, 2.1, 3.1],
//                   [4.4, 5.5, 6.1]]);
//    let mut data = array![[-1., -2., -3.], [1., -3., 5.], [2., 2., 2.]];<Paste>
//    println!("{:?}", a);
//    println!("{:?}", b);
//    println!("{:?}", a + b);
//    println!("{:?}", data);
//    println!("Hello, world!");
//}

use ndarray::prelude::*;

fn std1d(a: ArrayView1<'_, f64>) -> f64 {
    let n = a.len() as f64;
    if n == 0. {
        return 0.;
    }
    let mean = a.sum() / n;
    (a.fold(0., |acc, &x| acc + (x - mean).powi(2)) / n).sqrt()
}

fn std(a: &Array2<f64>, axis: Axis) -> Array1<f64> {
    a.map_axis(axis, std1d)
}

fn main() {
    // "recreating the following"
    // counts -= np.mean(counts, axis=0)
    // counts /= np.std(counts, axis=0)

    let mut data = array![[-1., -2., -3.], [1., -3., 5.], [2., 2., 2.]];

    println!("{:8.4}", data);
    println!("{:8.4} (Mean axis=0)", data.mean_axis(Axis(0)).unwrap());

    data -= &data.mean_axis(Axis(0)).unwrap();
    println!("{:8.4}", data);

    data /= &std(&data, Axis(0));
    println!("{:8.4}", data);
}
