use std::io;

pub fn fibonacci(){
    println!("Enter a number:");
    let mut a = 0;
    let mut b = 1;
    let mut n = 2;

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let target:u32 = input
        .trim()
        .parse()
        .expect("Please enter a number!");

    if target == 0 {
        println!("{}", 0);
        return;
    };

    if target == 1 { 
        println!("{}", 1);
        return;
    };

    let mut output:u64 = 0;
    while n <= target {
        output = a + b;
        a = b;
        b = output;
        n += 1;  
    };

    println!("{}", output);
}
