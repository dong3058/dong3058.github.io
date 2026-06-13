# 라이프타임

참조가 유효한 범위를 컴파일러에게 알려주는 어노테이션이다. 라이프타임은 값을 만들지 않고, 단지 관계를 설명한다.

## 왜 필요한가

여러 참조를 받아 그 중 하나를 반환할 때, 컴파일러는 반환된 참조가 어느 입력에서 왔는지 알아야 한다.

```rust,compile_fail
// 컴파일 에러: 라이프타임 명시 필요
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

## 라이프타임 어노테이션

`'a`처럼 작은따옴표로 시작하는 이름을 붙인다. 단순히 "반환값의 라이프타임은 x, y 중 짧은 쪽과 같다"는 계약이다.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xy");
        result = longest(s1.as_str(), s2.as_str());
        println!("{result}"); // OK: s2가 아직 살아있음
    }
}
```

## 구조체에서 라이프타임

구조체가 참조를 필드로 가지면 라이프타임을 명시해야 한다.
"구조체 인스턴스는 참조보다 오래 살 수 없다"는 제약이다.

```rust
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let novel = String::from("한 문장. 두 번째 문장.");
    let first = novel.split('.').next().unwrap();
    let excerpt = Excerpt { text: first };
    println!("{}", excerpt.text);
}
```

## 라이프타임 생략 규칙

간단한 경우엔 컴파일러가 라이프타임을 추론(elision)한다.

```rust
// 명시 안 해도 되는 경우들:
fn first_word(s: &str) -> &str {  // 입력 하나 → 출력도 같은 라이프타임
    s.split_whitespace().next().unwrap_or("")
}

impl Excerpt<'_> {
    fn announce(&self, msg: &str) -> &str {  // &self가 있으면 반환은 self 라이프타임
        self.text
    }
}
```

## `'static`

프로그램 전체 기간 동안 유효한 라이프타임이다. 문자열 리터럴은 모두 `'static`이다.

```rust
fn main() {
    let s: &'static str = "이 문자열은 바이너리에 저장됨";
}
```

> 전체 예제: [`examples/ch09_lifetimes.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch09_lifetimes.rs)
>
> ```bash
> cargo run --example ch09_lifetimes
> ```
