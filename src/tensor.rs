//! This is the main tensor module in which we have all the actual implementations done.

use crate::numeric::Numeric;

/// The base structure to hold the tensor elements.
#[derive(Debug, PartialEq)]
pub struct Tensor<T: Numeric> {
    shape: Vec<u32>,
    data: Vec<T>,
}

impl<T: Numeric> Tensor<T> {
    pub fn get_shape(&self) -> Vec<u32> {
        self.shape.clone()
    }

    pub fn get_data(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn new(shape: Vec<u32>, data: Vec<T>) -> Self {
        Tensor { shape, data }
    }

    pub fn scale(self, scalar: T) -> Self {
        let mut new_data = Vec::<T>::new();
        
        for i in self.data {
            new_data.push(i * scalar);
        }

        Self {
            shape: self.shape.clone(),
            data: new_data,
        }
    }

    pub fn t(&self) -> Self {
        if self.shape.len() > 2 {
            panic!("Only 2D tensors can be transposed.");
        }

        if self.shape.len() == 1 {
            return Self {
                shape: self.shape.clone(),
                data: self.data.clone(),
            };
        }

        let new_shape = vec![self.shape[1], self.shape[0]];
        let mut new_data = vec![T::zero(); self.data.len()];

        let (rows, cols) = (self.shape[0] as usize, self.shape[1] as usize);

        for i in 0..rows {
            for j in 0..cols {
                new_data[j * rows + i] = self.data[i * cols + j];
            }
        }

        Self {
            shape: new_shape,
            data: new_data,
        }
    }

    pub fn mul(&self, rhs: &Self) -> Result<Tensor<T>, String> {
        if self.shape[1] != rhs.shape[0] {
            let s = format!(
                "ShapeMismatch:The dimensions of two matrices are not compatible for multiplication- {:?} {:?}",
                self.shape, rhs.shape
            );
            return Err(s);
        }

        let rows = self.shape[0] as usize;
        let cols = rhs.shape[1] as usize;
        let common_dim = self.shape[1] as usize;

        let mut data = vec![T::zero(); rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                for k in 0..common_dim {
                    let val = self.data[i * common_dim + k] * rhs.data[k * cols + j];
                    data[i * cols + j] = data[i * cols + j] + val;
                }
            }
        }

        Ok(Tensor {
            shape: vec![rows as u32, cols as u32],
            data,
        })
    }

    pub fn add(&self, rhs: &Self) -> Result<Tensor<T>, String> {
        let mut result = Vec::with_capacity(self.data.len());

        if self.data.len() != rhs.data.len() {
            return Err(String::from("Two matrices data are not of same dimensions"));
        }

        for i in 0..self.data.len() {
            result.push(self.data[i] + rhs.data[i]);
        }

        Ok(Self {
            shape: self.shape.clone(),
            data: result,
        })
    }

   pub fn sub(&self, rhs: &Self) -> Result<Tensor<T>, String> {
        let mut result = Vec::with_capacity(self.data.len());

        if self.data.len() != rhs.data.len() {
            return Err(String::from("Two matrices data are not of same dimensions"));
        }

        for i in 0..self.data.len() {
            result.push(self.data[i] - rhs.data[i]);
        }

        Ok(Self {
            shape: self.shape.clone(),
            data: result,
        })
    }
}

#[cfg(test)]
#[test]
fn test_new() {
    let t = Tensor::new(vec![2], vec![5, 10]);

    assert_eq!(t.shape, vec![2]);
    assert_eq!(t.data, vec![5, 10]);
}
