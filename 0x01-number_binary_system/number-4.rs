#[allow(unused_variables)]

fn main(){
    let v1: u16 =  251_u16 + 8_u16;
    let v2: u16 = u16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}
