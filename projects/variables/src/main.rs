fn main() {
    let mut x = 5; // variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant

    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    shawdoing();
    changeWithMut();
}

// variable(let, basically immutable, 'mut' can be used for assigning new value)
// constant(const, completely immutable, write with upper letters)

fn shawdoing() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// The value of x in the inner scope is: 12
// The value of x is: 6

fn changeWithMut() {
    let mut x = 5;

    x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}