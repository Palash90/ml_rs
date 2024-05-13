//! This is the matrix module, a thin wrapper over Tensor.

use crate::numeric::Numeric;
use crate::tensor::Tensor;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Matrix<T: Numeric> {
    tensor: Tensor<T>,
}

impl<T: Numeric> Matrix<T> {
    /// It creates a new Matrix object
    /// 
    /// # Example
    /// ```rust
    /// use ml::matrix::Matrix;
    /// 
    /// let m = Matrix::new(vec![1,1], vec![4]).unwrap();
    /// ```
    /// 
    /// 
    /// # Panic
    /// The user can expect some panic!
    /// 
    pub fn new(shape: Vec<u32>, data: Vec<T>) -> Result<Self, String> {
        if shape.len() != 2 {
            panic!("{}", format!(
                "MatrixShapeError:Matrix must have two dimensions. Provided {}",
                shape.len()
            ));
        }

        Ok(Self {
            tensor: Tensor::new(shape, data),
        })
    }
}

impl<T: Numeric> Add for Matrix<T> {
    type Output = Result<Matrix<T>, String>;
    fn add(self, rhs: Self) -> Result<Matrix<T>, String> {
        let tensor = self.tensor + rhs.tensor;
        Ok(Self { tensor: tensor? })
    }
}

#[cfg(test)]
#[test]
#[should_panic(expected = "MatrixShapeError")]
fn test_new() {
    match Matrix::new(vec![2], vec![5, 10]) {
        Ok(_) => {}
        Err(s) => panic!("{}", s),
    }
}
