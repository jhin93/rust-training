# 30일 Rust 마스터 플랜 (2026.01.06 ~ 2026.02.04)

**목표: Meridial AI Training Freelancer Test 초보자 Pass (70%+)**

---

## 사전 준비 (Day 0: 2026년 1월 5일, 2시간)

### Setup Checklist

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 버전 확인
rustc --version
cargo --version

# VSCode 설치 (없으면)
# Extensions 설치:
- rust-analyzer
- Error Lens
- Better TOML

# Rustlings 설치
git clone https://github.com/rust-lang/rustlings.git
cd rustlings
cargo install --force --path .
rustlings watch

# 계정 생성
- Exercism: https://exercism.org/tracks/rust
- LeetCode: https://leetcode.com
```

---

## Phase 1: Rust 기초 (Day 1-10)

### Day 1 (월) - 2026년 1월 6일

**Hour 1 (09:00-10:00): 환경 설정 + Hello World**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch01-00-getting-started.html
   - Ch 1.1: Installation (확인)
   - Ch 1.2: Hello, World! (실습)
   - Ch 1.3: Hello, Cargo!

2. 실습
   mkdir ~/rust-learning
   cd ~/rust-learning
   cargo new hello_world
   cd hello_world
   cargo run
   
3. 수정해보기
   src/main.rs 열어서
   - println! 매크로 수정
   - 여러 줄 출력
   cargo run으로 확인
```

**Hour 2 (10:00-11:00): Variables & Data Types**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
   - Ch 3.1: Variables and Mutability
   - Ch 3.2: Data Types (전체 정독)
   
2. 노트 정리 (VSCode에 notes.md 생성)
   - let vs let mut
   - Shadowing 개념
   - Scalar types: integers, floats, bool, char
   - Compound types: tuples, arrays

3. 실습 프로젝트
   cargo new variables_practice
   src/main.rs에 예제 타이핑 (복붙 금지)
   - 불변 변수
   - 가변 변수
   - Shadowing 예제
```

**Hour 3 (11:00-12:00): Functions**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
   - Ch 3.3: Functions (전체)
   - Statements vs Expressions 개념 이해

2. 실습
   cargo new functions_practice
   src/main.rs에 작성:
   - 파라미터 있는 함수 3개
   - 리턴값 있는 함수 3개
   - Expression 활용 예제

3. 실험
   - 세미콜론 있을 때 vs 없을 때 차이
   - 여러 리턴 타입 시도
```

**Hour 4 (12:00-13:00): Control Flow**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch03-05-control-flow.html
   - Ch 3.5: Control Flow (전체)

2. 실습
   cargo new control_flow
   src/main.rs에 작성:
   - if/else 예제 5개
   - loop 예제 2개
   - while 예제 2개
   - for 예제 3개 (range 사용)

3. 미니 프로젝트
   "Fibonacci 10번째 수 계산" 프로그램 작성
   - for loop 사용
   - 함수로 분리
```

**Day 1 체크리스트:**
```
□ Rust 설치 완료 및 확인
□ The Rust Book Ch 1, 3.1, 3.2, 3.3, 3.5 완독
□ 4개 실습 프로젝트 완성
□ notes.md에 핵심 개념 정리
□ Fibonacci 프로그램 작동 확인
```

---

### Day 2 (화) - 2026년 1월 7일

**Hour 1 (09:00-10:00): Ownership 입문**
```bash
TODO:
1. The Rust Book 읽기 (천천히 3번 읽기)
   https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
   - Ch 4.1: What is Ownership?
   
2. YouTube 보기
   https://www.youtube.com/watch?v=VFIOSWy93H0
   제목: "Rust Ownership Explained"
   채널: Let's Get Rusty
   (10분 영상, 영어 자막 켜기)

3. notes.md에 정리
   - Stack vs Heap
   - Ownership Rules (3가지)
   - Move 개념
   - Clone vs Copy
```

**Hour 2 (10:00-11:00): Ownership 실습**
```bash
TODO:
1. cargo new ownership_practice

2. src/main.rs에 직접 타이핑 (복붙 금지)
   Book의 모든 예제 코드 입력:
   - String 소유권 이동 예제
   - 함수에 값 전달 예제
   - 리턴값으로 소유권 이동

3. 실험
   각 예제를 변형해보기:
   - 소유권 이동 후 원래 변수 사용 → 에러 확인
   - clone() 추가 → 에러 해결
   - 정수형은 Copy → 작동 확인
```

**Hour 3 (11:00-12:00): References & Borrowing**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
   - Ch 4.2: References and Borrowing

2. 핵심 개념 notes.md 정리
   - & (immutable reference)
   - &mut (mutable reference)
   - Borrowing Rules (2가지)
   - Dangling References

3. 실습
   cargo new references_practice
   - 불변 참조 예제 3개
   - 가변 참조 예제 3개
   - 규칙 위반 → 에러 확인
```

**Hour 4 (12:00-13:00): Rustlings 시작**
```bash
TODO:
1. Rustlings 폴더로 이동
   cd ~/rustlings
   rustlings watch

2. 문제 풀기 (순서대로)
   - intro1
   - intro2
   - variables1
   - variables2
   - variables3
   - variables4
   - variables5
   - variables6

3. 틀린 문제
   - 에러 메시지 읽기
   - notes.md에 기록
   - 다시 시도
```

**Day 2 체크리스트:**
```
□ Ownership 개념 이해 (3번 복습)
□ YouTube 영상 시청
□ References 개념 이해
□ Rustlings 8문제 완료
□ notes.md에 Ownership/Borrowing 정리
```

---

### Day 3 (수) - 2026년 1월 8일

**Hour 1 (09:00-10:00): Slice Type**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch04-03-slices.html
   - Ch 4.3: The Slice Type

2. 실습
   cargo new slices_practice
   src/main.rs:
   - String slice 예제 5개
   - Array slice 예제 3개
   - first_word() 함수 구현 (Book 예제)

3. notes.md 정리
   - Slice 문법: &s[0..5]
   - String slice: &str
   - Array slice: &[i32]
```

**Hour 2 (10:00-11:00): Rustlings - Functions & If**
```bash
TODO:
rustlings watch

문제 풀기:
- functions1
- functions2
- functions3
- functions4
- functions5
- if1
- if2
- if3

목표: 8문제 완료
힌트 보지 않고 시도 → 10분 고민 후 힌트 확인
```

**Hour 3 (11:00-12:00): Structs 기초**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch05-01-defining-structs.html
   - Ch 5.1: Defining and Instantiating Structs

2. 실습
   cargo new structs_practice
   
   src/main.rs에 작성:
   - User struct 정의
   - Rectangle struct 정의
   - Tuple struct 2개
   - Unit-like struct 1개

3. 메서드 없이 함수로 구현
   fn area(rect: &Rectangle) -> u32
```

