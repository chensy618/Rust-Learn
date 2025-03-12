fn main() {
    {
        // each value in Rust has a variable that’s called its owner
        let s1 = String::from("hello");
        println!("{}, world!", s1);
    }
    // move
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world!", s1); // error[E0382]: use of moved value: `s1`
        println!("{}, world!", s2);
    }
    // clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    // borrow
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // pass a reference, the ownership is not transferred
        println!("The length of '{}' is {}.", s1, len);
    }
    // lifetime
    {
        let s1 = String::from("hello12312312312");
        let s2 = String::from("world23213");
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is '{}'.", result);
    }
    // pattern match
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_and_return(s1);
        println!("The length of '{}' is {}.", s2, len);

        let number = 3;
        match number {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            _ => println!("Others"),
        }
        println!("number is {}", number);
    }
    //Option<T>
    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
        println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn calculate_length_and_return(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
