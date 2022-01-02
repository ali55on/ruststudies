#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_one(a: i32) -> i32 {
    a + 1
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // assert!() é true
    // assert!(!) é false
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));  // fail
    }

    // assert_eq!() é true
    // assert_ne!() é false
    #[test]
    fn it_adds_one() {
        assert_ne!(4, add_one(2)); // fail
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // format
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // should_panic só passa se entrar em panico
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // O should_panic espera a mensagem que contenha o texto como mensagem do panico.
    // Se o texto recebido da mensagem do panico for diferente, então esse teste falha.
    fn greater_than_100() {
        Guess::new(200);
    }

    // Usando "Result<T, E>"
    // Não pode usar o #[should_panic] em testes que usam Result<T, E>
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
