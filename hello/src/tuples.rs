/*
grupo de elemento no max de 12 elementos
*/
pub fn run(){
    
    let person: (&str, &str, i8) = ("Joao", "Silva", 37);

    println!("Nome {} -  Sobrenome {} - Idade {}",  person.0, person.1, person.2);

}