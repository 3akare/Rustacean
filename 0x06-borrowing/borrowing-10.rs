fn main(){
    let mut s: String = String::from("hello ");

    let r1: &mut String = &mut s;
    r1.push_str("world");

    println!("{}", r1);

    let r2: &mut String = &mut s;
    r2.push_str("!");

}
