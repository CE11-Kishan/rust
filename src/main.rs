
//Entry point of our application
fn main() {
    // Since println!() has !, Its a macro not function 
    println!("Hello, world!");
    println!("{}", is_even(20));
}

//Fucntion to check even number
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return false;
}

