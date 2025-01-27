#[allow(dead_code)]
fn main(){
    println!("Success!");
}


fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => { }
        _ => { }
    };

    never_return_fn();
}

fn never_return_fn() -> ! {
    // panic!();
    todo!();
    // unimplemented!();
}
