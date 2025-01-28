fn main(){
    #[derive(Debug)]
    struct Person{
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person { name: String::from("David"), age: Box::new(21) };

 let Person { ref name, ref age } = person; // `ref` borrows (immutably references) the fields, 
                                            // avoiding moves and keeping `person` valid.   println!("{}, {}", name, age);
    print!("{:?}", person);
}
