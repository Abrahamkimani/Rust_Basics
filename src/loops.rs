//Loops -Used to iterate until a condition is met


pub fn run(){
let mut count = 0;

//infinite Loop
/*loop {
    count += 2;
    println!("Number: {}", count);

    if count == 20 {
        break;
    }
}*/

    //while loop (FizzBuzz)
    /* while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 ==0  {
            println!("fizz");
        }else if count % 5 ==0 {
            println!("Buzz");
        }else {
            println!("{}",count);
        }

        //Increment
        count += 1;

    }*/

    //For range
    for x in 0..100{
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 ==0  {
            println!("fizz");
        }else if x % 5 ==0 {
            println!("Buzz");
        }else {
            println!("{}",x);
        }
    }
}