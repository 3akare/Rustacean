fn main(){
    println!("cargo:rustc-link-lib=static=add");
    println!("cargo:rustc-link-search=native=csrc");
}