**Hour 4 (12:00-13:00): Structs Method**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch05-03-method-syntax.html
   - Ch 5.3: Method Syntax

2. 실습 (어제 프로젝트 수정)
   cd structs_practice
   
   Rectangle impl 블록 추가:
   - area() method
   - can_hold() method
   - square() associated function

3. notes.md 정리
   - impl 블록
   - &self vs self vs &mut self
   - Associated functions
```

**Day 3 체크리스트:**
```
□ Slice 개념 이해
□ Rustlings 16문제 누적 완료
□ Structs 기초 및 Method 이해
□ impl 블록 작성 연습
```

---

### Day 4 (목) - 2026년 1월 9일

**Hour 1 (09:00-10:00): Enums 기초**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
   - Ch 6.1: Defining an Enum

2. 실습
   cargo new enums_practice
   
   src/main.rs:
   - IpAddrKind enum
   - Message enum (4가지 variant)
   - 각 variant에 데이터 포함

3. notes.md
   - enum vs struct 차이
   - variant에 데이터 넣기
```

**Hour 2 (10:00-11:00): Option & Match**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch06-02-match.html
   - Ch 6.2: The match Control Flow

2. 실습 (같은 프로젝트)
   - Option<T> 예제 3개
   - match로 Option 처리
   - match로 enum 처리
   - _ placeholder 사용

3. 핵심 개념
   - Option::Some / Option::None
   - match는 exhaustive
   - if let 간단한 패턴
```

**Hour 3 (11:00-12:00): Rustlings - Quiz + Primitive Types**
```bash
TODO:
rustlings watch

문제 풀기:
- quiz1 (첫 퀴즈!)
- primitive_types1
- primitive_types2
- primitive_types3
- primitive_types4
- primitive_types5
- primitive_types6

quiz1 못 풀면:
- 앞으로 돌아가서 복습
- 힌트 확인
- 30분 더 투자
```

**Hour 4 (12:00-13:00): 복습 Day**
```bash
TODO:
1. 지난 3일 notes.md 읽기

2. 취약점 재실습
   - Ownership 헷갈리면 → Day 2 다시
   - References 헷갈리면 → 예제 다시
   - Structs/Enums 헷갈리면 → 예제 다시

3. 미니 프로젝트
   "직원 관리 시스템" (간단 버전)
   
   struct Employee {
       name: String,
       department: Department,
       salary: u32,
   }
   
   enum Department {
       Engineering,
       Sales,
       HR,
   }
   
   - 3명의 직원 생성
   - 부서별 급여 합계 계산
   - match로 부서 출력
```

**Day 4 체크리스트:**
```
□ Enums 및 Match 이해
□ Option<T> 사용법 숙지
□ Rustlings 23문제 누적 완료
□ quiz1 통과
□ 미니 프로젝트 완성
```

---

### Day 5 (금) - 2026년 1월 10일

**Hour 1 (09:00-10:00): Vectors**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch08-01-vectors.html
   - Ch 8.1: Storing Lists of Values with Vectors

2. 실습
   cargo new collections_practice
   
   src/main.rs:
   - Vec::new() vs vec! 매크로
   - push() / pop()
   - get() vs []
   - 반복문으로 Vec 순회
   - Vec<T> 타입 명시

3. notes.md
   - Vec 생성 3가지 방법
   - 소유권 이슈 (vec 이동)
   - get()은 Option 반환
```

**Hour 2 (10:00-11:00): Strings**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch08-02-strings.html
   - Ch 8.2: Storing UTF-8 Encoded Text with Strings

2. 실습 (같은 프로젝트)
   - String::new() / String::from()
   - push_str() / push()
   - + 연산자 vs format! 매크로
   - chars() / bytes()

3. 중요 개념 notes.md
   - String vs &str
   - UTF-8 인코딩 (한글도 테스트)
   - 인덱싱 불가 이유
```

**Hour 3 (11:00-12:00): HashMap**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch08-03-hash-maps.html
   - Ch 8.3: Storing Keys with Associated Values in Hash Maps

2. 실습 (같은 프로젝트)
   use std::collections::HashMap;
   
   - insert() / get()
   - entry() / or_insert()
   - 반복문 순회
   - 단어 빈도수 카운팅 예제

3. 미니 챌린지
   "학생 점수 관리"
   - HashMap<String, u32>
   - 5명 학생 점수 입력
   - 평균 계산
   - 최고점 찾기
```

**Hour 4 (12:00-13:00): Rustlings - Vecs + Strings**
```bash
TODO:
rustlings watch

문제 풀기:
- vecs1
- vecs2
- move_semantics1
- move_semantics2
- move_semantics3
- move_semantics4
- move_semantics5
- move_semantics6

특히 move_semantics 집중:
- 소유권 이동 패턴 익히기
- 에러 메시지 읽기 연습
```

**Day 5 체크리스트:**
```
□ Vec, String, HashMap 기본 사용법 숙지
□ Collections 상호 비교 가능
□ Rustlings 31문제 누적 완료
□ 학생 점수 관리 프로그램 완성
```

---

### Day 6 (토) - 2026년 1월 11일

**Hour 1 (09:00-10:00): Error Handling - panic!**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
   - Ch 9.1: Unrecoverable Errors with panic!

2. 실습
   cargo new error_practice
   
   - panic! 매크로 실험
   - RUST_BACKTRACE=1 환경변수
   - 배열 인덱스 오버플로우
   - unwrap() 사용

3. 터미널 실습
   RUST_BACKTRACE=1 cargo run
   RUST_BACKTRACE=full cargo run
```

**Hour 2 (10:00-11:00): Result<T, E>**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
   - Ch 9.2: Recoverable Errors with Result

2. 실습 (같은 프로젝트)
   use std::fs::File;
   use std::io::ErrorKind;
   
   - File::open() Result 처리
   - match로 에러 핸들링
   - unwrap() vs expect()
   - ? 연산자 사용

3. notes.md
   - Result<T, E> 개념
   - Ok() vs Err()
   - 에러 전파 패턴
```

**Hour 3 (11:00-12:00): Rustlings - Structs + Enums**
```bash
TODO:
rustlings watch

문제 풀기:
- structs1
- structs2
- structs3
- enums1
- enums2
- enums3
- strings1
- strings2

목표: 8문제 완료
struct/enum 패턴 체화
```

