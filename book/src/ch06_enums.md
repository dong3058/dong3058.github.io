# 열거형과 패턴 매칭

## 열거형 기본

각 variant가 서로 다른 타입의 데이터를 가질 수 있다.

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 익명 구조체
    Write(String),              // String 하나
    ChangeColor(u8, u8, u8),    // 튜플
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("종료"),
        Message::Move { x, y } => println!("이동: ({x}, {y})"),
        Message::Write(text) => println!("출력: {text}"),
        Message::ChangeColor(r, g, b) => println!("색상: rgb({r},{g},{b})"),
    }
}

fn main() {
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("hello")));
}
```

## Option\<T\>

Rust에는 `null`이 없다. 대신 `Option<T>`를 사용한다.

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Some(result) => println!("결과: {result}"),
        None => println!("0으로 나눌 수 없음"),
    }

    // unwrap_or로 기본값 지정
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("{result}");
}
```

## match

모든 경우를 빠짐없이 처리해야 한다(exhaustive).

```rust
fn main() {
    let n = 3u32;

    let desc = match n {
        1 => "하나",
        2 | 3 => "둘 또는 셋",
        4..=9 => "넷에서 아홉",
        _ => "그 외",  // wildcard
    };
    println!("{desc}");
}
```

## if let: 하나의 패턴만 처리

`match`의 간결한 버전이다. 나머지 경우를 무시할 때 유용하다.

```rust
fn main() {
    let val: Option<i32> = Some(42);

    if let Some(n) = val {
        println!("값: {n}");
    }

    // else도 가능
    if let Some(n) = val {
        println!("{n}");
    } else {
        println!("없음");
    }
}
```

## while let

조건이 패턴에 매칭되는 동안 반복한다.

```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
```

> 전체 예제: [`examples/ch06_enums.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch06_enums.rs)
>
> ```bash
> cargo run --example ch06_enums
> ```
