use std::io;

fn main() {
    println!("guess the number!");
    println!("please enter a number:");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("fail to read line");

    println!("You guessed: {guess}");
}
