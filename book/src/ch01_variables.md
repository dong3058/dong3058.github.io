# 변수와 타입

## 불변 변수와 가변 변수

Rust의 변수는 기본적으로 **불변(immutable)**이다.
값을 바꾸려면 `mut`을 명시해야 한다.

```rust
fn main() {
    let x = 5;
    // x = 6; // 컴파일 에러: cannot assign twice to immutable variable

    let mut y = 5;
    y = 6; // OK
    println!("y = {y}");
}
```

## 기본 스칼라 타입

| 타입 | 예시 | 설명 |
|------|------|------|
| `i32` | `-1`, `0`, `42` | 32비트 정수 (기본값) |
| `u64` | `0`, `255` | 64비트 부호 없는 정수 |
| `f64` | `3.14` | 64비트 부동소수점 (기본값) |
| `bool` | `true`, `false` | 불리언 |
| `char` | `'a'`, `'한'` | 유니코드 문자 (4바이트) |

```rust
fn main() {
    let n: i32 = -10;
    let f: f64 = 3.14;
    let b: bool = true;
    let c: char = '한';

    println!("{n}, {f}, {b}, {c}");
}
```

## 타입 추론

타입을 명시하지 않아도 컴파일러가 추론한다. 단, 모호한 경우엔 명시해야 한다.

```rust
fn main() {
    let x = 42;       // i32로 추론
    let y = 3.14;     // f64로 추론

    // 파싱할 때는 타입 명시 필요
    let parsed: i32 = "42".parse().unwrap();
    println!("{parsed}");
}
```

## Shadowing

같은 이름으로 `let`을 다시 선언하면 이전 값을 가린다(shadow).
`mut`와 다르게 타입을 바꿀 수도 있다.

```rust
fn main() {
    let x = 5;
    let x = x * 2;    // 이전 x를 가림
    let x = x + 1;
    println!("x = {x}"); // 11

    // 타입 변환에 유용
    let spaces = "   ";        // &str
    let spaces = spaces.len(); // usize
    println!("spaces = {spaces}");
}
```

## 상수

`const`는 타입을 반드시 명시해야 하고, 런타임 값을 담을 수 없다.

```rust
const MAX_SCORE: u32 = 100_000;

fn main() {
    println!("MAX_SCORE = {MAX_SCORE}");
}
```

> 전체 예제: [`examples/ch01_variables.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch01_variables.rs)
>
> ```bash
> cargo run --example ch01_variables
> ```
