mod chap0301;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    chap0302();
}

fn chap0302() {
    let x = 5;
    let x = x + 1;
    // let mut x = x + 1;
    {
        let x = x * 2;
        // x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is : {x}");

    // let mut spaces = "     ";
    // spaces = spaces.len();
}


fn main1() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is : {secret_number}");

    loop {

        println!("Please input your guess.");

        // let apples = 5; // imutable.  In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
        // let mut bananas = 5; // mutable

        let mut guess = String::new();  // UTF-8 encoded bit of text.
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        // like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

        // Handling Potential Failure

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };//.expect("Please type a number!");

        println!("You guessed: {}", guess);
        //     println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("Too small!")}
            Ordering::Equal => { println!("You win!"); break;}
            Ordering::Greater => { println!("Too big!")}
        }
    }
}
