use std::sync::{Arc, Mutex, mpsc};
// use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    thread_vs_main();
    thread_ownership_with_move();
    channel_thread();
    thread_with_mutex();
}

//
fn thread_vs_main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// move
fn thread_ownership_with_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Transmitir e receber dados de uma thread pelo canal
fn channel_thread() {
    // Uma mensagem só
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from(":)");
        tx.send(val).unwrap();
        // Após enviar 'val' para 'send()', a posse de 'val' está em 'send()',
        // Não é mais possivel usar 'val' a partir dessa linha
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Várias mensagens
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Mutex
fn thread_with_mutex() {
    // Uma thread
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;  // m.lock() devolve um ponteiro inteligente, por isso a derreferencia
    }

    println!("m = {:?}", m);

    // Várias threads
    let counter = Arc::new(Mutex::new(0));  // Arc = Atomic reference counter
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {  // Garantir que threads terminem
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}