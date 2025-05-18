use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gues the number");
    let secret_number = rand::thread_rng().gen_range(0..=10);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Numero del usuario
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read linea");

        //Convertir la entrada del usuario a numero
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //Comparar el numero del usuario con el numero secreto
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smalll!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}
