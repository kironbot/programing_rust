fn main() {
    println!("hello world!");
}

#[test]
fn test_borrow() {
    // 単純な変数の参照
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }

    // vecの参照
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s {s = r;}
        }
        s
    }
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0);

    // 構造体の参照
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S{x: &x, y: &y};
            r = s.x;
        }
    }
    assert_eq!(*r, 10);

}

#[test]
fn test_point() {
    struct Point {x: i32, y: i32}
    let point = Point {x: 1000, y: 729};
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr : &&&Point = &rr;
    assert_eq!(rrr.y, 729);
}

#[test]
fn test_table_show() {
    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;

    fn show(table: &Table) {
        for (artist, works) in table {
            println!("works by {}:", artist);
            for work in works {
                println!(" {}", work);
            }
        }
    }
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}