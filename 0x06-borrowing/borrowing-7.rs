fn main(){
    let s: String = String::from("hello");
    let r1: &String = &s;
    let r2: &String = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
