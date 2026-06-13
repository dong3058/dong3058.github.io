# 에러 처리

Rust는 에러를 두 종류로 나눈다: **복구 불가능한 에러**(`panic!`)와 **복구 가능한 에러**(`Result<T, E>`).

## panic!

버그나 프로그래밍 오류처럼 복구할 수 없는 상황에 사용한다. 프로그램이 종료된다.

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[10]; // index out of bounds → panic
}
```

## Result\<T, E\>

외부 요인(파일, 네트워크 등)으로 인한 에러는 `Result`로 처리한다.

```rust
use std::fs;

fn main() {
    let result = fs::read_to_string("hello.txt");

    match result {
        Ok(content) => println!("{content}"),
        Err(e) => println!("에러: {e}"),
    }
}
```

## `?` 연산자

에러를 호출자에게 전파할 때 `match` 대신 쓴다. `Ok`면 값을 꺼내고, `Err`면 즉시 반환한다.

```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?; // Err이면 여기서 반환
    Ok(content.to_uppercase())
}

fn main() {
    match read_file("hello.txt") {
        Ok(s) => println!("{s}"),
        Err(e) => println!("실패: {e}"),
    }
}
```

## unwrap과 expect

빠른 프로토타이핑이나 "절대 실패하지 않는다"고 확신할 때만 쓴다. 실패하면 panic이다.

```rust
fn main() {
    // unwrap: 에러 시 기본 panic 메시지
    let s: i32 = "42".parse().unwrap();

    // expect: 에러 시 커스텀 메시지 (디버깅에 더 유용)
    let s: i32 = "42".parse().expect("숫자 파싱 실패");

    println!("{s}");
}
```

## 에러 타입 변환

`?`를 쓸 때 에러 타입이 다르면 `From` 트레이트로 변환되거나, `Box<dyn Error>`로 통일할 수 있다.

```rust
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

fn parse_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?; // io::Error → Box<dyn Error>
    let n: i32 = content.trim().parse()?;    // ParseIntError → Box<dyn Error>
    Ok(n * 2)
}
```

> 전체 예제: [`examples/ch07_error_handling.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch07_error_handling.rs)
>
> ```bash
> cargo run --example ch07_error_handling
> ```
