use ml::tensor::*;

#[test]
fn test_add_i32() {
    let m1: Tensor<i32> = Tensor::new(vec![2], vec![1, 2]); // Matrix::<i32>::new(vec!(1,2))
    let m2 = Tensor::new(vec![2], vec![3, 4]);

    let m3: Result<Tensor<i32>, String> = m1 + m2; // add(m1, m2) ---> m1.add(m2); ---> m1 + m2;

    let m3 = m3.unwrap();

    assert_eq!(Tensor::new(vec![2], vec!(4, 6)), m3);
}

#[test]
fn test_add_u32() {
    let m1 = Tensor::new(vec![2], vec![1u32, 2u32]);
    let m2 = Tensor::new(vec![2], vec![3u32, 4u32]);

    let m3 = m1 + m2;

    let m3 = m3.unwrap();

    assert_eq!(Tensor::new(vec![2], vec!(4u32, 6u32)), m3);
}

#[test]
#[should_panic]
fn test_add_panics() {
    let m1 = Tensor::new(vec![2], vec![1, 2, 5]);
    let m2 = Tensor::new(vec![2], vec![3, 4]);

    let m3 = m1 + m2;

    let m3 = m3.unwrap();

    assert_eq!(Tensor::new(vec![2], vec!(4, 6)), m3);
}

#[test]
fn test_sub() {
    let m1 = Tensor::new(vec![2], vec![1, 2]);
    let m2 = Tensor::new(vec![2], vec![3, 4]);

    let m3 = m1 - m2;

    let m3 = m3.unwrap();

    assert_eq!(Tensor::new(vec![2], vec!(-2, -2)), m3);
}

#[test]
fn test_scale() {
    let m1 = Tensor::new(vec![2], vec![1, 2]);

    let m3 = m1.scale(5);

    assert_eq!(Tensor::new(vec![2], vec!(5, 10)), m3);
}

#[test]
fn test_mul() {
    let m1 = Tensor::new(vec![1, 2], vec![1, 2]);
    let m2 = Tensor::new(vec![2, 1], vec![3, 4]);

    let m3 = m1 * m2;

    let m3 = m3.unwrap();

    assert_eq!(Tensor::new(vec![1, 1], vec!(11)), m3);
}

#[test]
fn test_transpose() {
    let m1 = Tensor::new(vec![1, 2], vec![1, 2]);

    let m3 = m1.t();

    assert_eq!(Tensor::new(vec![2, 1], vec!(1, 2)), m3);
}
