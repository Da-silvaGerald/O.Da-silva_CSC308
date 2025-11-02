use std::io;

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert string to bytes for easy indexing
    let mut end = s.len();

    // Move backwards to skip any trailing spaces
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    let mut start = end;

    // Move backwards until we reach the start of the last word
    while start > 0 && !bytes[start - 1].is_ascii_whitespace() {
        start -= 1;
    }

    // Return a slice of the original string
    &s[start..end]
}


fn main() {
    println!("Input a sentence:");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");

    let last = last_word(&mut sentence);

    println!("The last word is: '{}'", last);
}

