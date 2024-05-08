use super::*;
use std::ops::Add;

impl<T: Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Result<Matrix<T>, String>;

    fn add(self, rhs: Self) -> Result<Matrix<T>, String> {
        let mut result_vector = Vec::with_capacity(self.data.len());

        if self.data.len() != rhs.data.len() {
            return Err(String::from("Two matrices data are not of same dimensions"));
        }

        for i in 0..self.data.len() {
            result_vector.push(self.data[i] + rhs.data[i]);
        }

        Ok(Self {
            data: result_vector,
        })
    }
}
