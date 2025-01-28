fn main(){
    let x: String = String::from("hello world");
    let y: String = x.clone(); // expensive
    println!("{}, {}", x, y);
}

