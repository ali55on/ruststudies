fn main() {
    // struct User {active: bool, name: String, email: String, count: u32,}
    // struct sempre tem virgula após cada item, inclusive no último.
    // struct não possui ponto e virgula no fim, pois é uma instrução,
    // ...e não uma linha.
    
    // let user_1 = User {active: true...};
    // A instancia é uma linha e tmina com ponto e virgula.
    let user_1 = User {
        active: true,
        name: String::from("alien"),
        email: String::from("@mail"),
        count: 1,
    };

    println!("{}", user_1.name);

    // -----------------
    println!("------------------");
    
    // Para alterar um valor (como um setter), a instancia da struct, e
    // ...não a struct, precisa ser mutavel.
    let mut user_2 = User {
        active: true,
        name: String::from("E.T"),
        email: String::from("et@mail"),
        count: 1,
    };
    println!("{}", user_2.email);
    user_2.email = String::from("varginha@et");
    println!("{}", user_2.email);

    // -----------------
    println!("------------------");

    // Contruir um usuario passando somente aulguns parametros
    // e deixando outros como pré-definidos
    let my_user = build_user(String::from("my@mail"), String::from("myname"));
    println!("{}", my_user.email);

    // -----------------
    println!("------------------");

    // Pode usar valores de uma struct na outra
    let user_3 = User {
        active: user_2.active,
        name: user_2.name,  // veio movido, user_2.name agora é inválido
        email: String::from("et@mail"),
        count: user_2.count,
    };

    // println!("{}", user_2.name); causa um erro pq foi movido

    // Como a nova struct é quase igual a antiga, onde
    // só mudou realmet 1 valor, posso usar a sintaxe
    // de atualização ".."
    let user_4 = User {
        email: String::from("et@mail"),
        ..user_3  // Sempre no fim
    };  // user_3 foi movido para _user_4 e não pode mais ser usado
    println!("{}", user_4.name);

    // -----------------
    println!("------------------");

    // struct de tuplas
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(1, 2, 3);
    let _origin = Point(0, 0, 0);
    println!("{}", black.1);

    // -----------------
    // Unit-Like Structs / estruturas de unidades semelhantes
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

#[derive(Debug)]  // Suprimir warnings: field is never read
struct User {
    active: bool,
    name: String,
    email: String,
    count: u32,
}

 fn build_user(email: String, name: String) -> User {
    User {  // Aqui já é o retorno
        email,
        name,
        active: true,
        count: 1,
    }  // Sem ponto e virgula pq é o retorno
}