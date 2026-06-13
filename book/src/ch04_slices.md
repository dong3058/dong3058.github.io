# 슬라이스

소유권 없이 컬렉션의 **연속된 일부**를 참조한다. 가장 흔한 슬라이스는 `&str`(문자열 슬라이스)와 `&[T]`(배열 슬라이스)다.

## 문자열 슬라이스 `&str`

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];  // "hello"
    let world = &s[6..11]; // "world"
    println!("{hello}, {world}");

    // 처음/끝 생략 가능
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello}, {world}");
}
```

문자열 리터럴은 이미 `&str`이다.

```rust
fn main() {
    let s: &str = "hello"; // 바이너리에 저장된 슬라이스
}
```

## 슬라이스를 인자로 받기

`&String` 대신 `&str`을 받으면 `String`과 `&str` 모두 받을 수 있어서 더 유연하다.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let s = String::from("hello world");
    println!("{}", first_word(&s));      // String → 자동 deref
    println!("{}", first_word("hi there")); // &str 직접 전달
}
```

## 배열 슬라이스 `&[T]`

```rust
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("전체 합: {}", sum(&arr));
    println!("부분 합: {}", sum(&arr[1..4])); // [2, 3, 4]
}
```

> 전체 예제: [`examples/ch04_slices.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch04_slices.rs)
>
> ```bash
> cargo run --example ch04_slices
> ```
