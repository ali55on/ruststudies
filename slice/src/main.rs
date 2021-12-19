fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice1 = &s[0..len];
    let slice2 = &s[..len];
    let slice3 = &s[0..];
    let slice4 = &s[..];

    println!("{}, {}, {}, {}", slice1, slice2, slice3, slice4);

    // --------
    let worlds = String::from("Hello world");
    let first_w = first_word(&worlds);
    println!("{}", first_w);

    // --------
    // String literal (&str) tambem são fatias (Slice)
    // Então uma função pedindo &str funciaona com String  e &str
    let my_string = String::from("hello world");
    let _word = best_first_word(&my_string[0..6]);
    let _word = best_first_word(&my_string[..]);
    let _word = best_first_word(&my_string);

    let my_string_literal = "hello world";
    let _word = best_first_word(&my_string_literal[0..6]);
    let _word = best_first_word(&my_string_literal[..]);
    let _word = best_first_word(my_string_literal);

    // --------
    // Slice funciona com vetor
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn best_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
