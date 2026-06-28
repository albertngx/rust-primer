// Demonstrates references, dereferencing, Copy vs non-Copy types, and &mut with Vec

fn main() {
    // --- &T: borrowing (read-only reference) ---
    let numbers = vec![10u64, 20, 30];

    // &numbers[1..] borrows a slice — numbers is not consumed
    // m is &u64 on each iteration (a reference, not the value)
    // *m dereferences to get the u64 — works because u64 is Copy
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = fold(d, *m);
    }
    println!("fold result: {}", d); // numbers still usable here
    println!("numbers intact: {:?}", numbers);

    // --- &mut T: mutable reference, modifies the original in place ---
    let mut words = vec![String::from("hello"), String::from("world")];

    for s in &mut words {
        s.push_str("!"); // s is &mut String — writes through the reference
    }
    println!("mutated: {:?}", words); // ["hello!", "world!"]

    // --- non-Copy type: can't move out of a borrowed reference ---
    // Uncommenting this would fail to compile:
    //   let owned = *s; // error: cannot move out of `*s`, behind shared reference

    // Solutions when you need an owned String from a borrow:
    let greeting = &words[0];
    let cloned: String = greeting.clone();   // explicit copy of heap data
    let borrowed: &str = greeting;           // borrow as &str — no copy at all
    println!("cloned: {}, borrowed: {}", cloned, borrowed);

    // --- consuming the Vec (iterate without &) ---
    let names = vec![String::from("alice"), String::from("bob")];
    for name in names {
        println!("owned: {}", name); // name is a String here, fully owned
    }
    // names is gone — moved into the loop
}

fn fold(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { fold(b, a % b) }
}
