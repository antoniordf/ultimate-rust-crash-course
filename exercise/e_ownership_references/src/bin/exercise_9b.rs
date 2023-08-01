// This code has no errors!
// Borrow a mutable object as immutable
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(_s: &String) {}
