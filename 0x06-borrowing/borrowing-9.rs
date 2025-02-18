fn main(){
    let mut s = String::from("hello, ");
    
    borrow_object(&s);

    s.push_str("World");

    println!("Success!");
}

#[allow(unused_variables)]
fn borrow_object(s: &String) -> () {}
