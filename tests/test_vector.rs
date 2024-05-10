use ml::vector::*;

#[test]
fn test_add_i32() {
    let m1: Vector<i32> = Vector::new(vec![4], vec![1, 2, 5, 6]).unwrap(); // Matrix::<i32>::new(vec!(1,2))
    let m2 = Vector::new(vec![4], vec![3, 4, 7, 8]).unwrap();

    let m3: Result<Vector<i32>, String> = m1 + m2;

    let m3 = m3.unwrap();

    assert_eq!(Vector::new(vec![4], vec!(4, 6, 12, 14)).unwrap(), m3);
}
