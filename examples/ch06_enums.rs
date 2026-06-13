#![allow(dead_code)]

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("종료"),
        Message::Move { x, y } => println!("이동: ({x}, {y})"),
        Message::Write(text) => println!("출력: {text}"),
        Message::ChangeColor(r, g, b) => println!("색상: rgb({r},{g},{b})"),
    }
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    // enum match
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(255, 128, 0));

    // Option
    match divide(10.0, 2.0) {
        Some(r) => println!("10/2 = {r}"),
        None => println!("나눌 수 없음"),
    }
    let r = divide(10.0, 0.0).unwrap_or(f64::INFINITY);
    println!("10/0 = {r}");

    // if let
    let val: Option<i32> = Some(42);
    if let Some(n) = val {
        println!("값: {n}");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // match with range and guard
    let n = 15u32;
    let desc = match n {
        1 => "하나",
        2 | 3 => "둘 또는 셋",
        4..=9 => "넷에서 아홉",
        x if x % 2 == 0 => "10 이상 짝수",
        _ => "10 이상 홀수",
    };
    println!("{n}: {desc}");
}
