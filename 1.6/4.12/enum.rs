enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_color_change(msg: Message) {
    let Message::ChangeColor(r, g, b) = msg;
}

fn main() {
    let x: Message = Message::Move { x: 3, y: 4 };
    let x: Message = Message::ChangeColor(2, 2, 2);
    process_color_change(x);

    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
}
