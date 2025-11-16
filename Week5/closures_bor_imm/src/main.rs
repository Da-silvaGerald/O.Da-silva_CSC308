fn main() {
    let list: Vec<i32> = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows: impl Fn() = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
