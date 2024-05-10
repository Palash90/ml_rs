use ml::matrix::*;

#[test]
fn test_add_i32() {
    let m1: Matrix<i32> = Matrix::new(vec![2, 2], vec![1, 2, 5, 6]).unwrap(); // Matrix::<i32>::new(vec!(1,2))
    let m2 = Matrix::new(vec![2, 2], vec![3, 4, 7, 8]).unwrap();

    let m3: Result<Matrix<i32>, String> = m1 + m2; // add(m1, m2) ---> m1.add(m2); ---> m1 + m2;

    let m3 = m3.unwrap();

    assert_eq!(Matrix::new(vec![2, 2], vec!(4, 6, 12, 14)).unwrap(), m3);
}
