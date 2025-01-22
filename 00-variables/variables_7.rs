fn main(){
    let (mut x, y) = (1, 2);
    x += 3;

    assert_eq!(x, 4);
    assert_eq!(y, 2);

    println!("Success!");
}
