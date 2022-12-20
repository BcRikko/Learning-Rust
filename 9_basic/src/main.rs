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


    // 四則演算
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    let diff = 95.5 - 4.3;
    println!("Diff: {}", diff);

    let prod = 4 * 30;
    println!("Prod: {}", prod);

    let quot = 56.7 / 32.2;
    println!("Quot: {}", quot);

    let floor = 2 / 3;  // i32なので `0` になる
    println!("Floor: {}", floor);
    
    let floor2 = 2.0 / 3.0; // f64なので `0.666...` になる
    println!("Floor2: {}", floor2);
    
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ';
    let emoji = '🤔';
    // 👍🏻 のように色指定が入っていると「character literal may only contain one codepoint」になる
    println!("Char: {}, {}, {}", c, z, emoji);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple1: {0} {1} {2}", x, y, z);
    println!("Tuple2: {0} {1} {2}", tup.0, tup.1, tup.2);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {}", arr[0]);
}
