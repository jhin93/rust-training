fn main() {
    let mut x = 5; // variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant

    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
}

// variable(let, basically immutable, 'mut' can be used for assigning new value)
// constant(const, completely immutable, write with upper letters)