**Hour 4 (12:00-13:00): 주간 복습 + 미니 프로젝트**
```bash
TODO:
1. Week 1 총정리 (30분)
   Day 1-5 notes.md 읽기
   헷갈리는 개념 리스트업

2. 미니 프로젝트: "To-Do CLI" (30분)
   
   cargo new todo_cli
   
   기능:
   - Vec<String>으로 할 일 저장
   - add_task(task: String)
   - list_tasks()
   - complete_task(index: usize) -> Result<(), String>
   
   실행:
   cargo run
```

**Day 6 체크리스트:**
```
□ panic! vs Result 차이 이해
□ ? 연산자 사용법 숙지
□ Rustlings 39문제 누적 완료
□ Week 1 복습 완료
□ To-Do CLI 작동 확인
```

---

### Day 7 (일) - 2026년 1월 12일 (휴식 + 가벼운 복습)

**Hour 1-2 (10:00-12:00): 자유 복습**
```bash
TODO:
1. 선택 A: 취약한 부분 재학습
   - Ownership 다시 읽기
   - Borrowing 예제 다시
   - References 실습

2. 선택 B: Rust by Example 구경
   https://doc.rust-lang.org/rust-by-example/
   - Hello World
   - Primitives
   - Custom Types
   (재미있는 예제 훑어보기)

3. 선택 C: YouTube 추가 시청
   채널: Let's Get Rusty
   "Rust Crash Course" 시리즈 중
   헷갈리는 부분 찾아보기
```

**Hour 3-4 (14:00-16:00): Rustlings 밀린 문제**
```bash
TODO:
rustlings watch

밀린 문제 몰아서 풀기:
- modules1
- modules2
- hashmaps1
- hashmaps2
- quiz2

목표: 5문제 완료
quiz2 통과 필수
```

**Day 7 체크리스트:**
```
□ 자유롭게 복습
□ 스트레스 없이 진행
□ Rustlings 44문제 누적
□ quiz2 통과
```

---

### Day 8 (월) - 2026년 1월 13일

**Hour 1 (09:00-10:00): Generics**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch10-01-syntax.html
   - Ch 10.1: Generic Data Types

2. 실습
   cargo new generics_practice
   
   src/main.rs:
   - Generic function: largest<T>()
   - Generic struct: Point<T>
   - Generic enum 예제
   - Multiple generic types: Point<T, U>

3. notes.md
   - <T> 문법
   - Monomorphization
   - 성능 이슈 없음
```

**Hour 2 (10:00-11:00): Traits 기초**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch10-02-traits.html
   - Ch 10.2: Traits (Defining Shared Behavior)

2. 실습 (같은 프로젝트)
   - Summary trait 정의
   - impl Summary for NewsArticle
   - impl Summary for Tweet
   - Default implementation

3. 핵심 개념 notes.md
   - trait 정의 문법
   - impl Trait for Type
   - Default implementation
```

**Hour 3 (11:00-12:00): Traits 심화**
```bash
TODO:
1. The Rust Book 읽기 (계속)
   같은 페이지 하단:
   - Traits as Parameters
   - Trait Bounds
   - + 문법
   - where 절

2. 실습 (같은 프로젝트)
   - fn notify(item: &impl Summary)
   - fn notify<T: Summary>(item: &T)
   - fn notify<T: Summary + Display>(item: &T)
   - where 절로 리팩토링

3. 어려운 부분 notes.md 기록
```

**Hour 4 (12:00-13:00): Lifetimes 입문**
```bash
TODO:
1. The Rust Book 읽기 (천천히)
   https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
   - Ch 10.3: Validating References with Lifetimes
   (처음 반만 읽기)

2. 개념 이해만 (실습 적음)
   - 'a 문법
   - Dangling reference 방지
   - Borrow checker 작동 원리

3. YouTube 보기
   https://www.youtube.com/watch?v=1QoT9fmPYr8
   제목: "Rust Lifetimes Explained"
   채널: Let's Get Rusty

4. notes.md
   "Lifetime은 어렵다. 천천히."
   - 기본 개념만 정리
```

**Day 8 체크리스트:**
```
□ Generics 문법 이해
□ Traits 정의 및 구현 가능
□ Trait Bounds 기본 이해
□ Lifetimes 개념 파악 (깊이 X)
```

---

### Day 9 (화) - 2026년 1월 14일

**Hour 1 (09:00-10:00): Lifetimes 실습**
```bash
TODO:
1. The Rust Book 읽기 (나머지)
   Ch 10.3 하단
   - Lifetime Annotations in Function Signatures
   - Lifetime Annotations in Struct Definitions

2. 실습
   cargo new lifetimes_practice
   
   - longest() 함수 구현
   - ImportantExcerpt struct
   - 'static lifetime 이해

3. 일단 "작동만 하면 OK"
   완벽히 이해 못 해도 괜찮음
   계속 보면 익숙해짐
```

**Hour 2 (10:00-11:00): Rustlings - Generics + Traits**
```bash
TODO:
rustlings watch

문제 풀기:
- generics1
- generics2
- traits1
- traits2
- traits3
- traits4
- traits5

목표: 7문제 완료
Traits 문제는 어려울 수 있음
→ 30분 고민 후 힌트 확인
```

**Hour 3 (11:00-12:00): Lifetimes 문제 + Quiz**
```bash
TODO:
rustlings watch

문제 풀기:
- lifetimes1
- lifetimes2
- lifetimes3
- quiz3

quiz3 중요:
- Generics + Traits + Lifetimes 종합
- 못 풀면 1시간 더 투자
- 이해 안 되면 Day 8-9 복습
```

**Hour 4 (12:00-13:00): Testing 기초**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch11-01-writing-tests.html
   - Ch 11.1: How to Write Tests

2. 실습
   cargo new testing_practice
   
   src/lib.rs 생성:
   - #[cfg(test)] 모듈
   - #[test] 함수
   - assert!, assert_eq!, assert_ne!
   
   cargo test 실행

3. 간단한 함수 + 테스트
   - add() 함수
   - subtract() 함수
   - 각각 테스트 작성
```

**Day 9 체크리스트:**
```
□ Lifetimes 기본 실습 완료
□ Rustlings 55문제 누적 완료
□ quiz3 통과
□ Testing 기초 이해
```

---

### Day 10 (수) - 2026년 1월 15일

**Hour 1 (09:00-10:00): Iterators 기초**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch13-02-iterators.html
   - Ch 13.2: Processing a Series of Items with Iterators

2. 실습
   cargo new iterators_practice
   
   - iter() vs iter_mut() vs into_iter()
   - map() / filter() / collect()
   - sum() / count()
   - 체이닝 연습

3. notes.md
   - Iterator trait
   - Lazy evaluation
   - 성능 vs for loop
```

