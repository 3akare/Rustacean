#[allow(unreachable_code)]
fn main(){
    never_return();
    println!("Failed");
}

fn never_return() -> ! { panic!() }
