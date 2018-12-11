fn main() {
    // another_function(5, 6);
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {}", y);
    let num = 2;
    let prod = five_times(num);

    println!("{} times five is: {}", num, prod);
}

fn five_times(num: i32) -> i32 {
    num * 5
}

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }
