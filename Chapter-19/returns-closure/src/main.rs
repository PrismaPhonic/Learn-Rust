fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let add_one = returns_closure();

    let answer = add_one(1);

    println!("Here's what you get when you add 1 + 1: {}", answer);
}
