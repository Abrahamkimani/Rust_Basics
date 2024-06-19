pub fn run(){
    //print to console
    println!("Hello from print.rs file ");

    //Basic formatting
    println!("{} is from {}", "Brad","Mass");

    //positional arguments
    print!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    //Named aruguments
    print!(" \n {name} likes to play {activity}", name="John", activity="Baseball");

    //Placeholder traits
    println!("\n Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);

}