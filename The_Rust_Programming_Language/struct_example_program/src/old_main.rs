#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // dbg! retorna a propriedade do valor da expressão
        height: 50,
    };

    //
    println!("\n--------------");
    println!(
        "The area of the rectangle is {} square pixels.",
        // A função usa referencia para a instancia não ser movida
        // Continuará ser usada abaixo
        area(&rect1)
    );

    // A struct não implementa Display, mas o debug mostra os valores
    println!("\n--------------");
    println!("Rectangle is '{:?}'", rect1);

    // dbg! é a saida do stderr (padrão é stdout)
    println!("\n--------------");
    dbg!(&rect1);
}

// A função usa referencia para a instancia não ser movida
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}