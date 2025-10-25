fn main (){
    let name: String = String :: from("Chike"); // name comes into scope

    let len: usize = calculate_length_ref(&name);

    println! ("The length of '{name}' is {len}.");
} //main function ends here

fn calculate_length_ref(s: &String) -> usize {
    s.len() // return the length of the String
} // s goes out of scope here. Since it does not have ownership of what
// it refers to, nothing happens.