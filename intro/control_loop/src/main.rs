fn main() {
    let num = 10;
    if num < 10 {
        println!("Less than 10");
    } else if num > 10 {
        println!("Greater than 10");
    } else {
        println!("Equal to 10");
    }
    // if expression must be a boolean
    // if branches must return the same type

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loop
    let mut counter = 0;
    while counter < 5 {
        println!("counter: {}", counter);
        counter += 1;
    }
    // or use break to exit the loop, and it could return a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
