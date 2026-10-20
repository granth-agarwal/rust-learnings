use std::io;    // standard input output library

// main is the function which is run in Rust.
fn main() {
    println!("Guess the number!");  // `println!` is a macro, not a function/method.
    println!("Please input your guess.");

    /*
    in rust, variables are immutable by default. means their values can't be changed once assigned.
    hence we specifically use `mut` to make it mutable.
    `let` is just used to create a variable.

    String is a string data type provided by the standard library itself.
    basically, `String::new()` = `std::string::String::new()`
    `::` tells that the new function is an associated function of string type.
     */
    let mut guess = String::new();

    /*
    whenever we input something, we gotta store it in a mutable variable,
    coz that is how we change its value from whatever default it is, to the one we are inputting.

    if we want to keep that immutable, we can add mutability to a block like this:
    let input = {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    };

    we can also shadow the variable like this:
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input; // now immutable

    obviously, `&` symbol signifies a memory reference
     */

    io::stdin()     // part of std::io::Stdin, used to take input
        .read_line(&mut guess)  // takes input from terminal, reads a line as input as the name suggests. it returns a Result value, and Result is an enum. it can either be `Ok` or `Err`. obv as the name suggests, err represents an error.
        .expect("Failed to read line"); // in case of error, we need to have an error/exception handling logic, which is this logic. if we don't write it, the code will compile, but with a warning.

    /*
    printing with a placeholder.
    instead of this we can also do this:
    println!("you guessed: {}", guess);
    works just the same
     */
    println!("You guessed: {guess}");
}
