#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0);
    println!("{rect:?}");
    println!("넓이: {}", rect.area());
    println!("둘레: {}", rect.perimeter());
    println!("정사각형?: {}", rect.is_square());

    let small = Rectangle::new(3.0, 2.0);
    println!("rect이 small을 포함?: {}", rect.can_hold(&small));

    // 구조체 업데이트
    let user1 = User {
        name: String::from("홍길동"),
        email: String::from("hong@example.com"),
        age: 30,
        active: true,
    };
    let user2 = User {
        email: String::from("new@example.com"),
        ..user1
    };
    println!("{user2:#?}");
}
