use super::*;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result_vector = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            result_vector.push(self.data[i] + rhs.data[i]);
        }

        Self {
            data: result_vector,
        }
    }
}
