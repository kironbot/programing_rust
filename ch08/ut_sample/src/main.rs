fn main() {
    println!("Hello, world!");
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[should_panic]
fn must_be_panic() {
    assert!(false);
}