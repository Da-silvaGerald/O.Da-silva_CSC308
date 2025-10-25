fn main() {
    let name:String = String::from("Chike"); // name comes into scope

    let (returned_name:String, len:usize) = calculate_length(name);

    println! ("The length of '{returned_name}' is {len}.");
} // main function ends here

fn calculate_length(s:String) -> (String, usize) {
    let length:usize = s.len(); // len() returns the length of a String

    (s, length) // return the String and its length as a tuple
}