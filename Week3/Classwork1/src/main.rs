fn main() {
    let s: String = String :: from("CSC 308"); // s comes into scope

    take_ownership(name:s); // s's value moves into the function ... 
    // and so is no longer valid here

    let value_1: i32 = 10; // value_1 comes into scope
    let value_2: i32 = 20; // value_2 comes into scope

    add(a: value_1, b: value_2); // Since i32 implement the Copy trait,
    // value_1 and value_2 are not moved into the function,
    // instead, they are copied, so we can still use them afterward.

    println! ("{value_1}, {value_2}"); // This works fine because value_1 and value_2 are still valid here
} // main function ends here

fn take_ownership(name:String) {
// name comes into scope
    println! ("Hello {}", name); // name is valid until the end of this function
} // name goes out of scope here and `drop' is called. The backing
// memory is freed.

fn add(a: i32, b: i32) {
    // a and b come into scope
    print! ("{}", a + b); // a and b are valid until the end of this function
} // a and b go out of scope here. Since they implement the Copy trait,
// nothing special happens.