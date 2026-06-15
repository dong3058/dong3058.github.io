# 구조체

관련 데이터를 묶는 사용자 정의 타입이다.

구조체는 기본적으로 stack저장, 만약 내부 데이터중에서 heap이ㅣㅇㅆ으면 개만 heap으로감.

## 구조체 정의와 생성

```rust
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("홍길동"),
        email: String::from("hong@example.com"),
        age: 30,
        active: true,
    };//인스턴스 생성시에 user에는 순서대로 안넣어도된다. 즉 일반적인 자바의 클래스처럼 순서를 지킬필요없음.
    println!("{} ({})", user.name, user.age);
}
```

## 메서드와 연관 함수

`impl` 블록 안에 메서드를 정의한다.
- **메서드**: 첫 번째 파라미터가 `self` — 인스턴스에서 호출 (`user.area()`)
- **연관 함수**: `self` 없음 — 타입으로 호출 (`Rectangle::new(...)`)

구조체+impl은 자바의 class를 데이터와 action으로 분리한느낌.

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 연관 함수 (생성자 역할) Self는 impl의 대상이 되는 타입의 별칭임. 즉 여기선 impl을 rectangle이라는 구조체를 의미함.
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    // 메서드-->&self는 self:&self를 축약한것.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0);
    println!("{rect:?}");
    println!("넓이: {}", rect.area());
    println!("둘레: {}", rect.perimeter());
    println!("정사각형?: {}", rect.is_square());
}
```

## 구조체 업데이트 문법

기존 인스턴스를 기반으로 일부 필드만 바꿀 때 유용하다.

```rust
fn main() {
    let user1 = User {
        name: String::from("홍길동"),
        email: String::from("hong@example.com"),
        age: 30,
        active: true,
    };

    let user2 = User {
        email: String::from("new@example.com"),
        ..user1  // 나머지 필드는 user1에서 복사
    };
    // 주의: user1.name은 move됐으므로 user1.name 사용 불가-->구조체 내부에서 name은 string의힙스택 다입이고ㅓ, 나머지는 간단한 stack타입이기 때문에 name은 쓸수가없다는말.
    println!("{}", user2.name);
}
```

## `#[derive(Debug)]`

`println!("{:?}", ...)` 또는 `{:#?}` (pretty-print)로 출력하려면 `Debug` 트레이트가 필요하다.
`#[derive(Debug)]`로 자동 구현된다.

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("{p:?}");   // Point { x: 1.0, y: 2.5 }
    println!("{p:#?}");  // pretty-print
}
```

> 전체 예제: [`examples/ch05_structs.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch05_structs.rs)
>
> ```bash
> cargo run --example ch05_structs
> ```
