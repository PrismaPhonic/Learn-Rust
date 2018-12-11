use std::io;

fn main() {
    println!("This tool will generate the nth Fibonacci number for you");
    println!("Enter your choosen n:");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Couldn't read line!");

    let n: u64 = n.trim().parse()
        .expect("Please enter a number!");

    let fibonacci = get_nth_fibonacci(n);
    println!("Here's your fibonacci number: {}", fibonacci);
}

fn get_nth_fibonacci(n: u64) -> u64 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }

    let mut current = 1;
    let mut last = 0;
    for _num in 1..n {
        let temp = current;
        current = current + last;
        last = temp;
    }
    current
}
