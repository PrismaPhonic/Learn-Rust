fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third item in the vector is: {}", third);

    let i = 9;

    let does_not_exist = &v[100];
    match v.get(i) {
        Some(_) => { println!("Reachable element at index: {}", i)  },
        None => { println!("Unreachable element at index: {}", i) }
    }
}
