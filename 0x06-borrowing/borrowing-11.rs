#[allow(unused_variables)]
fn main(){
    let mut s: String = String::from("hello, ");
    let r1: &mut String = &mut s;
    let r2: &mut String = &mut s;

    println!("{}", r1);
}
