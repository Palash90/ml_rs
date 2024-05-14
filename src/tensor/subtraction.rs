use super::*;
use std::ops::Sub;

use crate::numeric::Numeric;

impl<T: Numeric> Sub for Tensor<T> {
    type Output = Result<Tensor<T>, String>;

    fn sub(self, rhs: Self) -> Result<Tensor<T>, String> {
        let mut result = Vec::with_capacity(self.data.len());

        if self.data.len() != rhs.data.len() {
            return Err(String::from("Two matrices data are not of same dimensions"));
        }

        for i in 0..self.data.len() {
            result.push(self.data[i] - rhs.data[i]);
        }

        Ok(Self {
            shape: self.shape,
            data: result,
        })
    }
}
