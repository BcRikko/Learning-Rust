fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    if number != 0 {
        println!("number was something other than zero");
    }

    fizzbuzz(45);

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);
}

fn fizzbuzz(x: i32) {
    if x % 15 == 0 {
        println!("FizzBuzz");
    } else if x % 3 == 0 {
        println!("Fizz");
    } else if x % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", x);
    }
}