use std::slice;

fn main() {
    raw_pointer();
    func();
    static_var();
    unsafe_trait();
}

fn raw_pointer() {
    let mut num = 5;

    // Faça a referencia e depois especifique os ponteiros e o tipo.
    let r1 = &num as *const i32;  // Referencia para 'num' como um ponteiro constante (imutavel) + T
    let r2 = &mut num as *mut i32;  // Referencia mutavel para 'num' como um ponteiro mutavel + T

    // Raw Pointers podem ser criados em código seguro.
    // Eles só não podem ser desreferenciados.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // O exemplo abaixo tbm é possivel, mas não há qualquer garantia de
    // que a memória estará a disposição 
    //
    //     let address = 0x012345usize;
    //     let r = address as *const i32;
}

fn func() {
    // Para chamar uma função insegura, precisamos de um bloco inseguro
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // Repare o código abaixo, pra fazer uma função com mesmo efeito
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // A função como acima.
    // Não precisa marcar a função toda como unsafe, somente o bloco inseguro
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        // Queremos retornar ([1, 2, 3], [4, 5, 6]) de uma [1, 2, 3, 4, 5, 6]

        let len = slice.len();
        assert!(mid <= len);

        // O código abaixo requer 2 referencias mutaveis para o mesmo slice.
        // O verificador de emprestimo bloqueia, pq n podemos fazer 2 referencias
        // mutaveis para o mesmo código. Mas esse código não é inseguro, porque
        // embora seja mutavel, as referencias não mudam o código. Mas pelas regras
        // de Rust, o compilador não sabe.
        // 
        //     (&mut slice[..mid], &mut slice[mid..])  //  de zero a mid e de mid ao fim

        // Então a abordagem é usar um raw pointer mutável para essa 'slice', pq o 
        // verificador de emprestimo "não funciona" com ele. vc tem poderes e responsabilidades
        let ptr = slice.as_mut_ptr();
        // Lembrete: Ponteiro raw é aceito em código seguro, por isso está antes de unsafe bloco.
        // Operações em ponteiros raw como a desreferencia dele, não são seguras.

        unsafe {
            (  
                // O 'slice::' abaixo é um 'std::slice' para operações de fatias,
                // e não deve ser confundido com o 'slice' na assinatura da função.
                // Aqui, o 'slice' da assinatura é o ponteiro 'ptr'

                // O ponteiro oaponta para o primeiro indice da fatia
                slice::from_raw_parts_mut(ptr, mid),  // vai do ponteiro a mid

                // ptr.add(mid) coloca o ponteiro no indice de mid
                // Aqui não se usa '..', então para calcular o fim usamos um número
                // que indica quantidade de itens que falta de mid até o fim
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),  // vai de mid ao fim

                // Note que 'slice::from_raw_parts_mut()' diz:
                //     "De fatias, eu quero essas raw partes"
                // Daí vc passa os indices dentro da chamada '(init_part: u32, end_part: u32)'

                // Unsafe onde? se declarar ponteiros raw é aceito em código seguro!?
                // A função 'slice::from_raw_parts_mut' é inseguro porque leva uma raw pointer e
                // deve confiar que este ponteiro é válido.
                // O 'add()' também não é seguro, porque ele deve confiar que o local de
                // deslocamento também é um ponteiro válido.

            ) // Aki Já é um retorno
        } 
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = split_at_mut(&mut v[..], 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

}

fn static_var() {
    // Variaveis estaticas imutaveis são seguras
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    // Variaveis estaticas mutaveis são inseguras
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}

fn unsafe_trait() {
    // Se o trait tem pelo menos um método que possui código unsafe
    // ele tem q ser marcado como unsafe

    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // Se implementarmos um tipo que contém um tipo que não é Send ou Sync,
    // como ponteiros raw, e queremos marcar esse tipo como Send ou Sync, devemos usar unsafe. 
}