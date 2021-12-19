use std::collections::HashMap;

fn main() {
    test_vector();
    test_string();
    test_hash_map();
}

fn test_hash_map() {
    // Criar mapa de hash (como um dicionario)
    let mut scores = HashMap::new();  // mutavel para inserir posteriormente
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // tambem pode-se usar o collect de um vetor
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let _scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // ---------
    // ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    // --------------
    // Pegar um valor
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);
    // get() aqui retorna Some(&10) ou None

    // --------
    // Iterando
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // ---------------------
    // Sobrescrever um valor
    let mut scores = HashMap::new();

    // Funciona como um dict python, ainda é a mesma chave
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Só insere se o a chave/valor não existir
    scores.entry(String::from("Yellow")).or_insert(50);  // update
    scores.entry(String::from("Blue")).or_insert(50);    // no

    // Note que o or_insert() em entry() é definido para retornar
    // uma referência mutável para o valor lá dentro do mapa.
    // Isso quer dizer q posso atribuílo a uma variavel, e se eu
    // mudar a variavel, o valor lá dentro do mapa tbm muda
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // A operação acima criou um mapa com a contagem de cada palavra.
    println!("{:?}", map);  // {"hello": 1, "world": 2, "wonderful": 1}
}
fn test_string() {
    // Criar string sem valor
    let _s = String::new();
    // Criar string com valor
    let _s = String::from("initial contents");

    // Criar string com valor a partir de uma str literal
    let data = "initial contents";
    let _s = data.to_string();
    // Criar string a partir de uma str literal, tbm funfa diretamente
    let _s = "initial contents".to_string();

    // ----------------------
    // Atualizando uma string
    let mut s = String::from("foo");
    s.push_str("bar");

    // o método push_str() envia a string como slice,
    // i.e, envia os endereços dos caracteres por referencia, assim
    // o valor de s2 não perde a propriedade (ownership)
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Diferente do método push_str(), o método push()
    // envia um único caractere
    let mut s = String::from("lo");
    s.push('l');

    // -----------------------------------
    // Atualizando string por concatenação
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2;
    // Note q s1 foi movido e não pode mais ser usado.
    // Já o s2 foi passado por referencia, pq o sinal + usa
    // o método add(), parecido com: fn add(self, s: &str) -> String {
    // Mesmo que o parametro de add() seja &str ao invéz de &String, podemos
    // usar também uma &String, porque na implementação real de add(), ele usa
    // um recurso chamado "coerção", para converter o &String em &str.

    // Várias Strings concatenadas é difícil de ler
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    // Várias Strings concatenadas
    // é mais fácil de ler usando o macro !format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);
    // O macro !format usa referencias para não se apropriar de
    // enhuma variavel, assim elas podem ser reutilizadas posteriormente

    // ----------
    // Fatias de string

    let hello = "Здравствуйте";
    let _s = &hello[0..4];
    // Зд  // pq são 2 bytes nesse idioma

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // न
    // म
    // स
    // ्  // sináis diacríticos
    // त
    // े  // sináis diacríticos

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // 224
    // 164
    // --snip--
    // 165
    // 135

}

fn test_vector() {
    // Cria um vetor sem inicia-lo. Precisa do tipo.
    let _v1: Vec<i32> = Vec::new();
    // Cria um vetor ja iniciado. O tipo é inferido.
    let _v2 = vec![1, 2, 3];
    // o vetor é sempre do mesmo tipo. se adicionar um tipo
    // ele se manterá, então não especificamos o tipo.
    let mut v3 = Vec::new();  // mut para alterar depois
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // -------------------------
    // Ler elementos de um vetor
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    // -------------------------
    // Ler elementos fora do range
    match v4.get(100) {
        Some(some) => println!("The third element is {}", some),
        None => println!("There is no third element."),
    }

    // ---------------------
    // Regras de propriedade
    let mut v5 = vec![1, 2, 3, 4, 5];

    v5.push(6);  // Pode mudar

    let first = &v5[0];  // Referência imutavel

    // v5.push(6);  // Erro: Não pode mudar após uma referência imutavel

    println!("The first element is: {}", first);

    // --------
    // Iterando
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    // Mudar o valor por desreferenciar.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // -------------------------
    // Tipos diferentes com enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
