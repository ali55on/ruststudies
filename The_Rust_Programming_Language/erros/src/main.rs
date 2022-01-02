use std::io::Read;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    // ---
    // Obs

    // Se no seu projeto você precisa tornar o binário final
    // o menor possível, você pode deixar de resolver e sim
    // abortar no panic adicionando panic = 'abort' à seção
    // apropriadada de [profile] no seu arquivo Cargo.toml
    //     [profile.release]
    //     panic = 'abort'

    // Para capturar um erro, Rust usa enum:
    //     enum Result<T, E> {Ok(T), Err(E),}

    // Para descobrir um tipo de um valor, é só gerar um erro (macro panic!)
    // que o compilador mastra o tipo:
    //     let f: u32 = File::open("hello.txt");  // panic!, File não é u32

    // ----------------------------------------------
    // Gerar um erro irrecuperável com o macro panic!

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error)
        },
    };

    // ------------
    // Tratar erros

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tentou criar um arquivo e houve um problema: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "Houve um problema ao abrir o arquivo: {:?}",
                error
            )
        },
    };

    // --------------------------------------------
    // Atalhos para Pânico em Erro: unwrap e expect

    let _f = File::open("hello.txt").unwrap();  // Se falhar devolve um panic
    let _f = File::open("hello.txt").expect("Falhou ao abrir hello.txt");

    // ----------------
    // Propagando Erros

    // long: Normal
    let _res = read_username_from_file_long();

    // short: Only on function, using '?'
    let _res = read_username_from_file_short();

    // very short: Only on function, using '?'
    let _res = read_username_from_file_very_short();

    // ----------------------------------------
    // Entrar em panic! ou Não Entrar em panic!

    // https://livro.rustbr.org/ch09-03-to-panic-or-not-to-panic.html

}

fn read_username_from_file_long() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // Note return
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_very_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

