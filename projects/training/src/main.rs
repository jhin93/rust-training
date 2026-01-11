fn main() {
    let mut test = 5;
    println!("test: {}", test);
    test = 6;
    println!("test: {}", test);
    another_function(2, "streednf");
}

fn another_function(x: i32, string: &str) -> i32 {
    println!("number: {}", x);
    println!("string: {}", string);
    return x;
}