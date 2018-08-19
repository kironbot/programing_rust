fn main() {
    print_padovan();
    print_person();
    print_label();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here

fn print_person() {
    struct Person { name: String, birth: i32}

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525});
    composers.push(Person { name: "Downland".to_string(), birth: 1563});
    composers.push(Person { name: "Lully".to_string(), birth: 1632});
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn print_label() {
    #[derive(Copy, Clone)]
    struct Label { number: u32 }

    fn print(l: Label) {println!("STAMP: {}", l.number); }

    let l = Label {number: 3};
    print(l);
    println!("My label number is: {}", l.number);
}
