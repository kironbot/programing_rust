fn main() {
    println!("Hello, world!");
}

struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap {pixels, size}
}

#[test]
fn test_grayscale_map() {
    let width = 1024;
    let height = 576;
    let image = new_map((width, height), vec![0; width * height]);
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

/// Two possible alternatives for what a `Broom` could be working on.
#[derive(Copy, Clone)]
enum BroomIntent {FetchWater, DumpWater}

// Receive he input Broom by value, taking ownership.
fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {height: b.height/2, .. b};
    let mut broom2 = Broom {name: broom1.name.clone(), .. broom1};

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

#[test]
fn test_broom() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);
}

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}

#[test]
fn test_queue() {
    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());
}