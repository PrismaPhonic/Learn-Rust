use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T, V, Y>
    where T: Fn(V) -> Y,
          V: Eq + Copy + Hash,
          Y: Eq + Copy
{
    calculation: T,
    value: HashMap<V, Y>,
}

impl<T, V, Y> Cacher<T, V, Y>
    where T: Fn(V) -> Y,
          V: Eq + Copy + Hash,
          Y: Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, V, Y> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // had to dereference with * so hashmap get returns
    // a copy of the int rather than a reference
    fn value(&mut self, arg: V) -> Y {
        let arg = arg.clone();
        let result = if self.value.contains_key(&arg) {
            *self.value.get(&arg).unwrap()
        } else {
            let v: Y = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        };
        result
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num * 2
    });
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// main calls generate_workout function with simulated
// values - would come from phone app in reality
fn main() {
    let simulated_user_specified_value = 27;
    let simulated_random_number = 5;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_varying_types() {
        let mut c = Cacher::new(|a: &str| -> usize {a.len()});
        let mut c2 = Cacher::new(|a: char| -> usize {a.len_utf8()});

        let v1 = c.value("yes");
        let v2 = c2.value('A');

        assert_eq!(v1, 3);

        // assert 'A' char is 1 byte in size
        assert_eq!(v2, 1);
    }
}
