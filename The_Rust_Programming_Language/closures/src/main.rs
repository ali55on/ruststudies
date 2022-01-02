use std::thread;
use std::time::Duration;

// Um cache cujo a propriedade 'valor' só faz o calculo do valor
// na primeira vez em que é usado. depois o valor já está em cache.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,  // Fn é o tipo 'trait' de uma closure
                        // i.e, Fn de função. Pq uma closure é uma função anonima.

                        // FnOnce: consome as variáveis
                        // FnMut: pode mudar o ambiente porque empresta valores mutuamente.
                        // Fn: toma emprestados valores do meio ambiente imutavelmente.
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // A variavel 'expensive_result' tem um tipo 'Cacher' com valor None;
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // quando 'expensive_result.value' for usado, ele gerará o valor
    // e daí manterá esse valor sem ter q gerar denovo da próxima vez.
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Closures, diferente das funções, podem acessar variáveis do escopo
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // Pode forçar mover uma variavel com a palavra 'move'
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // A var 'x' no pode mais ser usada, então o print abaixo não pode existir
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}


// Anotação de tipo é assim:
// let expensive_closure = |num: u32| -> u32 {...};

// O compilador pode inferir o tipo.
// Os exemplos abaixo são equivalentes. Note q o primeiro é função
//
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;


// O closure guarda o valor inferido, daí n pode mais ser usado com outro valor.
// O código abaixo não compila.
//
// let example_closure = |x| x;
//
// let s = example_closure(String::from("hello"));
// let n = example_closure(5);

// Note que ele guarda não só o tipo, mas tambem o valor.
// O código abaixo também não compila.
//
// let s = example_closure(2);
// let n = example_closure(5);
