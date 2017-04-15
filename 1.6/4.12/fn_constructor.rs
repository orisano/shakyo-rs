fn main() {
    enum Message {
        Write(String),
    }
    let m = Message::Write("Hello, world".to_string());

    fn foo(x: String) -> Message {
        Message::Write(x)
    }

    let x = foo("Hello, world".to_string());

    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
