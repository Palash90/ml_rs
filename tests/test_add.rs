use ml::matrix::*;

#[test]
fn test_add() {
    let m1 = Matrix::new(vec![1, 2]);
    let m2 = Matrix::new(vec![3, 4]);

    let m3 = m1 + m2; // add(m1, m2) ---> m1.add(m2); ---> m1 + m2;

    assert_eq!(Matrix::new(vec!(4, 6)), m3);
}
