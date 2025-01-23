fn main(){
    // Integer addtion
    assert!(1_u32 + 2_u32 == 3_u32);

    // Integer subtraction
    assert!(1_i32 - 2_i32 == -1_i32);
    assert!(1_i8 - 2_i8 == -1_i8);

    // Integer multiplication
    assert!(3_u8 * 50_u8 == 150_u8);

    // Integer Division and Modulus
    assert!(9.6_f32 / 3.2_f32 == 3.0_f32);
    assert!(24_u8 % 5_u8 == 4_u8);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise Operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1_u32 << 5); // my guess... 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
