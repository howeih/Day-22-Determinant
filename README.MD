#Day 22: Determinant
<br>
In linear algebra, the determinant is a value that can be computed from the elements of a square matrix. The determinant of a matrix A is denoted det(A), det A, or |A|. Geometrically, it can be viewed as the scaling factor of the linear transformation described by the matrix.

```
fn main() {
    let x = array![[4., 9., 2.], [3., 5., 7.], [8., 1., 6.]];
    let det = determint(x);
    assert_eq!(det, 360.);
}

```
