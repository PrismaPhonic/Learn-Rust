enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
	// tuple like enum variant
    let msg = Message::ChangeColor(0, 160, 255);

    // // struct like enum variant
    // let msg = Message::Move{x: 5, y: 7};
    
    // // string like enum variant
    // let msg = Message::Write("Written Message".to_string());

    // // nothing to destructure with this variant
    // let msg = Message::Quit;

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}
