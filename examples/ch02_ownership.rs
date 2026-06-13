fn takes_ownership(s: String) {
    println!("took: {s}");
}

fn gives_ownership() -> String {
    String::from("from function")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn main() {
    // Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved
    println!("s2={s2}");

    // Copy (i32는 Copy 트레이트)
    let x = 5;
    let y = x;
    println!("x={x}, y={y}");

    // Clone
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3={s3}, s4={s4}");

    // 함수로 소유권 이전
    takes_ownership(String::from("moved"));

    // 함수에서 소유권 반환
    let s5 = gives_ownership();
    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s6);
    println!("s5={s5}, s7={s7}");
}
