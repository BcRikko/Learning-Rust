fn main() {
    println!("Hello, world!");

    // Immutable
    let x = 5;

    // Mutable
    let mut y = 5;
    println!("The value of x is: {}", y);
    y = 6;
    println!("The value of x is: {}", y);
    
    // Constants
    const X: u32 = 5;
    println!("The value of X is: {}", X);

    // const mut Y: u32 = 5;
    //       ^^^ cannot be mutable
    // Y = 6;
    // println!("The value of X is: {}", Y);

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        // > 12
    }

    println!("The value of x is: {}", x);
    // > 6
}
