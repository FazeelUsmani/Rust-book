fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // Passing references not the variable
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // references
    println!("{} {}!", g1, g2);
}

// References are non-owning pointers, because they do not own the data they point to.