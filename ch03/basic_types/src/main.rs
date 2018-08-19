fn main() {
    println!("Hello, world!");
}

#[test]
fn test_integer() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(-4i32.abs(), -4);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    assert_eq!('*' as i32, 42);

    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    let mut v = vec![10, 20, 30, 40, 50];

    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
}