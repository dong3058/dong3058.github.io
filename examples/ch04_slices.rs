fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    // 문자열 슬라이스
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello}, {world}");

    // &str 파라미터: String과 &str 모두 받음
    println!("{}", first_word(&s));
    println!("{}", first_word("hi there"));

    // 배열 슬라이스
    let arr = [1, 2, 3, 4, 5];
    println!("전체 합: {}", sum(&arr));
    println!("부분 합: {}", sum(&arr[1..4]));
}
