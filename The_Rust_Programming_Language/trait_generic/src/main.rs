// Trait
pub trait Resumir {
    fn resumo_autor(&self) -> String;  // será implementado na instancia

    fn resumo(&self) -> String {
        format!("(Leia mais de {}...)", self.resumo_autor())
    }
}

// Tweet struct
pub struct Tweet {
    pub nomeusuario: String,
    pub conteudo: String,
    pub resposta: bool,
    pub retweet: bool,
}

impl Resumir for Tweet {
    fn resumo_autor(&self) -> String {
        format!("@{}", self.nomeusuario)
    }
    // resumo() está implementado no trait
}

// Artigo
pub struct ArtigoDeNoticia {
    pub titulo: String,
    pub local: String,
    pub autor: String,
    pub conteudo: String,
}

impl Resumir for ArtigoDeNoticia {
    fn resumo_autor(&self) -> String {
        format!("@{}", self.autor)
    }

    fn resumo(&self) -> String {
        format!("{}, by {} ({})", self.titulo, self.autor, self.local)
    }
}

// Generic
pub fn notificar<T: Resumir>(item: T) {
    println!("Notícias de última hora! {}", item.resumo());
}

// fn alguma_funcao<T: Mostrar + Clone, U: Clone + Debug>(t: T, u: U) -> i32{}
// fn alguma_funcao<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug {
// }

// Maior
fn maior<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut maior = list[0];

    for &item in list.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

// Main
fn main() {
    // Tweet
    let tweet = Tweet {
        nomeusuario: String::from("horse_ebooks"),
        conteudo: String::from("claro, como vocês provavelmente já sabem, pessoas"),
        resposta: false,
        retweet: false,
    };
    println!("1 novo tweet: {}", tweet.resumo());

    // Artigo
    let artigo = ArtigoDeNoticia {
        titulo: String::from("Os Penguins ganham a copa do campeonato Stanley"),
        local: String::from("Pittsburgh, PA, USA"),
        autor: String::from("Iceburgh"),
        conteudo: String::from("Os Penguins de Pittsburgh são novamente o melhor time de hockey da NHL."),
    };
    println!("Novo artigo disponível! {}", artigo.resumo());

    // Maior int
    let lista_numero = vec![34, 50, 25, 100, 65];
    let result = maior(&lista_numero);
    println!("O maior número é {}", result);

    // Maior char
    let lista_char = vec!['y', 'm', 'a', 'q'];
    let result = maior(&lista_char);
    println!("O maior char é {}", result);
}