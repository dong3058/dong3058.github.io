# 클로저와 이터레이터

## 클로저

환경을 캡처하는 익명 함수다. `|파라미터| 표현식` 형태로 작성한다.

```rust
fn main() {
    let multiplier = 3;

    // 외부 변수 multiplier를 캡처
    let multiply = |x| x * multiplier;

    println!("{}", multiply(5)); // 15
}
```

## 클로저 트레이트

클로저가 환경을 어떻게 캡처하느냐에 따라 트레이트가 결정된다.

| 트레이트 | 캡처 방식 | 설명 |
|----------|-----------|------|
| `FnOnce` | 소유권 이전 | 한 번만 호출 가능 |
| `FnMut` | 가변 참조 | 여러 번 호출 가능, 환경 변경 |
| `Fn` | 불변 참조 | 여러 번 호출 가능 |

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    let offset = 10;
    let result = apply(|x| x + offset, 5);
    println!("{result}"); // 15
}
```

`move`로 소유권을 클로저 안으로 이전할 수 있다. 스레드에 클로저를 넘길 때 주로 쓴다.

```rust
fn main() {
    let s = String::from("hello");
    let contains = move |c: char| s.contains(c); // s의 소유권 이전
    println!("{}", contains('e'));
}
```

## Iterator

`Iterator` 트레이트는 `next()` 메서드 하나를 요구한다.

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(3)
    println!("{:?}", iter.next()); // None
}
```

## 이터레이터 어댑터

체이닝으로 데이터를 변환한다. **지연 평가**라서 최종 소비자(`collect`, `sum` 등)를 호출하기 전까지 실행되지 않는다.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = v.iter()
        .filter(|&&x| x % 2 == 0)  // 짝수만
        .map(|&x| x * x)            // 제곱
        .collect();

    println!("{result:?}"); // [4, 16, 36]

    // sum, fold
    let sum: i32 = v.iter().sum();
    let product: i32 = v.iter().fold(1, |acc, &x| acc * x);
    println!("합: {sum}, 곱: {product}");
}
```

## 커스텀 이터레이터

```rust
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
    let sum: u32 = Counter::new(5).sum();
    println!("1~5 합: {sum}"); // 15

    // zip으로 두 이터레이터 묶기
    let pairs: Vec<_> = Counter::new(3).zip(Counter::new(3).skip(1)).collect();
    println!("{pairs:?}");
}
```

> 전체 예제: [`examples/ch10_closures_iterators.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch10_closures_iterators.rs)
>
> ```bash
> cargo run --example ch10_closures_iterators
> ```
