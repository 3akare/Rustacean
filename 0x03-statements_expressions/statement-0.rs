fn main(){
    let x: u32 = 5_u32;

    // ----- statement & expression ----- //
    
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
        // ----- expression ends -----// 
    };

    // ----- statement ends -----//

    let z: u32 = {
        2 * x  // this value is assigned to z, because of the absence of ';'
    };

    println!("x is {:#?}", x);
    println!("y is {:#?}", y);
    println!("z is {:#?}", z);
}
