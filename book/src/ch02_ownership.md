# 소유권

Rust에서 메모리 안전성을 보장하는 핵심 개념이다. GC 없이도 메모리 누수나 dangling pointer가 발생하지 않는 이유가 소유권 시스템 때문이다.

## 소유권 3가지 규칙

1. 모든 값은 **소유자(owner)**가 있다.
2. 소유자는 **동시에 하나**만 존재한다.
3. 소유자가 **스코프를 벗어나면** 값이 drop된다.

## Move: 소유권 이전

힙에 할당되는 타입(`String` 등)은 대입하면 소유권이 **이전(move)**된다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1의 소유권이 s2로 이전

    // println!("{s1}"); // 컴파일 에러: value borrowed here after move
    println!("{s2}"); // OK
}
```

함수에 넘겨도 마찬가지다.

```rust
fn takes_ownership(s: String) {
    println!("{s}");
} // 여기서 s drop

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}"); // 에러: s는 이미 이전됨
}
```

## Copy: 스택 타입은 복사

`i32`, `f64`, `bool`, `char` 같은 스칼라 타입은 `Copy` 트레이트가 구현되어 있어서
대입해도 소유권 이전 없이 **값이 복사**된다.

```rust
fn main() {
    let x = 5;
    let y = x;  // Copy: x도 y도 사용 가능
    println!("{x}, {y}");
}
```

## Clone: 명시적 깊은 복사

힙 데이터를 진짜로 복사하고 싶으면 `.clone()`을 호출한다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 힙 데이터 복사
    println!("{s1}, {s2}"); // 둘 다 유효
}
```

## 소유권 반환

함수가 값을 반환하면 소유권이 호출자에게 돌아온다.

```rust
fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s // s를 반환 → 소유권 이전
}

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}");
}
```

매번 값을 넘기고 돌려받는 건 번거롭다. 이를 해결하는 게 **참조(reference)**다.

> 전체 예제: [`examples/ch02_ownership.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch02_ownership.rs)
>
> ```bash
> cargo run --example ch02_ownership
> ```
