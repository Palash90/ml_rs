#[derive(Debug, PartialEq)]
pub struct Matrix {
    data: Vec<i32>,
}

// Matrix::new(v);
impl Matrix {
    pub fn new(d: Vec<i32>) -> Matrix {
        Matrix { data: d }
    }
}

pub mod addition;

mod multiplication;

#[cfg(test)]
#[test]
fn test_new() {
    let t = Matrix::new(vec![5, 10]);

    assert_eq!(t.data, vec![5, 10]);
}
