fn length(s: &String) -> usize {
    s.len()
}

fn append(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    // 불변 참조
    let s = String::from("hello");
    let len = length(&s);
    println!("{s}의 길이: {len}");

    // 가변 참조
    let mut s2 = String::from("hello");
    append(&mut s2);
    println!("{s2}");

    // 불변 참조 여러 개 OK
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    // NLL: r1, r2 사용 끝난 후 가변 참조 가능
    let r3 = &mut s2;
    println!("{r3}");
}
