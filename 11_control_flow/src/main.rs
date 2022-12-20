fn main() {
    flow_if();
    flow_loop();
}

/****************************************
 * 条件分岐
 ****************************************/

fn flow_if() {
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

/****************************************
 * 繰り返し
 ****************************************/

fn flow_loop() {
    // loop {
    //     println!("again!");
    // }

    countup_loop();
    countdown_while();
    collection_for()
}

fn countup_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}

fn countdown_while() {
    let mut number= 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn collection_for() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number)
    }
    println!("LIFTOFF!!!");
}