**Hour 2 (10:00-11:00): Closures**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch13-01-closures.html
   - Ch 13.1: Closures (Anonymous Functions)

2. 실습 (같은 프로젝트)
   - |x| x + 1 문법
   - 타입 추론
   - 환경 캡처 (move 키워드)
   - Fn, FnMut, FnOnce traits

3. 실전 예제
   Vec<i32>를
   - filter로 짝수만
   - map으로 제곱
   - collect로 새 Vec
```

**Hour 3 (11:00-12:00): Rustlings - Tests + Iterators**
```bash
TODO:
rustlings watch

문제 풀기:
- tests1
- tests2
- tests3
- tests4
- iterators1
- iterators2
- iterators3
- iterators4
- iterators5

목표: 9문제 완료
Iterators는 중요하고 자주 사용됨
```

**Hour 4 (12:00-13:00): Phase 1 마무리 프로젝트**
```bash
TODO:
미니 프로젝트: "단어 빈도수 분석기"

cargo new word_counter

기능:
1. 텍스트 입력받기 (String)
2. 단어 분리 (split_whitespace())
3. HashMap으로 카운팅
4. Iterator로 정렬
5. 상위 5개 출력

핵심 사용:
- Vec, String, HashMap
- Iterators, Closures
- Result (입력 검증)
- Tests (최소 3개)

완성 기준:
cargo test 통과
cargo run 작동
```

**Day 10 체크리스트:**
```
□ Iterators 및 Closures 이해
□ Rustlings 64문제 누적 완료
□ Phase 1 (기초) 완료
□ 단어 빈도수 분석기 완성
```

---

## Phase 2: Rust 중급 (Day 11-20)

### Day 11 (목) - 2026년 1월 16일

**Hour 1 (09:00-10:00): Modules & Packages**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
   - Ch 7.1: Packages and Crates
   - Ch 7.2: Defining Modules

2. 실습
   cargo new --lib restaurant
   
   src/lib.rs:
   - mod front_of_house
   - pub mod hosting
   - pub fn add_to_waitlist()
   
   파일 구조 실험:
   - 같은 파일 내 모듈
   - 별도 파일로 분리

3. notes.md
   - crate vs package
   - binary vs library crate
   - mod 키워드
```

**Hour 2 (10:00-11:00): Paths & Use**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
   - Ch 7.3: Paths
   - Ch 7.4: use Keyword

2. 실습 (같은 프로젝트)
   - 절대 경로: crate::
   - 상대 경로: self::, super::
   - use로 간소화
   - pub use (re-exporting)
   - as 별칭

3. 디렉토리 구조 실습
   src/
   ├── lib.rs
   ├── front_of_house.rs
   └── front_of_house/
       └── hosting.rs
```

**Hour 3 (11:00-12:00): Cargo 심화**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
   - Ch 14.1: Release Profiles
   - Ch 14.2: Publishing a Crate

2. 실습
   Cargo.toml 편집:
   
   [profile.dev]
   opt-level = 0
   
   [profile.release]
   opt-level = 3
   
   - cargo build
   - cargo build --release
   - 속도 차이 확인

3. crates.io 둘러보기
   https://crates.io/
   - 인기 crate 확인
   - 문서 읽기 연습
```

**Hour 4 (12:00-13:00): Rustlings - Modules + Macros**
```bash
TODO:
rustlings watch

문제 풀기:
- modules1 (이미 했으면 복습)
- modules2
- modules3
- macros1
- macros2
- macros3
- macros4

목표: 7문제 완료
Macros는 어려울 수 있음
→ 기본만 이해, 깊이 파지 않기
```

**Day 11 체크리스트:**
```
□ Modules 및 Paths 이해
□ use 키워드 활용
□ Cargo profiles 이해
□ Rustlings 71문제 누적 완료
```

---

### Day 12 (금) - 2026년 1월 17일

**Hour 1 (09:00-10:00): Smart Pointers - Box**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch15-01-box.html
   - Ch 15.1: Box<T>

2. 실습
   cargo new smart_pointers
   
   - Box::new() 사용
   - Heap allocation
   - Recursive type (Cons List)
   - Deref trait

3. notes.md
   - Box<T>가 필요한 경우 3가지
   - Stack vs Heap
```

**Hour 2 (10:00-11:00): Rc & RefCell**
```bash
TODO:
1. The Rust Book 읽기
   https://doc.rust-lang.org/book/ch15-04-rc.html
   - Ch 15.4: Rc<T>
   https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
   - Ch 15.5: RefCell<T>

2. 실습 (같은 프로젝트)
   - Rc::new() / Rc::clone()
   - Reference counting
   - RefCell::new() / borrow() / borrow_mut()

3. notes.md
   - Rc는 immutable만
   - RefCell은 runtime 체크
   - Rc<RefCell<T>> 패턴
```

**Hour 3 (11:00-12:00): Rustlings - Standard Library Types**
```bash
TODO:
rustlings watch

문제 풀기:
- box1
- rc1
- arc1
- cow1
- threads1
- threads2
- threads3

목표: 7문제 완료
Threads는 간단히만
```

**Hour 4 (12:00-13:00): Exercism 시작**
```bash
TODO:
1. Exercism 가입 확인
   https://exercism.org/tracks/rust

2. CLI 설치
   exercism configure --token=YOUR_TOKEN

3. Easy 문제 풀기 (3개)
   - Hello World
   - Leap
   - Reverse String

4. 제출 및 피드백 받기
   exercism submit src/lib.rs
```

**Day 12 체크리스트:**
```
□ Box, Rc, RefCell 개념 이해
□ Rustlings 78문제 누적 완료
□ Exercism 3문제 완료
```

---

### Day 13 (토) - 2026년 1월 18일

**Hour 1-2 (10:00-12:00): 실전 프로젝트 시작**
```bash
TODO:
미니 프로젝트: "CLI Todo App"

cargo new todo_app

기능:
1. 파일 I/O (todos.txt)
2. Add task
3. List tasks
4. Complete task
5. Delete task
6. Save/Load

사용 개념:
- Structs (Task)
- Enums (Command)
- Vec, String
- Result error handling
- File I/O (std::fs)
- Iterators

Hour 1: 구조 설계
- main.rs에 기본 틀
- Task struct 정의
- Command enum 정의

Hour 2: 기본 기능 구현
- add_task()
- list_tasks()
```

**Hour 3 (14:00-15:00): 프로젝트 계속**
```bash
TODO:
기능 추가:
- complete_task(id)
- delete_task(id)
- save_to_file()
- load_from_file()

