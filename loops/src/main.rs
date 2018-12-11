fn main() {
    // loop {
    //     println!("again!");
    // }
    
    // loop construct, must use break to exit
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     println!("{}", counter);
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("Result is: {}", result);
    
    // // while loop, works as expected
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");
    
    // // for loop, use for...in array.iter() syntax
    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // for loop that uses range construct
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
