fn main() {
// iter: produz um iterador sobre referências imutáveis.
// into_iter: iterador que leva propriedade e retorna os valores da propriedade
// iter_mut: iterar sobre referências mutáveis
    
    
}

#[test]
fn iterator_test() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    //
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

// Uma coleção com iteradores usando filtro
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// Criando meu própio iterador: Itera até 5
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// Iterando meu iterador com outro iterador igual
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()      // Counter conta de 1 a 5
        .zip(Counter::new().skip(1))  // Emparelha com outro Counter no zip. Novo Counter começa do 2° item
        .map(|(a, b)| a * b)         // Multiplica items do 1° com 2°: 1*2=2, 2*3=6, 3*4=12, 4*5=20, 5*None=5
        .filter(|x| x % 3 == 0)     // Guarda valores que são multiplos de 3:     6,     12
        .sum();                    // Soma os itens pegos pelo filtro:            6   +  12 = 18

    assert_eq!(18, sum);          // Comparação: true
}