
//Entry point of our application
fn main() {
    // Since println!() has !, Its a macro not function 
    println!("Hello, world!");
    println!("{}", is_even(20));
    println!("{}", fib(4));

    let name = String::from("Kishan");
    println!("The length of string is {}", get_str_len(name));
}

//Fucntion to check even number
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return false;
}

//Function to write fibbonacci of number
// 0 1 1 2 3 5 8
fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}

//Function to get string length
fn get_str_len(str: String) -> usize{
    str.chars().count()  //Implicit return
}