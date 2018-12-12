// Learning how to Define and Instantiate Structs

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1's email right now is: {}", user1.email);

    user1.email = String::from("newemail@example.com");
    println!("user1's email has been mutated, it is now: {}", user1.email);

    // let user2 = build_user(String::from("peter@peter.com"), String::from("peter"));

    // println!("{}'s email is {} and he has a sign in count of {}", user2.username, user2.email, user2.sign_in_count);
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user1
    };

    println!("user2 has an email of {}", user2.email);

    // Tuple structs are simply tuples that we want to define with a unique type, as all structs
    // create their own type
    struct RGB(u8, u8, u8);
    let black = RGB(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
