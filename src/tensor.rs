use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Tensor<T: Add + Copy> {
    data: Vec<T>,
}

// Matrix::new(v);
impl<T: Add + Copy> Tensor<T> {
    pub fn new(d: Vec<T>) -> Self {
        Tensor { data: d }
    }
}

pub mod addition;

mod multiplication;

#[cfg(test)]
#[test]
fn test_new() {
    let t = Tensor::new(vec![5, 10]);

    assert_eq!(t.data, vec![5, 10]);
}
