fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Article {
    title: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    body: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("[{}] {}", self.title, &self.content[..self.content.len().min(50)])
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.body)
    }
}

fn notify(item: &impl Summary) {
    println!("속보: {}", item.summarize());
}

fn notify_all(items: &[&dyn Summary]) {
    for item in items {
        println!("{}", item.summarize());
    }
}

fn main() {
    // 제네릭 함수
    let numbers = vec![34, 50, 25, 100, 65];
    println!("최댓값: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("최댓값: {}", largest(&chars));

    // 트레이트
    let article = Article {
        title: String::from("Rust 입문"),
        content: String::from("Rust는 메모리 안전성을 보장하는 언어입니다."),
    };
    let tweet = Tweet {
        username: String::from("dong3058"),
        body: String::from("Rust 공부 중!"),
    };

    notify(&article);
    notify(&tweet);

    // dyn Trait (동적 디스패치)
    let items: Vec<&dyn Summary> = vec![&article, &tweet];
    notify_all(&items);
}
