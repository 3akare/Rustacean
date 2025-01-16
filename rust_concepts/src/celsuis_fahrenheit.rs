use std::io;

pub fn celsuis_fahrenheit(){
    println!("Enter °F:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let val_f:f32 = input
        .trim()
        .parse()
        .expect("Please enter a number!");

    let val_c:f32 = (val_f - 32.0) * (5.0/9.0);
    println!("Celsuis (°C): {}", val_c);
}
