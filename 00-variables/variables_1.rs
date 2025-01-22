fn main (){
    let mut x: i32 = 1; // The mut keyword. By default variables in rust are immutable.
    x += 2;

    assert_eq!(x, 3);
    println!("Success");
}
