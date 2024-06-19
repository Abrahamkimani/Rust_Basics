//primitive str = immutable fixed-length string somwhere in memory
//string = Growable, heap-allocated data structure -Use when you need to modify or own

pub fn run(){

    let mut hello = String::from("Hello ");

    //get length
    println!("Length: {}", hello.len());

    //push a char
    hello.push('W');

    //push a string
    hello.push_str("orld");

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //check if empty
    println!("Is Empty: {}", hello.is_empty());

    //contains
    let mut s = String::with_capacity(10);
    println!("contains 'World' {}", hello.contains("World"));


    s.push('b');    assert_eq!(1,s.len());
    println!("{}", hello);

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace

    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}",s.len());

    //Assertions testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    


    println!("{}",s);


}
