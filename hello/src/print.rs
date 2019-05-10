pub fn run(){
    // print to console
    println!("Hello World from print.rs file");

    //println!(1) não funciona!
    println!("Number: {}", 1);

    println!("{} vem antes do {}", "Primeiro", "Segundo");

    println!("{} vem antes do {} e {0} é numero {}", "Primeiro", "Segundo", 1);
    
    // argumentos com nomes
    println!("{name} gosta de {sport}", name = "Joao", sport = "Futebol");

    println!("Binario: {:b} Hexadecimal {:x} Octal {:o}", 10, 10, 10);

    //para debug
    println!("{:?}", (12, true, "hello"));

    // 
    println!("{} + {0} = {}", 10, 10 + 10);
}