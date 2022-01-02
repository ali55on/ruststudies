fn main() {
    // 'Option<T>' é usado quando se quer verificar um
    // valor nulo. Valores nulos não existem em 'Rust'.
    //
    // A estrutura de option é um enum normal:
    //
    //     enum Option<T> {
    //         None,
    //         Some(T),
    //     }
    //
    // O '<T>' significa 'tipo', como 'i32'
    //
    // 'Some' é abreviação de 'Option::Some()'
    // 'None' é abreviação de 'Option::None'
    

    // Para usar uma var do tipo None, é
    // nescessário explicitar o tipo
    let _absent_number: Option<i32> = None;

    // Um valor como 'Option<i32>' é diferente de 'i32'
    // Por isso um valor 'i32' não pode ser
    // atribuido a um 'Option<i32>'. O código abaixo falha:
    //     let num: Option<i32> = Some(5);
    
    // Códigos que funcinam:
    let _number = Some(5);
    let _number: Option<i32> = Some(5);
    let _number: Option<i32> = Option::Some(5);
    let _string = Some("a string");

    // Option<T> com match
    let five = Some(5);
    let _six = plus_one(five);   // 'five' é um Option(i32)
                                 // e o resultado aqui é 6, como: Option::Some(6)
    let _none = plus_one(None);  // 'None' é um Option::None

    // Também pode se usar o 'if left' no lugar de '_ => ()'
    // https://doc.rust-lang.org/book/ch06-03-if-let.html
}

fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
        // No 'match' a verificação é exaustiva.
        // Não pode faltar nenhuma verificação
        // de todos os valores de um 'enum' ou 'struct'
        // que forem verificados com o match.
        // Se faltar a verificação de 'None' nesta função aqui,
        // o compilador não compila.
}