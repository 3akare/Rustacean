fn main(){
    let s: String = String::from("Hello world");
    let s: String = print_str(s);

    println!("{}", s);
}

fn print_str(s: String) -> String{
    println!("{}", s);
    s
}
