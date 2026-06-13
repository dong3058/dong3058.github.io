# 시작하며

Rust를 공부하면서 이해한 내용을 정리한 노트입니다.

## 이 책의 구조

각 챕터는 개념 설명 + 실제로 돌아가는 예제 코드로 구성됩니다.
예제는 [`examples/`](https://github.com/dong3058/dong3058.github.io/tree/main/examples) 디렉토리에 있고,
로컬에서 직접 실행할 수 있습니다.

```bash
cargo run --example ch01_variables
```

코드 블록의 **▶ Run** 버튼을 누르면 [Rust Playground](https://play.rust-lang.org)에서 바로 실행됩니다.

## 환경 설정

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 버전 확인
rustc --version
cargo --version
```

## 참고 자료

- [The Rust Programming Language](https://doc.rust-lang.org/book/) — 공식 책
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — 연습 문제
