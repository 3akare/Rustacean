fn main(){
    let mut s = String::from("hello, ");
    borrow_object(&mut s);
    println!("Success!");
}

#[allow(unused_variables)]
fn borrow_object(s: &mut String) -> () {}
