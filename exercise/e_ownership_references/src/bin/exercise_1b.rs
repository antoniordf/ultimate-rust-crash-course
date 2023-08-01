fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

// Result: the memory address of x is 0x7fff666304ac

// In the code provided, p is a reference to x, not a copy of x. When you create p with let p = &x;, you're saying "let p be a reference to x".
// This means that p doesn't hold the value 5, it holds the memory address where x (and therefore the value 5) is stored.

// When you print p using the {:p} format specifier, you're asking Rust to print the memory address that p points to, not the value at that address.
// This is why you see a memory address printed out, not 5.

// If you wanted to print the value that p points to (i.e., 5), you would need to dereference p using the * operator, like so:
// println!("the value of x is {}", *p);
// This would print 5, because *p dereferences the pointer p to get the value it points to.
