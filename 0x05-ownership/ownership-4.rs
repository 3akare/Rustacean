fn main(){
    let x: (u8, u8, (), &str) = (1, 2, (), "hello"); // String != &str. &str is a string literal and is hardcoded into the binary. It has a fixed size, and thus can be copied (not moved)
    let y = (x.0, x.1, x.2, x.3);

    println!("{:?}, {:?}", x, y);
}
