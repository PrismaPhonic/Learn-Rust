use std::collections::HashMap;

fn main() {
    let alice = "Either the well was very deep, or she fell very slowly, for she had plenty of time as she went down to look about her and to wonder what was going to happen next.";

    let mut map = HashMap::new();

    for word in alice.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }

    println!("{:#?}", map);
}
