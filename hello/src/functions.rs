pub fn run () {
    greeting("Hello", "name");
    
    let get_sum = add(1, 3);
    println!("get_sum: {}", get_sum);
    println!("add({})", add(1, 2));

    let n3: i32 = 10;
    let add_nums = | n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}