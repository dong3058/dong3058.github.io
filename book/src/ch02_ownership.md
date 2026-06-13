# 소유권

Rust에서 메모리 안전성을 보장하는 핵심 개념이다. GC 없이도 메모리 누수나 dangling pointer가 발생하지 않는 이유가 소유권 시스템 때문이다.

## 소유권 3가지 규칙

1. 모든 값은 **소유자(owner)**가 있다.
2. 소유자는 **동시에 하나**만 존재한다.
3. 소유자가 **스코프를 벗어나면** 값이 drop된다.-->drop은 즉 메모리에서 제거됨을 의미한다.
스택의 경우에는 차례대로 pop을 하면되다 그외의 복잡한 자료형은 진짜 데이터는 heap에 저장된다.
이때 이 데이터를 해제 하기위해선 drop트레잇이 구현되어있어야한다.
어쩃든 스코프를 벗어나면 stack,heap에걸친 메모리가 모두해제된다.

## 힙과 스택

메모리에 저장되는 데이터들은 스택과 힙이라는 2가지 갈래로 구분된다.
stack은 선입 후출의 구조를,heap은 좀더 복잡한구조를가진다.

stack의 경우 간단한 데이터들 위주로 돌아가며 stack이라는 한공간에 들어가 맨위부터 순서대로뺴면되는지라 빠르다.
크기가 예측 가능한 데이터값 그자체 라던가, 혹은 heap메모리에 저장되는 자바식으로 표현하면 객체 타입의 데이터의 메타데이터(메모리주소 같은것들)이 저장된다.

여기서 이전과 복사의 차이가 발생한다.

힙에 할당된 타입을 다른 변수에 대입시에 러스트는 기본적으로 힙에 저장된 데이터를 완전한 복사본을 만드는게 아니라
새변수에다가 메타 데이터를 심어버린다.-->즉 소유권의 이전.
그러면 기존의 변수는 해당 힙메모리에 저장된변수에 대한 소유권을 잃어버린다.

## Move: 소유권 이전

힙에 할당되는 타입(`String` 등)은 대입하면 소유권이 **이전(move)**된다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1의 소유권이 s2로 이전

    // println!("{s1}"); // 컴파일 에러: value borrowed here after move
    println!("{s2}"); // OK
}
```

함수에 넘겨도 마찬가지다.

```rust
fn takes_ownership(s: String) {
    println!("{s}");
} // 여기서 s drop

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}"); // 에러: s는 이미 이전됨
}
```

## Copy: 스택 타입은 복사

`i32`, `f64`, `bool`, `char` 같은 스칼라 타입은 `Copy` 트레이트가 구현되어 있어서
대입해도 소유권 이전 없이 **값이 복사**된다.-->즉 메모리에 같은값이 새로이할당된 새변수가 탄생한다.
즉 copy 트레잇이 구현된경우에는  값이 복사 되는현상이 발생함.
drop트레잇이 구현된 케이스에는 copy를 붙이지못함.
간단히 생각해서 변수 크기가 고정되서 컴파일 타임에 크기가 예측이 가능한 케이스에는 copy트레잇이 붙어있고
복사 현상이 가능함.-->왜냐면 컴파일 타임에 크기가 예측가능하면 stack에 직접 변수값을 넣을수있기때문.


```rust
fn main() {
    let x = 5;
    let y = x;  // Copy: x도 y도 사용 가능
    println!("{x}, {y}");
}
```

## Clone: 명시적 깊은 복사

힙 데이터를 진짜로 복사하고 싶으면 `.clone()`을 호출한다.--->즉 애는 아예 값자체를 새로이 힙메모리에 할당 및 스택 공간에 해당 힙메모리 변수의 메타데이터를 부여한다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 힙 데이터 복사
    println!("{s1}, {s2}"); // 둘 다 유효
}
```

## 소유권 반환

함수가 값을 반환하면 소유권이 호출자에게 돌아온다.

```rust
fn gives_ownership() -> String {
    String::from("hello")
}
//함수의 파라미터에다가 힙메모리 데이터를 넘긴다는건-->바로 소유권을 이전한다는 의미이다.
fn takes_and_gives_back(s: String) -> String {
    s // s를 반환 → 소유권 이전
}

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}");
}
```

매번 값을 넘기고 돌려받는 건 번거롭다. 이를 해결하는 게 **참조(reference)**다.

> 전체 예제: [`examples/ch02_ownership.rs`](https://github.com/dong3058/dong3058.github.io/blob/main/examples/ch02_ownership.rs)
>
> ```bash
> cargo run --example ch02_ownership
> ```
