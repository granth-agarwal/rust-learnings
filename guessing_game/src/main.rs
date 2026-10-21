use std::io;    // standard input output library
use rand::prelude::*;   // rand is a crate, and crate is a collection of rust source code files which allow specific functionality, usually not included with the std package.
// The prelude module contains the most commonly used parts of the rand crate, and use makes those items available in our program’s scope.
use std::cmp::Ordering;

// main is the function which is run in Rust.
fn main() {
    println!("Guess the number!");  // `println!` is a macro, not a function/method.

    /*
    secret_number is an immutable variable. rand is the package.
    rng() provides a generator to generate random numbers
    random_range() takes a range and returns a number accordingly
    1..=100 is the range syntax, .. is the separator-type thing, and 1 and 100 is inclusive.
    beginning is always inclusive, 1..100 will have end exclusive.
    */
    let secret_number = rand::rng().random_range(1..=100);

    /*
    in rust, variables are immutable by default. means their values can't be changed once assigned.
    hence we specifically use `mut` to make it mutable.
    `let` is just used to create a variable.

    String is a string data type provided by the standard library itself.
    basically, `String::new()` = `std::string::String::new()`
    `::` tells that the new function is an associated function of string type.
     */
    let mut guess = String::new();

    loop {
        println!("Please input your guess.");

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
        this is how we shadow guess, like we learnt above.
        when we enter a value from terminal to guess through read_line, we press enter afterwards, which adds a newline character to it.
        trim() removes the newline character and/or whatever breaks the input might have.
        parse() looks at the data type clearly defined with the variable (u32) here and converts the String to that data type.
         */
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        /*
        printing with a placeholder.
        instead of this we can also do this:
        println!("you guessed: {}", guess);
        works just the same
         */
        println!("You guessed: {guess}");
        /*
        Ordering is an enum which has variants, Less, Greater, and Equal. self-explanatory.
        match is similar to switch case.
        A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        now it compares the guess with secret_number.
        passed secret_number as a reference because cmp method expects it like that. if we pass a value, it gives an error.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
