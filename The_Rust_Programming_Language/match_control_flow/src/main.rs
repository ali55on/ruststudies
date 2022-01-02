#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Quarter(state) => {
            println!("{:?}", state);
            0
        },
    }
}

fn add_fancy_hat(num: i32) { println!("{}: Adicionando chapéu!", num); }
fn remove_fancy_hat(num: i32) { println!("{}: Removendo chapéu!", num); }
fn reroll() { println!("Rolando outra vez!"); }

fn main() {
    // match na função
    println!("\n-------------");
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // match somente em valors válidos
    println!("\n-------------");

    let mut dice_roll = 0;
    loop {

        match dice_roll {
            3 => add_fancy_hat(3),
            7 => remove_fancy_hat(7),
            _ => (),
            // Também pode se usar o 'if left' no lugar de '_ => ()'
            // https://doc.rust-lang.org/book/ch06-03-if-let.html
        }

        if dice_roll == 10 {
            break
        } else {
            dice_roll += 1;
        }
    }

    // match com execução de valor opcional
    println!("\n-------------");

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(3),
        7 => remove_fancy_hat(7),
        _ => reroll(),
    }
}
