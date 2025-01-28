fn main(){
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("hello world");
    let _s = s.clone().into_bytes(); //s can't be used after this
                                     // Another solution with be s.as_bytes(), uses the reference
                                     // of s
    s
}
