pub fn run() {
    
    let mut c = 0;

    loop {
        c += 1;
        print!("{},", c);
        if c == 10 {
            println!("----");
            break;
        }
    }
    c = 0;
    while c <= 10 {
        if c % 15 == 0 {
            println!("FizzBuzz");
        } else if c % 3 == 0 {
            println!("Fizz");
        } else if c % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", c);
        }
        c += 1;
    }
    
    for c in 0..10 {
        print!("{},", c)
    }

}