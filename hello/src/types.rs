/*
Integer u8, i8, u16, i16.. 128
Floats, f32, f64
Bollean (bool)
Characters (char)
Tuples
Array
*/

pub fn run () {
    
    // default type i32
    let _x  = 1;
    let _x2  = 3;
    
    // default f64
    let _y = 2.5;

    // tipo especifico
    let _z: i64 = 4545454545;

    //Encontre tamanho maximo para cada tipo
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;

    let eh_maior = _x > _x2;

    println!("{:?}", (_x, _y, _z, is_active, eh_maior));

    let a1= 'a';
    let face = '\u{1F600}';

    println!("{} => {}", a1, face);

}