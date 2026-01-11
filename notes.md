** 1. 'rustc' 명령어로 컴파일 compile
** rustc main.rs

** 2. ./main 명령어로 실행
** ./main

** Hello, world!
** 컴파일과 실행이 별개인 대신 여러분의 프로그램을 컴파일하여 만든 실행 파일을 러스트가 설치되지 않은 곳에서도 실행할 수 있다.


** cargo new로 새 프로젝트를 생성할 수 있습니다.

** cargo build 명령으로 프로젝트를 빌드build할 수 있습니다.

** cargo run 명령어는 한 번에 프로젝트를 빌드하고 실행할 수 있습니다. - cargo build 실행 후 바이너리 경로를 입력해서 실행하는 것보다 편리

** cargo check 명령으로 바이너리를 생성하지 않고 프로젝트의 에러를 체크할 수 있습니다. - 코드를 작성하는 동안 여러분의 프로젝트가 컴파일되는지 지속적으로 검

** 빌드로 만들어진 파일은 작성한 소스 코드와 뒤섞이지 않도록 target/debug 디렉터리에 저장됩니다.

** release build(cargo build --release)
프로젝트를 완성해서 배포(릴리즈)할 준비가 끝났다면, cargo build --release 명령어를 사용해 릴리즈 빌드를 생성할 수 있습니다. 일반 빌드와 차이점은 target/debug 가 아닌 target/release 에 실행 파일이 생성된다는 점, 그리고 컴파일 시 최적화를 진행하여 컴파일이 오래 걸리는 대신 러스트 코드가 더 빠르게 작동하는 점입니다. 릴리즈 빌드가 더 빠르게 작동한다면, 왜 일반 빌드시에는 최적화를 진행하지 않을까요? 이에 대한 해답은 빌드가 두 종류로 나뉘게 된 이유이기도 한데, 개발 중에는 빌드가 잦으며 작업의 흐름을 끊지 않기 위해 빌드 속도 또한 빠를수록 좋지만, 배포용 프로그램은 잦은 빌드가 필요 없으며 빌드 속도보단 프로그램의 작동 속도가 더 중요하기 때문입니다. 이와 같은 이유로, 작성한 코드 작동 속도를 벤치마킹할 시에는 릴리즈 빌드를 기준으로 해야 한다는 것도 알아두시기 바랍니다.

** variable vs constant

fn main() {
    let mut x = 5; // variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant

    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
}

// variable(let, basically immutable, 'mut' can be used for assigning new value)
// constant(const, completely immutable, write with upper letters)


** shadowing


fn shawdoing() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// The value of x in the inner scope is: 12
// The value of x is: 6

** shadowing VS mut 

섀도잉은 변수를 mut로 표시하는 것과는 다릅니다. 실수로 let 키워드 없이 변수에 값을 재할당하려고 한다면 컴파일 타임 에러가 발생하기 때문입니다. let을 사용하면, 값을 변형하면서 변형이 완료된 후에는 불변으로 유지할 수 있습니다.

mut과 섀도잉의 또 다른 차이점은 다시금 let 키워드를 사용하여 새로운 변수를 만드는 것이기 때문에 같은 변수명으로 다른 타입의 값을 저장할 수 있다는 것입니다. 예를 들어, 프로그램이 사용자에게 어떤 텍스트 사이에 몇 개의 공백을 넣고 싶은지 공백문자를 입력하도록 요청하고, 이 값을 숫자로 저장하고 싶다 칩시다:

    let spaces = "   ";
    let spaces = spaces.len();

    첫 번째 spaces는 문자열 타입이고 두 번째 spaces는 숫자 타입입니다. 따라서 섀도잉은 spaces_str과 spaces_num 같이 구분되는 변수명을 쓸 필요가 없도록 여유를 줍니다; 즉, 더 간단한 spaces라는 이름을 재사용할 수 있게 해 줍니다. 그런데 여기에서 mut을 사용하려 한다면, 보시다시피 컴파일 타임 에러가 발생합니다:
    
    let mut spaces = "   ";
    spaces = spaces.len();

    $ cargo run
    Compiling variables v0.1.0 (file:///projects/variables)
    error[E0308]: mismatched types
    --> src/main.rs:3:14
    |
    2 |     let mut spaces = "   ";
    |                      ----- expected due to this value
    3 |     spaces = spaces.len();
    |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `variables` due to previous error


간단한 차이 요약:
mut: 같은 변수의 값을 "변경"합니다. (예: x = 5; x = 10; – x의 값이 업데이트됨)
Shadowing: 같은 이름으로 "새로운 변수"를 만듭니다. (예: let x = 5; let x = 10; – 두 번째 x가 첫 번째 x를 가림)
Rust에서 이 둘을 선택할 때는 "변경의 의도"와 "불변성 유지"를 고려합니다. 
불변성을 최대한 유지하는 것이 Rust의 철학이므로, shadowing을 선호하는 경우가 많습니다.
학습/실무 팁: 초보 시 mut를 먼저 배우지만, 실무에서 shadowing이 더 자주 쓰임 
(예: 웹 서버에서 요청 처리). 잘못 사용 시 러ntime 패닉보단 컴파일 에러로 잡힘.

** str을 인자로 쓸때 &를 붙이는 이유(&str)
string: &str에서 &를 붙이는 이유는 함수가 문자열의 소유권을 가져오지 않고, 빌려오기(borrow)만 하기 위함입니다.
즉, 함수를 호출한 쪽(caller)이 여전히 문자열을 소유한 채로 유지하고, 함수 안에서는 그 문자열을 잠깐 읽기만 합니다.

```rust
let s = String::from("hello");  // s가 소유권을 가짐
let slice = &s;                 // slice는 s를 빌려서 가리킴 (&str)
```
만약 &를 빼고 string: str이라고 썼다면? 
- 컴파일 오류가 납니다.
- 이유: str 타입은 크기가 고정되어 있지 않은 타입이라서 함수 인자로 직접 받을 수 없습니다.
- str은 항상 &str 형태로만 사용됩니다. (Rust 공식 문서에서도 "str is almost always used as &str"라고 명시)

*** '&' 붙일때와 안붙일떄
```rust
fn take_ownership(x: i32, string: String) {
    println!("string: {}", string);
}  // 함수가 끝나면서 string이 drop됨 (메모리 해제)

fn main() {
    let s = String::from("hello");
    take_ownership(42, s);
    println!("{}", s);  // <- 이 println!에서 에러 발생. take_ownership s의 소유권이 함수로 넘어갔음
}
```