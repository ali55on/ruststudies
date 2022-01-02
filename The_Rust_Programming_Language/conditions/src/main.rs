fn main() {
    // main_if();
    // main_loop();
    // main_loop_return();
    // main_while();
    main_for()
}

fn _main_if() {
    let number = 3;

    if number % 4 == 0 {
        println!("Número é divisível por 4");
    } else if number % 3 == 0 {
        println!("Número é divisível por 3");
    } else if number % 2 == 0 {
        println!("Número é divisível por 2");
    } else {
        println!("Número não é divisível por 4, 3 ou 2");
    }
    // ternário?
    let condition: bool = true;
    let _num = if condition { 5 } else { 6 };
}

fn _main_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Loop com retorno
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn _main_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn main_for() {
    let a = [10, 20, 30, 40, 50];
    // foreach
    for element in a {
        println!("the value is: {}", element);
    }
    // for ranged
    for number in (1..4).rev() {
        println!("{}!", number);
    }

}