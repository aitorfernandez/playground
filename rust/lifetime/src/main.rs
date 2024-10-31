fn main() {
    let i: i32 = 3; // lifetime starts

    {
        let borrow = &i; // lifetime stars
        println!("borrow 1: {borrow}");

        let named_borrowed = NamedBorrowed {
            x: &borrow,
            y: &borrow,
        };
        println!("name borrowed: {} {}", named_borrowed.x, named_borrowed.y);
    } // borrow lifetime ends and named_borrowed lifetime ends

    {
        let borrow = &i; // lifetime stars
        println!("borrow 2: {borrow}");
    } // lifetime ends

    {
        let s1 = String::from("Hello");
        let s2 = String::from("World");
        let res = longest(&s1, &s2);
        println!("the longer string is: {res}");
    }
} // lifetime ends

// If we didn't specify the lifetimes, the Rust compiler wouldn't be able to guarantee
// that the returned reference is valid as long as the inputs.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Live as long as the references lives
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}