Result<(), Box<dyn Error>> 활용
```

**Hour 4 (15:00-16:00): 테스트 & 마무리**
```bash
TODO:
1. Tests 작성
   #[cfg(test)]
   mod tests {
       - test_add_task()
       - test_complete_task()
       - test_save_load()
   }

2. cargo test 통과 확인

3. README.md 작성
   - 사용법
   - 예제
   - 기능 설명
```

**Day 13 체크리스트:**
```
□ CLI Todo App 완성
□ File I/O 실습
□ Tests 작성 및 통과
□ GitHub에 푸시 (선택)
```

---

### Day 14 (일) - 2026년 1월 19일 (휴식 + 복습)

**Hour 1-2 (10:00-12:00): Week 2 복습**
```bash
TODO:
1. Day 11-13 notes.md 읽기

2. 취약 부분 재학습
   - Modules 다시
   - Smart Pointers 다시
   - 프로젝트 코드 리뷰

3. Rust by Example 추가 읽기
   https://doc.rust-lang.org/rust-by-example/
   - Scoping rules
   - Traits
   - Error handling
```

**Hour 3-4 (14:00-16:00): Exercism 추가 문제**
```bash
TODO:
Exercism Easy 문제 5개 더:
- Hamming
- Raindrops
- Bob
- Armstrong Numbers
- Acronym

목표: 총 8문제 완료
피드백 읽고 개선하기
```

**Day 14 체크리스트:**
```
□ Week 2 복습 완료
□ Exercism 8문제 누적 완료
□ 컨디션 회복
```

---

### Day 15 (월) - 2026년 1월 20일

**Hour 1 (09:00-10:00): Async 기초 (맛보기만)**
```bash
TODO:
1. The Rust Book 읽기 (가볍게)
   https://doc.rust-lang.org/book/ch16-00-concurrency.html
   - Ch 16.1: Threads (간단히)
   - Ch 16.2: Message Passing (간단히)

2. Async 개념만 이해
   - async fn
   - .await
   - Future trait (깊이 X)

3. notes.md
   "Async는 고급 주제. 기본만 알기"
```

**Hour 2 (10:00-11:00): Error Handling 심화**
```bash
TODO:
1. 복습
   https://doc.rust-lang.org/book/ch09-00-error-handling.html
   Ch 9 전체 다시 읽기

2. thiserror crate 맛보기
   Cargo.toml에 추가:
   [dependencies]
   thiserror = "1.0"
   
   custom error 정의 실습

3. anyhow crate 맛보기
   간단한 에러 처리
```

**Hour 3 (11:00-12:00): Rustlings 마무리**
```bash
TODO:
rustlings watch

남은 문제 모두 풀기:
- clippy (Rust linter 규칙)
- conversions
- advanced_errs

목표: Rustlings 완전 제패
전체 95문제 완료!
```

**Hour 4 (12:00-13:00): Exercism Medium 도전**
```bash
TODO:
Exercism Medium 문제 3개:
- Luhn (난이도: Easy-Medium)
- Allergies
- Queen Attack

더 어려워짐 주의:
- 1문제당 20분 목표
- 막히면 힌트 보기
- 해설 읽기
```

**Day 15 체크리스트:**
```
□ Async 기본 개념 파악
□ Error handling 심화
□ Rustlings 95문제 전체 완료! 🎉
□ Exercism 11문제 누적 완료
```

---

### Day 16 (화) - 2026년 1월 21일

**Hour 1-2 (09:00-11:00): LeetCode 시작 (Rust로 풀기)**
```bash
TODO:
1. LeetCode 가입
   https://leetcode.com/

2. Language를 Rust로 설정

3. Easy 문제 5개 (Array/String):
   - 1. Two Sum
   - 9. Palindrome Number
   - 13. Roman to Integer
   - 20. Valid Parentheses
   - 21. Merge Two Sorted Lists

Hour 1: 문제 3개
Hour 2: 문제 2개

목표: 컴파일 에러 없이 Submit
```

**Hour 3 (11:00-12:00): LeetCode 계속 (HashMap 활용)**
```bash
TODO:
Easy 문제 3개 더:
- 217. Contains Duplicate
- 242. Valid Anagram
- 349. Intersection of Two Arrays

HashMap 사용 연습
```

**Hour 4 (12:00-13:00): Algorithm 복습**
```bash
TODO:
1. notes.md에 알고리즘 패턴 정리
   - Two Pointers
   - HashMap for O(1) lookup
   - Vec sorting

2. 틀린 문제 다시 풀기

3. Rust 특유의 패턴 정리
   - iter() vs into_iter()
   - collect::<Vec<_>>()
   - entry().or_insert()
```

**Day 16 체크리스트:**
```
□ LeetCode 8문제 완료
□ Array/String/HashMap 패턴 익힘
□ Rust 코딩테스트 감각 생김
```

---

### Day 17 (수) - 2026년 1월 22일

**Hour 1-2 (09:00-11:00): 중급 프로젝트 시작**
```bash
TODO:
미니 프로젝트: "JSON API 파서"

cargo new json_parser

목표:
1. HTTP 요청 (reqwest crate)
2. JSON 파싱 (serde_json)
3. 데이터 저장 (CSV)
4. Error handling

Cargo.toml에 추가:
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.1"

