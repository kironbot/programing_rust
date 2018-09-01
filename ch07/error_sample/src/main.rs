fn main() {
    // Panic sample
    println!("share is {}", pirate_share(10, 0));
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}