pub fn run(){
    let hello = "world";
    let mut hello2 = String::from("Hello World 2");

    println!("{} and {}", hello, hello2);

    println!("String \"{}\" - Length: {}", hello2, hello2.len());
    println!("String \"{}\" - Capacity: {}", hello2, hello2.capacity());
    println!("String \"{}\" - Is empty: {}", hello2, hello2.is_empty());

    hello2.push('W');
    hello2.push_str("orld!");
    println!("{}", hello2);
    
    //loop
    for word in hello2.split_whitespace() {
        println!("{}", word );
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
    
    //testing
    assert_eq!(2, s.len());

    assert_eq!(10, s.capacity());
    
}