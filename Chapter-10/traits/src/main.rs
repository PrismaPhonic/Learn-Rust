extern crate traits;

use traits::Tweet;
use traits::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("Britney_Spears"),
        content: String::from("I love cheese and pickles!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
