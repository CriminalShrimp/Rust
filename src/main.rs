fn main() {

    print!("Hello world");


    let logical: bool = true;

    // No rust aloca-se espaco para as variaveis, mas elas tambem sao alocadas por padrao sem preicsar declar
    let a_float: f64 = 1.0;
    let an_integer   = 5i32;
    let  x = 9;
    println!("The value of x is: {}", x);// {} para printar a variavel
    let xs: [i32; 5] = [1, 2, 3, 4, 5];// Array
    let word = String::from("Hello, World!"); //string
    //Para mudar uma variavel é necessario escrever mut (provavelmnte de mutavel)  A funcao shadowing do rust faz possivel mudar variaveis.
    let mutable = true;
    let mut count = 0u32;

    // Exemplo de looping
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("Ta bao");
            break;
        }
    }
    // If
    if let x = 9{
        println!("é nove");
    } else {
        println!("n é nove");
    }


    // While
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("aa");
        } else if n % 3 == 0 {
            println!("AAA");
        } else if n % 5 == 0 {
            println!("AAAAAAAAAAAAAAAAAAAA");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
