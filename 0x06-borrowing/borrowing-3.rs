fn main(){
    let mut s: String = String::from("hello, ");
    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}
