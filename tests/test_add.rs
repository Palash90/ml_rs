use ml::matrix::*;

#[test]
fn test_add() {
    let m1 = Matrix { data: vec![1, 2] };
    let m2 = Matrix { data: vec![3, 4] };

    let m3 = m1 + m2; // add(m1, m2) ---> m1.add(m2); ---> m1 + m2;

    assert_eq!(Matrix { data: vec!(1, 2) }, m3);
}
