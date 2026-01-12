fn main() {
    another_function(10);
}

fn another_function(x: i32) {
    if x < 5 {
        println!("x is less than 5: {}", x);
    } else if x == 5 {
        println!("x is equal to 5: {}", x);
    } else {
        println!("x is greater than 5: {}", x);
    }
}