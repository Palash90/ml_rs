#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub data: Vec<i32>,
}

fn create() -> Matrix {
    Matrix { data: vec![1, 2] }
}

pub mod addition;

mod multiplication;
