fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T: (Fn(i32) -> i32)>(f: T, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    let add_two = |num| {
        num + 2
    };

    let answer2 = do_twice(add_two, 5);

    println!("The first answer is: {}", answer);
    println!("The second answer is: {}", answer2);
}
