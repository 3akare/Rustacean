use std::mem::size_of_val;

fn main(){
    let _v: () = ();
    assert_eq!(0, size_of_val(&_v));

    println!("Success!");
}
