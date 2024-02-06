// library carate ~ package : a collection of Rust source code files.
// binary carate ~ the code we write which is executable
// Rust doesn't yet included "random" functionality in it's standard library
use std::io;
// Rnd is a trait. Traits are mehotd which implemented by carate.
// cargo doc --open => build documentaion provided by all your dependencies locally.
use rand::Rng;
// Ordering is an enum with these variants : Less , Greater , Equal
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
	println!("{}" ,secret_number);
	println!("Guess the number");
    println!("Please enter your guess:");
    // new is an associated function
    let mut guess = String::new();
    // std::io::stdin()
    // read_line will append the entered text to end of the variable content
    // & indicates that the this argument is a reference
    // "mut" make the reference mutable
	// read_line returns a Result type which is an ENUM and it's purpose is for error-handling
	// each state of that enum called a "variant".
	// Result's variants os Ok and Err.
    io::stdin().read_line(& mut guess).expect("failed to read line");
    // println!("your guess is {}",guess);
	println!("your guess is {guess}");
	// These are match expression arms
	let guess:u32 = guess.trim().parse().expect("enter a valid number");
	match guess.cmp(&secret_number){
		Ordering::Less => println!("Too Small"),
		Ordering::Greater => println!("Too big"),
		Ordering::Equal => println!("You win!"),
	}
}
