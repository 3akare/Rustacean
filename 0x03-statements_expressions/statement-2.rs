fn main(){
    let v: u8 = {
        let x: u8 = 3;
        x
    };
    assert!(v == 3);
    println!("Success!");
}
