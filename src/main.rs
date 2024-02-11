use std::{
    cmp::Ordering,
    io
};

use rand::Rng;

fn main() {
    println!("=== Adivinhe o número ===");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        
        println!("Insira seu palpite.\n");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Erro ao ler a linha de código.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {guess} \n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("==> O seu valor é menor..."),
            Ordering::Greater => println!("==> O seu valor é maior..."),
            Ordering::Equal => {
                println!("=== Você acertou ==="); 
                break;
            }
        }
    }
}
