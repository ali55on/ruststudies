use std::io;


fn main() {
    // Tuples: fix size
    // ----------------
    let numeros: (i32, f64, u8) = (200, 5.6, 1);
    let (x, y, z) = numeros;
    println!("X: {}, Z: {}, Y: {}", x, z, y);
    // Os valores são acessados com ponto
    let t = (500, 6.4, 1);
    let five_hundred = t.0;
    let six_point_four = t.1;
    let one = t.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // Arrays: fix size
    // ----------------
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // Para tipar um array (matriz) coloque o tipo e a quantidade de itens
    // ...separados por ponto e virgula.
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    // Os valores são acessados com colchetes
    let _first = v[0];
    let _second = v[1];

    // Para criar um array que contém o mesmo valor para cada elemento,
    // ...você pode especificar o valor inicial, seguido por um ponto-e-vírgula
    // ...e o comprimento da array
    let _a = [3; 5];

    // Pegar valor do user:
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
    // O codigo acima é igual a: let _a = [3, 3, 3, 3, 3];

    // Vectors
    // -------
}