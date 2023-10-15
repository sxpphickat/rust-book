use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please enter a number:");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
