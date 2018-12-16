extern crate regex;
use regex::Regex;

fn main() {
    let test = "Here is some pig latin";
    let plat = piglatin(test);
    println!("{}", plat);
}

// Found that I had to use String type because you can't reassign an &str in a Vector in rust
// because &str sizes aren't known at the time of compilation, so there's no garauntee you aren't
// occupying more  space than existed in the vector beforehand (that or Rust is just insanely weird
// and I have no idea what's going on)
fn piglatin(str: &str) -> String {
    let mut words = str.split_whitespace().map(|w| String::from(w)).collect::<Vec<String>>();
    let vowel = Regex::new(r"^[aeiouAEIOU]").unwrap();
    for word in &mut words {
        if vowel.is_match(word) {
            word.push_str("hay");
        } else {
            let w: Vec<char> = word.chars().collect();
            let f = w[0];
            let rest: String = w[1..].into_iter().collect();
            *word = format!("{}{}ay", rest, f);
        }
    }
    words.join(" ")
}
