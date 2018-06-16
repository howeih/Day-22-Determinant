#[macro_use(array)]
extern crate ndarray;
use ndarray::Array2;
use ndarray::Dim;

fn determint(x: Array2<f64>) -> f64 {
    assert!(x.is_square());
    let mut det: f64 = 0.;
    let x_dim = x.raw_dim();
    if x_dim == Dim([2, 2]) {
        return *x.get((0, 0)).unwrap() * *x.get((1, 1)).unwrap()
            - *x.get((0, 1)).unwrap() * *x.get((1, 0)).unwrap();
    }
    for (col, pivot) in x.row(0).iter().enumerate() {
        let mut ary = Array2::<f64>::zeros((x_dim[0] as usize - 1, x_dim[1] as usize - 1));
        let mut ary_r = 0;
        for x_r in 1..x_dim[0]{
            let mut ary_c = 0;
            for x_c in 0..x_dim[1]{
                if x_c == col{
                    continue;
                }
                ary[[ary_r, ary_c]] = x[[x_r, x_c]];
                ary_c += 1;
            }
            ary_r+=1;
        }
        if col % 2 == 1{
            det += -pivot* determint(ary);
        }
        else{
            det += pivot* determint(ary);
        }
    }
    det
}

fn main() {
    let x = array![[4., 9., 2.], [3., 5., 7.], [8., 1., 6.]];
    let det = determint(x);
    assert_eq!(det, 360.);
}
