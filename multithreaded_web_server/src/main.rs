use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // escutar
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() { // lista de coisas capturadas ao "ouvir"
        let stream = stream.unwrap();  // 'Result<T, E>' onde 'T' é um 'TcpStream' de 'std::net'

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {  // std::net::TcpStream
    let mut buffer = [0; 1024];  // Array com 1024 zeros. Um bufer de 1024 bytes.

    // A leitura do stream leva o buffer como parametro para preenche-lo
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    // response.as_bytes() -> slice de um array de inteiro 
    // stream.write()      -> Recebe u8 (inteiro de 8 bits)
    //
    // Significa que um slice de um array de inteiro, sera lido de 8 em 8 casas
    // e convertidos em caracteres utf8, que serão agrupados para a formatar uma String.

    stream.flush().unwrap();
    // 'flush' irá esperar e evitar que o programa continue até que todos os
    // bytes são gravados na conexão

    // -----------------------------------------------------------------------------
    //     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    //
    // 'from_utf8_lossy()' leva um inteiro de 8 bits (u8).
    // Significa que um slice de um array de inteiro, sera lido de 8 em 8 casas
    // e convertidos em caracteres utf8, que serão agrupados para a formatar uma String.
    
    // 'lossy' em 'from_utf8_lossy' é "Perda". Significa que quando processa 
    // uma sequência UTF-8 inválida (talvez no fim com menos de 8 bits),
    // essa parte é substituídas por '�', ' ' ou outro caractere.
}
