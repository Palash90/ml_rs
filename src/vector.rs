use crate::numeric::Numeric;
use crate::tensor::Tensor;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Vector<T: Numeric> {
    tensor: Tensor<T>,
}

impl<T: Numeric> Vector<T> {
    pub fn new(shape: Vec<u32>, data: Vec<T>) -> Result<Self, String> {
        if shape.len() != 1 {
            return Err(format!(
                "VectorShapeError:Vector must have ony one dimension. Provided {}",
                shape.len()
            ));
        }

        Ok(Self {
            tensor: Tensor::new(shape, data),
        })
    }
}

impl<T: Numeric> Add for Vector<T> {
    type Output = Result<Vector<T>, String>;
    fn add(self, rhs: Self) -> Result<Vector<T>, String> {
        let tensor = self.tensor + rhs.tensor;
        Ok(Self { tensor: tensor? })
    }
}

#[cfg(test)]
#[test]
#[should_panic(expected= "VectorShapeError")]
fn test_new() {

    match  Vector::new(vec![2, 3], vec![5, 10]) {
        Ok(_) => {},
        Err(s) => panic!("{}", s)
    }
}
