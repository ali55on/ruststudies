fn main() {
    let s1 = String::from("hello");

    // Não move o valo r de s1, por isso s1 pode ser usado abaixo
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // ------------------------
    // Referncias são imutaveis. para alterar, torne mutavel
    let mut s = String::from("hello");
    change(&mut s);

    // ------------------------
    // Uma referencia não pode ser mutavel logo após uma
    // ...referencia imutavel da mesma instacia.
    // A nova referencia só pode ser mutável após o ultimo
    // ...uso da referencia imutavel q foi criada antes. 
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Rust tbm previne "dangling pointer"
    // ...que são ponteiros para variaveis que foram descartadas
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
