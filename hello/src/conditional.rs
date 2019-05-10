pub fn run() {
    let age: u8 = 20;
    let check_id: bool = true;

    if age >= 21 && check_id {
        println!("age {}", age);
    } else if age < 21 && check_id {
        println!("Menor que 21 anos");
    } else {
        println!("else");
    }

    let is_of_age = if age >= 21 {true} else {false};
    println!("is_of_age {}", is_of_age );
}