fn main() {
    hello();

    //https://pt.stackoverflow.com/questions/485812/quais-s%C3%A3o-as-diferen%C3%A7as-entre-string-e-str-em-rust
    let _s = "Hello".to_string();
    let _s2 = String::from("world");
    let _s3: String = "also this".into();

    let alias = "linux".to_string();
    wellcome("Tux", alias);

    // Expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // função com retorno usa expresão
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn hello() {
    println!("Hello, world!");
}

fn wellcome(name: &str, alias: String) {
    println!("Wellcome {} ({})", name, alias);
}

// função com retorno usa expresão
fn plus_one(x: i32) -> i32 {
    x + 1
}