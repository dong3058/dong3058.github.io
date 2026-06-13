use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    NegativeNumber(i32),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "파싱 에러: {e}"),
            AppError::NegativeNumber(n) => write!(f, "음수 불가: {n}"),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.parse()?; // ParseIntError → AppError::ParseError (From 자동 변환)
    if n < 0 {
        return Err(AppError::NegativeNumber(n));
    }
    Ok(n as u32)
}

fn main() {
    // unwrap / expect (절대 실패하지 않는다고 확신할 때만)
    let n: i32 = "42".parse().expect("숫자여야 함");
    println!("parsed: {n}");

    // Result 처리
    let cases = ["10", "-5", "abc", "99"];
    for case in cases {
        match parse_positive(case) {
            Ok(n) => println!("{case} → {n}"),
            Err(e) => println!("{case} → 에러: {e}"),
        }
    }
}
