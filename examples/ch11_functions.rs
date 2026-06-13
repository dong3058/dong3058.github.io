fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}

fn greet(name: &str) {
    println!("안녕, {name}!");
}

fn min_max(list: &[i32]) -> (i32, i32) {
    let mut min = list[0];
    let mut max = list[0];
    for &n in &list[1..] {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

fn double(x: i32) -> i32 { x * 2 }
fn triple(x: i32) -> i32 { x * 3 }

fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    // 기본
    println!("3 + 4 = {}", add(3, 4));

    // 표현식: 블록이 값을 반환
    let y = {
        let x = 3;
        x * 2
    };
    println!("y = {y}");

    // 이른 반환
    println!("|−5| = {}", absolute(-5));

    // 반환값 없음
    greet("동근");

    // 튜플로 여러 값 반환
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&nums);
    println!("min={min}, max={max}");

    // 함수 포인터
    let ops: Vec<fn(i32) -> i32> = vec![double, triple];
    for op in &ops {
        print!("{} ", op(5));
    }
    println!();
    println!("apply double to 4: {}", apply(double, 4));

    // 재귀
    for i in 0..=7 {
        println!("{i}! = {}", factorial(i));
    }
}
