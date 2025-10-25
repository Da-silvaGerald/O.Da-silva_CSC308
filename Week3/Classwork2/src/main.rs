fn main () {
    let name: String = String :: from("Chike"); // name comes into scope

    let returned_name: String = take_ownership_and_give_back(name); // name is moved into the function
    // and then returned back to the calling function

    println! ("Returned name is: {returned_name}"); // returned_name is valid here
} // main function ends here

fn take_ownership_and_give_back(name: String) -> String {
    // name comes into scope
    name // name is returned and moves out to the calling function
}