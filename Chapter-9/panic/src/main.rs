fn main() {
    // simple test to force a panic! from Vec<T>
    let v = vec![1,2,3];

    v[99];
}
