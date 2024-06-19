//Vectors  -They are resizable arrays

use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4];

    //Reassign value

    numbers[2] = 20;

    //add on to vectors
    numbers.push(5);
    numbers.push(20);

    //pop off last value;
    numbers.pop();


    println!("{:?}", numbers);

    //Get single values
    println!("Single value {:?}", numbers[0]);

    //Get vector length
    println!("Vector length: {}", numbers.len());

    //Vector are stack allocated
    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
    
    //loop through vector values
    /*for x in numbers.iter(){
        println!("Number: {}", x);
    } */

    //loop & mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
        
    }

    println!("Numbers Vec {:?}", numbers);


}