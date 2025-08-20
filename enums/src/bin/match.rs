// Enum with match
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Exiting..."),
        Message::Write(text) => println!("Message: {}", text),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
    }
}

fn main() {
    process_message(Message::Quit);
    process_message(Message::Write("Hello Rust!".to_string()));
    process_message(Message::Move { x: 10, y: 20 });
}
