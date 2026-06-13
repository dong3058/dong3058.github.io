# 제네릭과 트레이트

## 제네릭 함수

타입을 파라미터로 받아서 중복 코드를 줄인다.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("최댓값: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("최댓값: {}", largest(&chars));
}
```

## 제네릭 구조체

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 4.5 };
    println!("{p1:?}, {p2:?}");
}
```

## 트레이트 정의

공통 동작을 정의하는 인터페이스다.

```rust
trait Summary {
    fn summarize(&self) -> String;

    // 기본 구현 제공 가능
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..20.min(self.summarize().len())])
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}
```

## 트레이트 바운드

함수 파라미터가 특정 트레이트를 구현해야 함을 제약한다.

```rust
// impl Trait 문법 (간결)
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 제네릭 + where 절 (복잡한 경우 더 명확)
fn notify_generic<T>(item: &T)
where
    T: Summary + std::fmt::Debug,
{
    println!("{item:?}: {}", item.summarize());
}
```

## 트레이트 객체 (동적 디스패치)

컴파일 타임에 타입을 알 수 없을 때 `dyn Trait`을 사용한다.

```rust
fn print_all(items: &[Box<dyn Summary>]) {
    for item in items {
        println!("{}", item.summarize());
    }
}
```

> 전체 예제: [`examples/ch08_generics_traits.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch08_generics_traits.rs)
>
> ```bash
> cargo run --example ch08_generics_traits
> ```
