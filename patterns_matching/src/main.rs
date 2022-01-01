fn main() {
    patterns_type();  // if let, while let, for, let, parametro de função
    refutability();
    pattern_syntax();  // Literais e escopo
    multiple_pattern();  // |, ..=
    destructuring_structs();
    destructuring_enums();
    complex_destructuring();
    ignoring_values_with_underscore();  // _
    ignoring_values_with_dots();  // ..
    match_guard(); // 'if' dentro do match. Escopo
    arroba_binding();  // Testar e capturar dentro do match
}

fn patterns_type() {
    // if let - Não é exaustivo
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse(); // _ Err não será usado

    // Se houver alguma coisa, guarde em 'let color'
    if let Some(color) = favorite_color {  
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {  // 'age' obscurecido/sombreado/shadowed
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // ------------------------------------------------
    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Emquanto houver alguma coisa, guarde em 'let top'
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ------------------------------------------------
    // for
    // O padrão está na sintaxe após o 'for', na caso abaixo
    // está nas variaveis 'index' e 'value'
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // ------------------------------------------------
    // let
    // Um simples 'let' tambem é um padrão, como: let PATTERN = EXPRESSION;
    let _x = 5;

    // Abaixo, o padrão tem que corresponder a quandidade,
    // e Ruste vincula o valor ao padrão
    let (_x, _y, _z) = (1, 2, 3);

    // ------------------------------------------------
    // Parâmetros de Função 
    fn foo(_x: i32) {}
    foo(5);

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);

}

fn refutability() {
    // Os padrões vêm em duas formas: refutáveis e irrefutáveis
    // Se sempre pode corresponder a algo é irrefutável, como: let x = 5;
    // Se nem sempre pode corresponder a algo é rrefutável,
    // como um 'Option' que há possibilidade de ser 'None' ao invés de 'Some'.

    // 'let' e 'for' somente aceitam padrões irrefutáveis.
    // 'if let' e 'while let' aceitam padrões refutáveis e irrefutáveis.

    let some_value: Option<i32> = None;

    // Saber disso ajuda entender o erro do compilador. O código abaixo não compila
    // pq 'Some' é refutavel, mas 'let' sozinho ó aceita irrefutavel.

    // let Some(x) = some_value;

    // Mas se usar o 'if', tranformamos o 'let' em 'if let' q aceita refutabilidade:
    if let Some(x) = some_value {
        println!("{}", x);
    }

    // O contrário tbm é verdade, e o código abaixo produz um Warning, pq
    // embora compile, pois a expressão 'if lef' aceita sim um irrefutavel, o VALOR passado
    // para a expressão é sempre irrefutavel, mas a expressão aceita também um refutavel.
    if let x = 5 {  // Warning
        println!("{}", x);
    };

}

fn pattern_syntax() {
    // Literais
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Nomes de variaveis no escopo de 'match', só funcionam no escopo de 'match'.
    // Para usar dentro do 'match', uma variavel fora do escopo de
    // 'match', use 'match guard' (exemplo de 'match guard' nas proximas funções)
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),  // 'y' nasce e marre aqui, não obscurece 'x'
        _ => println!("Default case, x = {:?}", x),  // não obscurece 'x'
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);  // Some(5) e 10 sem alteração
}

fn multiple_pattern() {
    // OU lógico
    let x = 1;
    match x {
        1 | 2 => println!("one or two"), // 1 ou 2
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Padrões multiplos com range (de num a num)
    let x = 5;
    match x {  // De 1 a 5; i.e, 1 ou 2 ou 3 ou 4 ou 5.
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';  // Igual acima, mas com 'char'.
    match x {    // 'char' é tratado como inteiro pelo compilador
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring_structs() {

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // As variaveis 'a' e 'b' são criadas com os valores de 'x' e 'y'
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Poderia ser o mesmo nome como: let Point { x: x, y: y } = p;
    // Mas aí o compilador reclama de redundancia com um warning.
    // Então passa-se somente o nome do valor, sem o nome do parametro 
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // Mas aqui causa confução

    // Desestruturar com match
    let p = Point { x: 0, y: 7 };

    match p {
        // Note que aqui só testa 'x', pq o y ja tem um valor direto, mesmo q errado, só pra ignorar
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),  // igualmente acima, aqui só testa y
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),  // Aqui testa os dois
    }

}

fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },  // struct
        Write(String),
        ChangeColor(i32, i32, i32),  // tupla
    }

    let msg_color = Message::ChangeColor(0, 160, 255);

    match msg_color {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {  // x: x, y: y
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(  // qualquer variavel servia, como 'a', 'b' e 'c'
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    // enum aninhado
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum NewMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        // Correto = Hsv
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

}

fn complex_destructuring() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn ignoring_values_with_underscore() {
    
    // parâmetros de função
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // match
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // Ignorar em um match com tupla
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // Ignorar variaveis
    let _x = 5;  // 'x' nunca é usado, mas o compilador não reclama
    let y = 10;  // 'y' nunca é usado, e o compilador reclama

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");  // _ não tem valor, não pode ser usado
    }

    println!("{:?}", s);

}

fn ignoring_values_with_dots() {

    // .. com struct
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        // Usar somente 'x' e ignorar o resto
        Point { x, .. } => println!("x is {}", x),
    }

    // .. com tupla
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {  // .. espande somete o nescessário
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // Cuidado! a expressão a seguir não funciona: (.., second, ..)
    // Rust não sabe quantos .. voce deseja pular para descobris o valor em 'second'
    // Seria melhor usar: (_, second, ..)
}

fn match_guard() {
    // Usa um 'if'
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Testar variaveis fora de match
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),  // y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // Combinar cou OU lógico
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),  //
    }
}

fn arroba_binding() {
    // O @ binding nos permite testar e salvar numa variavel
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,  // *
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {  // #
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),  // +
    }

    // *
    // Testamos se o valor está no range
    // e vinculamos o valor na variavel 'id_variable' usando o '@'.
    // O '@' funciona como um operador de atribuição. Ex:
    // MINHA_VAR = VALOR_DO_RANGE; Sendo que o sinal de '=' seria o '@'

    // #
    // Testamos se o valor é 10, 11 ou 12, mas não capturamos o valor
    // em nenhuma variavel.

    // +
    // Capturamos o valor na variavel 'id' abreviada (Note que é 'id' e não 'id: id'),
    // porém não fizemos nenhum teste de range ou OU lógico.
}