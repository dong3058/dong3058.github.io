fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // 클로저 기본
    let multiplier = 3;
    let multiply = |x| x * multiplier;
    println!("3 * 5 = {}", multiply(5));

    // Fn 트레이트
    let offset = 10;
    println!("apply: {}", apply(|x| x + offset, 5));

    // move 클로저
    let s = String::from("hello");
    let contains_e = move |c: char| s.contains(c);
    println!("'e' 포함?: {}", contains_e('e'));

    // 이터레이터 어댑터
    let v = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = v.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("짝수 제곱: {result:?}");

    let sum: i32 = v.iter().sum();
    let product: i32 = v.iter().fold(1, |acc, &x| acc * x);
    println!("합={sum}, 곱={product}");

    // 커스텀 이터레이터
    let counter_sum: u32 = Counter::new(5).sum();
    println!("1~5 합: {counter_sum}");

    let pairs: Vec<_> = Counter::new(3)
        .zip(Counter::new(3).skip(1))
        .collect();
    println!("pairs: {pairs:?}");

    // flat_map
    let words = vec!["hello world", "foo bar"];
    let letters: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{letters:?}");
}
