use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Título
    println!("Adivinhe o número.");
    // Número ramdômico
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // Pedir número de adivinhação 
        println!("Por favor, digite um número:");
        // Váriavel para guardar o número pedido
        let mut guess = String::new();
        // Input que pega o numero digitado e joga na variavel acima
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // (Shadow) parsear a var digitada para ver se é um caractere válido e
        // compatível com um número. se for, retorna inteiro
        let guess: u32 = match guess.trim().parse() {
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
            Ok(num) => num,
            Err(_) => continue,
        };

        // Exibir número digitado
        println!("Seu número é: {}", guess);

        // Verificar se 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Acertou! Mizeravi...");
                break;
            }
        }
    }
}