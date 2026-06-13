# 참조와 빌림

소유권을 넘기지 않고 값을 사용하는 방법이다. `&`로 **참조**를 만들고, 참조를 받는 걸 **빌림(borrowing)**이라고 한다.

## 불변 참조

```rust
fn length(s: &String) -> usize {
    s.len()
} // s는 참조이므로 drop되지 않음

fn main() {
    let s = String::from("hello");
    let len = length(&s); // 빌려줌 (소유권 이전 없음)
    println!("{s}의 길이: {len}"); // s 여전히 유효
}
```

## 가변 참조

값을 수정하려면 `&mut`을 사용한다.

```rust
fn append(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    append(&mut s);
    println!("{s}"); // hello, world
}
```

## 빌림 규칙

컴파일러가 강제하는 규칙이다.

1. **불변 참조는 여러 개** 동시에 가능
2. **가변 참조는 한 번에 하나만** 허용
3. 불변 참조가 살아있는 동안 **가변 참조 불가**

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}"); // OK: 불변 참조 둘

    // r1, r2는 이 시점 이후 사용 안 함 (NLL: Non-Lexical Lifetime)

    let r3 = &mut s; // OK: 불변 참조들이 이미 끝남
    println!("{r3}");
}
```

```rust,compile_fail
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s; // 컴파일 에러: r1이 아직 살아있음
    println!("{r1}, {r2}");
}
```

## Dangling Reference 방지

Rust 컴파일러는 참조가 가리키는 값이 먼저 drop되는 상황을 막아준다.

```rust,compile_fail
fn dangle() -> &String {      // 컴파일 에러
    let s = String::from("hi");
    &s  // s는 여기서 drop되는데 참조를 반환하려 함
}
```

소유권을 직접 반환하면 해결된다.

```rust
fn no_dangle() -> String {
    let s = String::from("hi");
    s  // 소유권 이전
}
```

> 전체 예제: [`examples/ch03_references.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch03_references.rs)
>
> ```bash
> cargo run --example ch03_references
> ```
