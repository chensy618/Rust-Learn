fn main() {
    // default immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // mutable variable
    let mut y = 5;
    y = 6;
    println!("The value of y is: {}", y);

    // shadowing : same concept as the local variable and the global variable in C/C++
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z is: {}", z);
    }
    print!("the original value of z is: {}", z);
    // type of int
    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    // f32, f64
    // bool
    // char
    // String
    // tuple : let tup: (i32, f64, u8) = (500, 6.4, 1); could store multiple types
    // array : fixed size, let nums: [i32; 5] = [1, 2, 3, 4, 5];
    // scope rule : variable is valid from the point at which it’s declared until the end of the current scope
    // const : immutable, must be annotated with a type
    // static : global variable, must be annotated with a type
}
