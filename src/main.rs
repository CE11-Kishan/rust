use std::fs::read_to_string;
use chrono::{Local, Utc};

//Entry point of our application
fn main() {
    // Since println!() has !, Its a macro not function
    println!("Hello, world!");
    println!("{}", is_even(20));
    println!("{}", fib(4));

    let name = String::from("Kishan");
    println!("The length of string is {}", get_str_len(name));

    let user = User {
        username: String::from("CE11-Kishan"),
        email: String::from("Kishan@mgithub.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("The user is {}", user.username);

    let rect = Rect {
        width: 10,
        height: 20,
    };

    println!("The area of rectangle is {}", rect.area());

    println!("Without self parameter function behave as static method {}", Rect::debug());

    let direction = Direction::SOUTH;
    move_direction_to_north(direction);
    println!("The direction has changed to north");

    let rect = Shape::Rectangle(10.0, 2.0);
    println!("Area of rectangel is {}", calculate_area(rect));

    let index = find_first_a(String::from("Kishan"));
    match index {
        Some(value) => println!("Index is {}", value),
        None => println!("No 'a' found")
    }

    read_file();

    print_current_time();

    print_local_current_time();


    //Vector
    let num = vec![1,2,3];  // Intialize using macro
    let even_num = get_even_numbers(num);
    println!("Even numbers are {:?}", even_num);

    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    get_even_number_inplace(&mut numbers);
    println!("Even numbers are {:?}", numbers);


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
fn get_str_len(str: String) -> usize {
    str.chars().count() //Implicit return
}

//Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rect {
    width: u32,
    height: u32,
}

//Implement struct
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

//Enum
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

fn move_direction_to_north(direction: Direction) -> Direction {
    Direction::NORTH
}

//Enum with values
enum Shape {
    Rectangle(f64, f64),
    Circle(f64)
}

fn calculate_area(shape: Shape) -> f64 {
    //Pattern matching syntax
    match shape {
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => 3.14 * radius * radius
    }
}

// Option Enum
//Function to find first occurenece of char
fn find_first_a(str: String) -> Option<i32> {
    for (index, char) in str.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

// Result Enum
//Function to read content of file
fn read_file() {
    let content = read_to_string("Rust_Notes.txt");
    
    match content {
        Ok(data) => println!("{}", data),
        Err(error) => println!("Error: {}", error)
    }
}

// Use chrono carate
fn print_current_time() {
    let date = Utc::now();
    println!("The universal time is {}", date);
}

fn print_local_current_time() {
    let date = Local::now();
    println!("The current local time is {}", date);
}


// Understand Memory Management
fn create_string() {
    // Here the memory is created in heap
    // Here the owner of s variable is in block scope
    // As ownership goes out of scope then the memory is deallocated automatically
    // Unlike Garbage collector it doesnt need to run in background to deallocate memory
    let s: String = String::from("Kishan...");

    println!("The string is {}", s);
}

// Vector
// Function that takes a vector and return a vector with even values
fn get_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut even_numbers: Vec<i32> = Vec::new();

    for num in numbers {
        if num % 2 == 0 {
            even_numbers.push(num);
        }
    }

    return even_numbers;
}

fn get_even_number_inplace(numbers: &mut Vec<i32>) {
    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] % 2 != 0 {
            numbers.remove(i);
        }

        i += 1;
    }
}