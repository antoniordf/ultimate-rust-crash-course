fn main() {
    let mut s = String::from("hello, ");

    let _r1 = &mut s;
    let _r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    // r1.push_str("world");
}
