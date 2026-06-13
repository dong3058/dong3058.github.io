#![allow(dead_code)]

// 라이프타임 어노테이션: x, y 중 짧은 쪽과 같은 라이프타임을 반환
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce(&self, announcement: &str) -> &str {
        println!("주목: {announcement}");
        self.text
    }
}

// 라이프타임 생략: 입력 하나 → 출력도 같은 라이프타임 (컴파일러가 추론)
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    // longest
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("가장 긴 문자열: {result}");
    }

    // 구조체에서 라이프타임
    let novel = String::from("한 문장. 두 번째 문장.");
    let first = novel.split('.').next().unwrap();
    let excerpt = Excerpt { text: first };
    println!("발췌: {}", excerpt.text);
    println!("레벨: {}", excerpt.level());

    // 생략 규칙
    println!("첫 단어: {}", first_word("hello world"));

    // 'static: 문자열 리터럴
    let s: &'static str = "바이너리에 저장됨";
    println!("{s}");
}
