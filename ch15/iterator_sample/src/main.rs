fn main() {
    fn triangle_1(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..n+1 {
            sum += i;
        }
        sum
    }

    fn triangle(n: i32) -> i32 {
        (1..n+1).fold(0, |sum, item| sum + item)
    }
    println!("triangle_1(10)= {}", triangle_1(10));
    println!("triangle(10)= {}", triangle(10));
    println!("\n");

    println!("There's");
    let v = vec!["antimony", "arsenic", "alumium", "selenium"];

    for element in &v {
        println!("{}", element);
    }
    println!("\n");

    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/kiron/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("kiron")));

    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebestraume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebestraume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);

    use std::iter::FromIterator;
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");

    let text = " ponies \n  giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
    let v: Vec<&str> = text.lines().map(str::trim).filter(|s| *s != "iguanas").collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    use std::str::FromStr;
    let text = "1\nfrond .25 289\n3.1415 estuaryn\n";
    for number in text.split_whitespace().filter_map(|w| f64::from_str(w).ok()) {
        println!("{:4.2}", number.sqrt());
    }
    println!("\n");

    use std::collections::HashMap;

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["San Paulo", "Brazilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
    println!("\n");

    let iter = (0..10).scan(0, |sum, item| {
        *sum += item;
        if *sum > 10 {
            None
        } else {
            Some(item * item)
        }
    });
    assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);

    struct Flaky(bool);

    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true; // Oh...
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);

    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);

    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    use std::iter::{once, repeat};
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes)
        .map(|tuple|
            match tuple {
                (i, ("", "")) => i.to_string(),
                (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
            });
    for line in fizz_buzz {
        println!("{}", line);
    }

    let v: Vec<i32> = vec![1, 2, 3];
    let it = v.iter();
    assert_eq!(it.count(), 3);
    let it = v.iter();
    assert_eq!(it.max(), Some(&3));

    let v = [1, 2, 3, 4, 5];
    assert_eq!(v.iter().fold(0, |n, _| n+1), 5);
    assert_eq!(v.iter().fold(0, |n, i| n+i), 15);
    assert_eq!(v.iter().fold(1, |n, i| n*i), 120);

    let mut it = 0..10;
    assert_eq!(it.nth(1), Some(1));
    assert_eq!(it.nth(0), Some(2));
}