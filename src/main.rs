fn main() {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index <5 {
        println!("the value is {}", a[index]);
        index+=1;
    }

    println!("Now let's do it with a for expression");

    for element in a {
        println!(" the value is {}", element);
    }
}
