use std::fmt::Display;

fn maior_com_um_anuncio<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Anúncio! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("a string longa é longa");

    {
        let string2 = String::from("xyz");
        let resultado = maior(string1.as_str(), string2.as_str());
        println!("A string mais longa é {}", resultado);
    }
}
