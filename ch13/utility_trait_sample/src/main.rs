fn main() {
    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec!["cloud collector".to_string(), "king of the gods".to_string()]
    };
    println!("before assignment");
    a = Appellation {name: "Hera".to_string(), nicknames: vec![]};
    println!("at end block");
}

struct Appellation {
    name: String,
    nicknames: Vec<String>
}

impl Drop for Appellation {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            println!("(AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}