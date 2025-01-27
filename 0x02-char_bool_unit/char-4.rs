#[allow(unused_variables)]
#[allow(dead_code)]

fn main(){
    let _v: () = ();
    let v: (i32, i32) = (2, 3);

    assert_eq!(_v, implicitly_ret_unit());
    println!("Success!");
}

fn implicitly_ret_unit(){
    println!("I will return ()");
}

fn explicity_ret_unit() ->  () {
    println!("I will return ()");
}