Hour 1: 설정 및 구조
- Struct 정의 (#[derive(Serialize, Deserialize)])
- reqwest로 GET 요청

Hour 2: JSON 파싱
- serde_json::from_str()
- Vec<Struct>로 변환
```

**Hour 3 (11:00-12:00): 프로젝트 완성**
```bash
TODO:
기능 완성:
1. API 호출 함수
2. JSON → Rust struct
3. CSV 저장 함수
4. main()에서 통합

사용 API 예시:
https://jsonplaceholder.typicode.com/users

테스트:
cargo run
output.csv 확인
```

**Hour 4 (12:00-13:00): Documenting**
```bash
TODO:
1. 코드에 주석 추가
   /// 함수 설명
   /// # Example
   /// ```
   /// let result = my_function();
   /// ```

2. cargo doc --open
   문서 자동 생성 확인

3. README.md 작성
   - 프로젝트 설명
   - 사용법
   - Dependencies
```

**Day 17 체크리스트:**
```
□ HTTP 요청 및 JSON 파싱 실습
□ serde, reqwest 사용법 익힘
□ 문서 작성 연습
□ 프로젝트 완성
```

---

### Day 18 (목) - 2026년 1월 23일

**Hour 1-2 (09:00-11:00): LeetCode 집중**
```bash
TODO:
Easy 문제 8개:
- 26. Remove Duplicates from Sorted Array
- 27. Remove Element
- 66. Plus One
- 88. Merge Sorted Array
- 121. Best Time to Buy and Sell Stock
- 125. Valid Palindrome
- 136. Single Number
- 169. Majority Element

목표: 2시간 8문제
1문제당 15분
```

**Hour 3 (11:00-12:00): Exercism Medium 계속**
```bash
TODO:
Exercism Medium 문제 3개:
- Nucleotide Count
- Scrabble Score
- Pangram

패턴 익히기:
- String manipulation in Rust
- HashMap for counting
- Iterators chaining
```

**Hour 4 (12:00-13:00): 알고리즘 노트 정리**
```bash
TODO:
1. notes.md에 패턴 정리
   
   ## Array 패턴
   - Two Pointers
   - Sliding Window
   
   ## String 패턴
   - chars() iteration
   - String vs &str 변환
   
   ## HashMap 패턴
   - entry().or_insert()
   - 빈도수 카운팅

2. 자주 쓰는 메서드 정리
   - collect()
   - iter().enumerate()
   - windows()
   - chunks()
```

**Day 18 체크리스트:**
```
□ LeetCode 16문제 누적 완료
□ Exercism 14문제 누적 완료
□ 알고리즘 패턴 정리
```

---

### Day 19 (금) - 2026년 1월 24일

**Hour 1-2 (09:00-11:00): 고급 프로젝트**
```bash
TODO:
미니 프로젝트: "Markdown → HTML 변환기"

cargo new markdown_converter

기능:
1. .md 파일 읽기
2. Markdown 파싱
3. HTML 생성
4. 파일 저장

사용 crate:
[dependencies]
pulldown-cmark = "0.9"

구현:
- src/main.rs: CLI 인터페이스
- src/lib.rs: 변환 로직
- tests/: 통합 테스트

Hour 1: 기본 구조
Hour 2: 파싱 및 변환
```

**Hour 3 (11:00-12:00): 프로젝트 완성**
```bash
TODO:
1. 기능 완성
   - read_markdown()
   - convert_to_html()
   - write_html()

2. 테스트 작성
   tests/integration_test.rs

3. cargo test 통과 확인
```

**Hour 4 (12:00-13:00): Phase 2 복습**
```bash
TODO:
1. Day 11-19 notes.md 전체 읽기

2. 약점 체크리스트
   □ Modules 이해?
   □ Smart Pointers 이해?
   □ Error handling 자신?
   □ Crates 사용 가능?
   □ Testing 할 수 있나?

3. 부족한 부분 1개 선택해서 재학습
```

**Day 19 체크리스트:**
```
□ Markdown 변환기 완성
□ 외부 crate 활용 능숙
□ Phase 2 복습 완료
```

---

### Day 20 (토) - 2026년 1월 25일

**Hour 1-2 (10:00-12:00): 종합 복습 Day**
```bash
TODO:
1. 전체 notes.md 읽기 (30분)
   Day 1-19 전부

2. 헷갈리는 개념 리스트 (30분)
   종이에 적기:
   - 아직 이해 부족한 개념
   - 자주 틀리는 패턴
   - 기억 안 나는 문법

3. 집중 재학습 (60분)
   리스트의 상위 3개 개념
   Book 다시 읽기
   예제 다시 풀기
```

**Hour 3-4 (14:00-16:00): Mock Test**
```bash
TODO:
1. LeetCode Easy 5문제 (60분)
   타이머 켜기
   한 번에 5문제 풀기
   막혀도 스킵하지 말기
   
   추천 문제:
   - 283. Move Zeroes
   - 344. Reverse String
   - 387. First Unique Character
   - 412. Fizz Buzz
   - 509. Fibonacci Number

2. 결과 분석 (60분)
   - 걸린 시간
   - 컴파일 에러 횟수
   - 로직 실수
   - Rust 특유 에러
   
   notes.md에 기록
```

**Day 20 체크리스트:**
```
□ Phase 2 (중급) 완료
□ 전체 복습 완료
□ Mock test 완료
□ 약점 파악 완료
```

---

## Phase 3: 실전 준비 (Day 21-27)

### Day 21 (일) - 2026년 1월 26일 (휴식)

**Hour 1-2 (자유 시간): 가벼운 복습**
```bash
TODO (선택):
1. YouTube 시청
   채널: No Boilerplate
   "Rust for the impatient"
   
2. Rust by Example 재독
   https://doc.rust-lang.org/rust-by-example/
   재미있는 챕터만

3. Reddit 구경
   r/rust
   r/learnrust
   초보자 질문 읽기
```

**Hour 3-4: Exercism 자유롭게**
```bash
TODO:
좋아하는 문제 풀기
Medium 도전해도 됨
부담 없이 진행
```

---

### Day 22 (월) - 2026년 1월 27일

**Hour 1-2 (09:00-11:00): LeetCode 집중 (Medium 도전)**
```bash
TODO:
Medium 문제 4개:
- 2. Add Two Numbers
- 3. Longest Substring Without Repeating Characters
- 5. Longest Palindromic Substring
- 15. 3Sum

시간 제한: 문제당 30분
못 풀어도 OK
→ 해설 보고 이해하기
```

**Hour 3 (11:00-12:00): Rust Patterns 학습**
```bash
TODO:
1. Rust Design Patterns 읽기
   https://rust-unofficial.github.io/patterns/
   
   읽을 섹션:
   - Idioms (Rust다운 코드)
   - Design Patterns (일부)
   
2. notes.md에 정리
   - Newtype pattern
   - Constructor pattern
   - Builder pattern (간단히)
```

**Hour 4 (12:00-13:00): 코드 리뷰**
```bash
TODO:
1. 지금까지 만든 프로젝트 전부 열기
   - Todo CLI
   - JSON API Parser
   - Markdown Converter

2. 리팩토링 아이디어
   - 중복 코드 제거
   - 에러 처리 개선
   - 함수 분리
   - 주석 추가

3. 1개 프로젝트 개선하기
```

**Day 22 체크리스트:**
```
□ LeetCode Medium 도전
□ Rust Patterns 학습
□ 프로젝트 리팩토링
```

---

### Day 23 (화) - 2026년 1월 28일

**Hour 1-2 (09:00-11:00): 최종 프로젝트 기획**
```bash
TODO:
프로젝트: "Command Line 유틸리티 모음"

cargo new cli_tools --lib

기능 (각각 모듈로):
1. Base64 인코더/디코더
2. Password 생성기
3. File hasher (SHA256)
4. 간단한 암호화

Hour 1: 구조 설계
- lib.rs에 mod 선언
- 각 모듈 파일 생성
- 공통 함수 정의

Hour 2: Base64 & Password
- base64 crate 사용
- rand crate로 패스워드 생성
```

**Hour 3 (11:00-12:00): 프로젝트 계속**
```bash
TODO:
기능 구현:
- File hasher
- 간단한 XOR 암호화

Cargo.toml:
[dependencies]
base64 = "0.21"
rand = "0.8"
sha2 = "0.10"
```

**Hour 4 (12:00-13:00): 테스트 & 문서**
```bash
TODO:
1. 각 모듈 테스트 작성
2. 통합 테스트 작성
3. cargo doc --open
4. README.md 작성
5. examples/ 디렉토리에 사용 예제
```

**Day 23 체크리스트:**
```
□ CLI 유틸리티 완성
□ 복합 crate 프로젝트 경험
□ 문서 및 테스트 완비
```

---

### Day 24 (수) - 2026년 1월 29일

**Hour 1-2 (09:00-11:00): LeetCode 마라톤**
```bash
TODO:
Easy 문제 10개 연속 풀기
타이머: 2시간

목표: 속도와 정확도
- 컴파일 에러 최소화
- 로직 한 번에 맞히기
- Rust idioms 사용

추천 문제:
- 268. Missing Number
- 350. Intersection of Two Arrays II
- 383. Ransom Note
- 392. Is Subsequence
- 405. Convert a Number to Hexadecimal
- 434. Number of Segments in a String
- 448. Find All Numbers Disappeared in an Array
- 455. Assign Cookies
- 459. Repeated Substring Pattern
- 461. Hamming Distance
```

**Hour 3 (11:00-12:00): Rust Quiz**
```bash
TODO:
1. Rust Quiz 풀기
   https://dtolnay.github.io/rust-quiz/
   
   - 첫 10문제 풀기
   - 틀린 문제 분석
   - 개념 재학습

2. notes.md에 트릭 기록
```

**Hour 4 (12:00-13:00): Weak Points 보완**
```bash
TODO:
1. LeetCode 틀린 문제 다시 풀기

2. Exercism 피드백 반영

3. 자주 실수하는 패턴 정리
   notes.md에 "실수 체크리스트"
```

**Day 24 체크리스트:**
```
□ LeetCode 26문제 누적 완료
□ Rust Quiz 도전
□ 약점 보완
```

---

### Day 25 (목) - 2026년 1월 30일

**Hour 1-2 (09:00-11:00): Exercism 마무리**
```bash
TODO:
Exercism 문제 풀기
Medium/Hard 도전:
- Bowling
- Roman Numerals
- Saddle Points
- Rotational Cipher
- Wordy

목표: 총 20문제 완료
```

**Hour 3-4 (11:00-13:00): 종합 Mock Test**
```bash
TODO:
실전처럼 테스트 (2시간)

Section 1: 개념 (30분)
- Ownership 설명하기 (영어 작문)
- Borrowing 규칙 설명
- Trait vs Interface 차이
- Lifetime이 필요한 이유

Section 2: 코딩 (90분)
- LeetCode Easy 3문제
- LeetCode Medium 2문제
- Exercism Medium 1문제

시간 엄수
자료 참고 금지
```

**Day 25 체크리스트:**
```
□ Exercism 20문제 완료
□ 종합 Mock Test 완료
□ 실전 감각 체득
```

---

### Day 26 (금) - 2026년 1월 31일

**Hour 1-2 (09:00-11:00): Rust Cheat Sheet 만들기**
```bash
TODO:
1. 새 파일: rust_cheatsheet.md

2. 섹션별 정리:
   
   ## Syntax
   - let vs let mut
   - & vs &mut
   - impl vs trait
   
   ## Collections
   - Vec: 자주 쓰는 메서드
   - HashMap: 패턴들
   - String: 변환
   
   ## Patterns
   - Result 처리 3가지
   - Option 처리 3가지
   - Iterator 체이닝
   
   ## Common Errors
   - Borrow checker 에러
   - Lifetime 에러
   - Type mismatch

3. 코드 스니펫 추가
   복붙 가능하게
```

**Hour 3 (11:00-12:00): 최종 프로젝트 선택 및 시작**
```bash
TODO:
선택 1: Web Scraper
- reqwest + scraper crate
- HTML 파싱
- CSV 저장

선택 2: CLI Tool
- clap crate로 arguments
- 파일 처리
- 진행 상황 표시

선택 3: 간단한 REST API
- actix-web 또는 rocket
- JSON endpoint
- 기본 CRUD

1개 선택해서 구조 잡기
```

**Hour 4 (12:00-13:00): 프로젝트 구현**
```bash
TODO:
선택한 프로젝트 구현 시작
기본 기능 완성
```

**Day 26 체크리스트:**
```
□ Cheat Sheet 완성
□ 최종 프로젝트 선택
□ 프로젝트 기초 구현
```

---

### Day 27 (토) - 2026년 2월 1일

**Hour 1-2 (10:00-12:00): 최종 프로젝트 완성**
```bash
TODO:
1. 기능 완성
2. 테스트 작성
3. 문서 작성
4. GitHub 업로드

README.md 포함:
- 프로젝트 설명
- 사용법
- 예제
- 배운 점
```

**Hour 3 (14:00-15:00): Portfolio 정리**
```bash
TODO:
GitHub Profile README.md 작성

## Rust Projects
1. Todo CLI - File I/O, Error handling
2. JSON API Parser - HTTP, Serde
3. Markdown Converter - External crates
4. CLI Utilities - Crypto, Hashing
5. [최종 프로젝트명] - [기술]

각 프로젝트 링크
기술 스택 명시
```

**Hour 4 (15:00-16:00): 최종 복습**
```bash
TODO:
1. Cheat Sheet 읽기
2. notes.md 전체 훑기
3. 자신감 체크:
   □ Ownership 설명 가능?
   □ Traits 구현 가능?
   □ Error handling 자신?
   □ Iterators 자유자재?
   □ 프로젝트 만들기 가능?

4. 마음 정리
   "나는 준비됐다"
```

**Day 27 체크리스트:**
```
□ 최종 프로젝트 완성
□ Portfolio 정리
□ 27일 학습 완료
□ 테스트 준비 완료
```

---

## Phase 4: Buffer (Day 28-30)

### Day 28-30 (일-화) - 2026년 2월 2-4일

**각 Day 별 2시간씩 (자유롭게)**

```bash
TODO:
1. 가벼운 복습
2. LeetCode Easy 추가 풀이
3. Cheat Sheet 보강
4. 멘탈 관리
5. 테스트 전날: 가벼운 코딩만

목표:
- 과도한 공부 금지
- 번아웃 방지
- 컨디션 유지
```

---

## 테스트 당일 (Day 31) - 2026년 2월 5일

### 준비

```bash
전날 밤:
□ 일찍 자기 (8시간 수면)
□ 컴퓨터 충전
□ 인터넷 확인
□ Cheat Sheet 한 번만 읽기

당일 아침:
□ 가벼운 아침식사
□ 커피 (평소 마시면)
□ 30분 warming up (Easy 문제 1개)
□ 화장실
□ 조용한 공간 확보

테스트 중:
□ 모르는 문제 스킵 후 나중에
□ 시간 배분 (General 30분, Rust 90분)
□ 침착하게
□ 컴파일 에러 차분히 읽기
```

---

## 추가 자료 링크 모음

### 필수
```
The Rust Book
https://doc.rust-lang.org/book/

Rustlings
https://github.com/rust-lang/rustlings

Exercism Rust Track
https://exercism.org/tracks/rust

LeetCode
https://leetcode.com/
```

### 보조
```
Rust by Example
https://doc.rust-lang.org/rust-by-example/

Rust Cheat Sheet
https://cheats.rs/

Let's Get Rusty (YouTube)
https://www.youtube.com/@letsgetrusty

r/rust (Reddit)
https://www.reddit.com/r/rust/

r/learnrust (Reddit)
https://www.reddit.com/r/learnrust/
```

---

## 진행 상황 체크리스트

**매일 저녁 기록:**
```
날짜: 2026년 1월 __일
오늘의 목표:
□ 
□ 
□ 

완료 여부:
□ 
□ 
□ 

배운 것:
-
-

어려웠던 것:
-
-

내일 계획:
-
-
```

---

## 최종 목표

```
Day 30 (2026년 2월 4일) 기준:

✅ Rust 기초 ~ 중급 완성
✅ Rustlings 95문제 완료
✅ LeetCode 30+ 문제
✅ Exercism 20+ 문제
✅ 실전 프로젝트 5개
✅ Cheat Sheet 완성
✅ GitHub Portfolio

예상 Pass 확률: 70-80%
```


## Rust coding test Pattern
### Array 문제
```rust
// 자주 쓰는 패턴들

// 1. Two Pointers
let mut left = 0;
let mut right = nums.len() - 1;
while left < right {
    // logic
}

// 2. Iteration with index
for (i, &num) in nums.iter().enumerate() {
    // logic
}

// 3. Sliding Window
for window in nums.windows(k) {
    // logic
}

// 4. 정렬 후 처리
let mut nums = nums;
nums.sort_unstable();

```

### String 문제
```rust
// 자주 쓰는 패턴들

// 1. chars() iteration
for ch in s.chars() {
    // logic
}

// 2. bytes() for ASCII
for byte in s.bytes() {
    // logic
}

// 3. String 변환
let s: String = chars.iter().collect();
let s = String::from("hello");

// 4. Reverse
let reversed: String = s.chars().rev().collect();

// 5. char counting
let mut map = std::collections::HashMap::new();
for ch in s.chars() {
    *map.entry(ch).or_insert(0) += 1;
}
```

### Hash Map 문제
```rust
use std::collections::HashMap;

// 1. 빈도수 카운팅
let mut map = HashMap::new();
for &num in nums {
    *map.entry(num).or_insert(0) += 1;
}

// 2. 존재 여부 확인
if map.contains_key(&key) {
    // logic
}

// 3. Two Sum 패턴
let mut map = HashMap::new();
for (i, &num) in nums.iter().enumerate() {
    let complement = target - num;
    if let Some(&j) = map.get(&complement) {
        return vec![j, i];
    }
    map.insert(num, i);
}
```

### Linked LIst
```rust
// Definition for singly-linked list
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// 1. Traverse
let mut current = head;
while let Some(node) = current {
    // logic
    current = node.next;
}

// 2. Dummy head 패턴
let mut dummy = Box::new(ListNode::new(0));
let mut tail = &mut dummy;
```


## LEET CODE

- Array
1. Two Sum ⭐⭐⭐
26. Remove Duplicates from Sorted Array
27. Remove Element
53. Maximum Subarray ⭐⭐
66. Plus One
88. Merge Sorted Array
118. Pascal's Triangle
119. Pascal's Triangle II
121. Best Time to Buy and Sell Stock ⭐⭐
122. Best Time to Buy and Sell Stock II
167. Two Sum II
169. Majority Element
217. Contains Duplicate
268. Missing Number
283. Move Zeroes

- String
13. Roman to Integer
14. Longest Common Prefix
20. Valid Parentheses ⭐⭐⭐
28. Find the Index
58. Length of Last Word
125. Valid Palindrome ⭐
242. Valid Anagram ⭐⭐
344. Reverse String
345. Reverse Vowels
383. Ransom Note
387. First Unique Character ⭐
392. Is Subsequence
415. Add Strings
459. Repeated Substring Pattern
520. Detect Capital

- HashMap/HashSet
1. Two Sum (중복)
136. Single Number ⭐
202. Happy Number
205. Isomorphic Strings
217. Contains Duplicate (중복)
219. Contains Duplicate II
242. Valid Anagram (중복)
349. Intersection of Two Arrays ⭐
350. Intersection of Two Arrays II
389. Find the Difference

- Linked List
21. Merge Two Sorted Lists ⭐⭐
83. Remove Duplicates from Sorted List
141. Linked List Cycle ⭐
160. Intersection of Two Linked Lists
203. Remove Linked List Elements

- Tree
100. Same Tree
101. Symmetric Tree
104. Maximum Depth of Binary Tree ⭐
226. Invert Binary Tree ⭐
543. Diameter of Binary Tree

- Maths/Bit
9. Palindrome Number
231. Power of Two
268. Missing Number (중복)
338. Counting Bits
461. Hamming Distance

-Two Pointers
26. Remove Duplicates (중복)
27. Remove Element (중복)
125. Valid Palindrome (중복)
344. Reverse String (중복)
345. Reverse Vowels (중복)

-Medium 10문제 (입문용)
2. Add Two Numbers (Linked List)
3. Longest Substring Without Repeating Characters ⭐⭐⭐
5. Longest Palindromic Substring
15. 3Sum ⭐⭐
49. Group Anagrams
56. Merge Intervals
75. Sort Colors
94. Binary Tree Inorder Traversal
102. Binary Tree Level Order Traversal
238. Product of Array Except Self ⭐⭐