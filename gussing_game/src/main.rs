use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    println!("the secret number is {secret_num}");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    loop {
        
        println!("please input you data");

        match guess.cmp(&secret_num) {
            Ordering::Less => {
    
                println!("Too small!")
            },
            Ordering::Greater => {
                println!("Too big!");
                break;
            },
            Ordering::Equal => println!("You win!"),
        }

    }
}
