#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main(){
    let v: u8 = {
        let mut x: u8  = 1;
        x += 2;
        x
    };

    let _v: u8 = {
        let x: u8 = 1;
        x + 2
    };

    let __v: () = {
        let mut x: u8 = 1;
        x += 2
    };

    assert_eq!(v, 3);
    assert_eq!(_v, 3);
    assert_eq!(__v, ());
    println!("Success!");
}
