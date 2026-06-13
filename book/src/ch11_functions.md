# 함수

## 기본 선언

`fn` 키워드로 선언하고, 파라미터 타입과 반환 타입을 반드시 명시한다.

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // 세미콜론 없음 = 이 값이 반환값
}

fn main() {
    println!("{}", add(3, 4)); // 7
}
```

## 구문 vs 표현식

Rust에서 가장 중요한 개념 중 하나다.

- **구문(statement)**: 값을 반환하지 않는다. 끝에 `;`가 붙는다.
- **표현식(expression)**: 값을 반환한다. `;`를 붙이면 구문이 된다.

함수도 기본꼴을 생각해보면 일종의 표현식으로생각할수있다.-->반환값이 없으면 VOID,있으면 기타등등을 반환한다고 보면될듯.

```rust
fn main() {
    let y = {
        let x = 3;
        x * 2       // 표현식: 6을 반환 → y = 6
    };
    println!("{y}");
}
```

함수 본문의 마지막 표현식이 자동으로 반환값이 된다. `return`은 중간에 일찍 반환할 때만 쓴다.

```rust
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n;  // 이른 반환
    }
    n  // 마지막 표현식 = 반환값
}
```

## 반환값이 없는 함수

반환 타입을 생략하면 유닛 타입 `()`을 반환한다.--->`()` 는 자바의 `void`로 생각하면될듯. 단 실제값을 가진 실제타입이다.

```rust
fn greet(name: &str) {
    println!("안녕, {name}!");
}  // 암묵적으로 () 반환
```

## 여러 값 반환

튜플로 묶어서 반환한다.

```rust
fn min_max(list: &[i32]) -> (i32, i32) {
    let mut min = list[0];
    let mut max = list[0];
    for &n in &list[1..] {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&nums);  // 구조분해
    println!("min={min}, max={max}");
}
```

## 함수를 값으로: 함수 포인터

함수도 값처럼 변수에 담거나 다른 함수에 넘길 수 있다. 타입은 `fn(파라미터) -> 반환`으로 표기한다.

```rust
fn double(x: i32) -> i32 { x * 2 }
fn triple(x: i32) -> i32 { x * 3 }

fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let ops: Vec<fn(i32) -> i32> = vec![double, triple];
    for op in &ops {
        println!("{}", op(5));
    }

    println!("{}", apply(double, 4)); // 8
}
```

> 클로저(`|x| x * 2`)와 달리 함수 포인터는 환경을 캡처하지 않는다.
> 환경 캡처가 필요하면 [클로저](./ch10_closures_iterators.md)를 쓴다.

## 재귀

```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    for i in 0..=10 {
        println!("{i}! = {}", factorial(i));
    }
}
```

> 전체 예제: [`examples/ch11_functions.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch11_functions.rs)
>
> ```bash
> cargo run --example ch11_functions
> ```
