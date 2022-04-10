use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Play guess");

    let secret_num = rand::thread_rng().gen_range(1..101);

    println!("Secret num is: {}", secret_num);
    println!("Enter you guess");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("FAILED READ LINE");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You won!"),
    }

    println!("You entered: {}", guess);
}
