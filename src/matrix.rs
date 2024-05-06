pub struct Matrix {
    pub data: Vec<i32>,
}

fn create() -> Matrix {
    Matrix { data: vec![1, 2] }
}

mod addition;

mod multiplication;
