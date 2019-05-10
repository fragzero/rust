use std::env;

pub fn run () {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let status = "100%";

    println!("Argumentos {:?}", args);
    println!("Command {}", command);

    if command == "status" {
        println!("Status is {} ", status);
    }
}