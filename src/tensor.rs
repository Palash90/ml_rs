//! This is the main tensor module in which we have all the actual implementations done.

use crate::numeric::Numeric;

/// The base structure to hold the tensor elements.
#[derive(Debug, PartialEq)]
pub struct Tensor<T: Numeric> {
    shape: Vec<u32>,
    data: Vec<T>,
}

impl<T: Numeric> Tensor<T> {
    pub fn new(shape: Vec<u32>, data: Vec<T>) -> Self {
        Tensor { shape, data }
    }
}

pub mod addition;

mod multiplication;

#[cfg(test)]
#[test]
fn test_new() {
    let t = Tensor::new(vec![2], vec![5, 10]);

    assert_eq!(t.shape, vec![2]);
    assert_eq!(t.data, vec![5, 10]);
}
