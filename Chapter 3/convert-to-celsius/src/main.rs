use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit:");
    let mut f = String::new();

    io::stdin().read_line(&mut f)
        .expect("Failed to read input!");

    let f: f64 = f.trim().parse()
        .expect("Please type a number!");

    let c = f_to_c(f);
    println!("If it's {0} fahrenheit then it's {1:.2} celsius!", f, c);

    println!("Now enter a temperature in Celsius:");
    let mut c = String::new();

    io::stdin().read_line(&mut c)
        .expect("Failed to read input!");

    let c: f64 = c.trim().parse()
        .expect("Please type a number!");

    let f = c_to_f(c);
    println!("If it's {0} celsius then it's {1:.2} fahrenheit!", c, f);
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.00) * (5.00/9.00)
}

fn c_to_f(c: f64) -> f64 {
    (c * (9.0/5.0)) + 32.0
}
