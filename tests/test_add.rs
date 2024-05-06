use ml::matrix::*;

#[test]
fn test_add_i32() {
    let m1 = Matrix::new(vec![1, 2]); // Matrix::<i32>::new(vec!(1,2))
    let m2 = Matrix::new(vec![3, 4]);

    let m3 = m1 + m2; // add(m1, m2) ---> m1.add(m2); ---> m1 + m2;

    assert_eq!(Matrix::new(vec!(4, 6)), m3);
}

#[test]
fn test_add_u32() {
    let m1 = Matrix::new(vec![1u32, 2u32]); // Matrix::<i32>::new(vec!(1,2))
    let m2 = Matrix::new(vec![3u32, 4u32]);

    let m3 = m1 + m2; // add(m1, m2) ---> m1.add(m2); ---> m1 + m2;

    assert_eq!(Matrix::new(vec!(4u32, 6u32)), m3);
}
