use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Matrix<T: Add + Copy> {
    data: Vec<T>,
}

// Matrix::new(v);
impl<T: Add + Copy> Matrix<T> {
    pub fn new(d: Vec<T>) -> Self {
        Matrix { data: d }
    }
}

pub mod addition;

mod multiplication;

#[cfg(test)]
#[test]
fn test_new() {
    let t = Matrix::new(vec![5, 10]);

    assert_eq!(t.data, vec![5, 10]);
}
