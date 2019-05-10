//Lista fixa de valores
use std::mem;

pub fn run(){
    
    //mut para poder alterar o valor durante o codigo
    // Arrays
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Arrays: {:?}", numbers);
    println!("Segundo elemento: {}", numbers[1]);
    numbers[1] = 20;
    println!("Segundo elemento: {}", numbers[1]);
    println!("Tamanho do array: {}", numbers.len());
    println!("Tamanho na memoria: {} bytes", mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
    
    println!("===================");
    
    // Vectors
    let mut numbers_v:  Vec<i32> = vec![1, 2, 3, 4];
    println!("Vectors: {:?}", numbers_v);
    println!("Segundo elemento: {}", numbers_v[1]);
    numbers_v.push(5);
    println!("Vectors: {:?}", numbers_v);
    numbers_v.pop();
    println!("Vectors: {:?}", numbers_v);

    //loop
    for x in numbers_v.iter() {
        println!("Number {}", x);
    }

    for x in numbers_v.iter_mut() {
        *x *= 2;
    }
    println!("Number multiplicados por 2 {:?} ", numbers_v);
}