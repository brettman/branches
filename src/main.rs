fn main() {
    let condition = true;

    // if is an *expression* so can be used as an assignment
    let number = if condition { 5 } else {6};
    println!("The value of the number is {}", number);
}
