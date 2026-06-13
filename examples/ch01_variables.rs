const MAX_SCORE: u32 = 100_000;

fn main() {
    // 불변 vs 가변
    let x = 5;
    let mut y = 5;
    y += 1;
    println!("x={x}, y={y}");

    // 기본 타입
    let n: i32 = -10;
    let f: f64 = 3.14;
    let b: bool = true;
    let c: char = '한';
    println!("n={n}, f={f}, b={b}, c={c}");

    // 타입 추론
    let inferred = 42; // i32
    let parsed: i32 = "99".parse().unwrap();
    println!("inferred={inferred}, parsed={parsed}");

    // shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces={spaces}");

    let x = x * 2;
    let x = x + 1;
    println!("x after shadowing={x}");

    // 상수
    println!("MAX_SCORE={MAX_SCORE}");
}
