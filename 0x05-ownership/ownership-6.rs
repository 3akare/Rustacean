fn main(){
    let x: Box<u8> = Box::new(5); //x is a pointer to where 5 is being stored on the heap
    let mut y: Box<u8> = Box::new(1);

    *y = 4;

    assert_eq!(*x, 5);
    println!("Success!");
}
