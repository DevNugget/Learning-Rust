use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess da number!");

    let secret_number = rand::thread_rng().gen_range(1..=200);

    loop {
        println!("Enter yo guess yes yes.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read dem line!");

        println!("Yo man guessed {guess}");

        let guess: u32 = guess.trim().parse().expect("Please type a number instead ma nah nahhh!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low ma dude!"),
            Ordering::Greater => println!("IT'S TOO BIG! *Cough* That's what she said."),
            Ordering::Equal => {
                println!("YEEE MAN YOU GOT IT!");
                break;
            }
        }
    }
}
