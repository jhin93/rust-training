# 20일 Rust AI Trainer 테스트 준비 (하루 2시간) - 개정판

## Daily Task List (Day 1-20)

### Week 1: Rust 기본 + 알고리즘 시작

**Day 1 (2시간) - Rust 기초 문법 집중**
```
Hour 1: 변수와 함수 마스터
□ let/let mut 차이 손으로 10번 쓰기 - 불변/가변 변수 개념 체화
□ 함수 선언 문법 타이핑 20번 - fn 키워드와 리턴 타입 익히기
□ println! 매크로 5가지 예제 - 출력 문법 완전 숙지
□ cargo new로 3개 프로젝트 만들어보기 - Rust 프로젝트 구조 이해

Hour 2: 조건문과 반복문 암기
□ if/else/else if 예제 10개 타이핑 - 조건문 완전 체득
□ for loop 5가지 패턴 암기 - 범위, 벡터, enumerate 순회
□ while loop 예제 3개 - 조건 반복 이해
□ match 표현식 기초 5개 - Rust의 강력한 패턴 매칭 입문
```

**Day 2 (2시간) - Vec 완전 정복**
```
Hour 1: Vec 생성과 기본 메서드
□ vec! 매크로 vs Vec::new() 차이 이해 - 벡터 초기화 2가지 방법
□ push/pop/insert/remove 각 5번씩 - 데이터 추가/제거 완벽 숙지
□ 인덱싱 vs get() 차이 실험 - 안전한 접근 vs panic 이해
□ len/is_empty/contains 활용 - 벡터 상태 확인 메서드들

Hour 2: LeetCode Easy #1 + Vec 복습
□ 문제: Two Sum (해시맵 없이 브루트포스) - 이중 for문으로 먼저 풀기
□ Vec 순회 3가지 방법 비교 - for, iter(), enumerate() 차이
□ 풀이 후 Vec 메서드 총정리 노트 작성 - 오늘 배운 것 문서화
```

**Day 3 (2시간) - HashMap 핵심**
```
Hour 1: HashMap 생성과 CRUD
□ use 문으로 HashMap import - 표준 라이브러리 사용법
□ insert/get/remove 각 10번 타이핑 - 기본 연산 손에 익히기
□ entry().or_insert() 패턴 암기 - 빈도수 카운팅의 핵심 패턴
□ for (key, val) in map 순회 - HashMap 반복 완전 이해

Hour 2: LeetCode Easy #2 + HashMap 활용
□ 문제: Valid Anagram - 문자 빈도수 비교 문제
□ HashMap으로 문자 개수 세기 구현 - entry().or_insert() 실전 적용
□ Two Sum을 HashMap으로 다시 풀기 - O(n) 최적화 경험
```

**Day 4 (2시간) - String 처리**
```
Hour 1: String vs &str 완벽 이해
□ String::from() vs to_string() 차이 - 소유권 있는 문자열 생성
□ chars() iterator 활용법 5가지 - 문자 단위 순회 마스터
□ split/trim/replace 예제 각 3개 - 문자열 조작 필수 메서드
□ format! 매크로 활용 - 문자열 포매팅 익히기

Hour 2: LeetCode Easy #3 + String 실전
□ 문제: Valid Palindrome - 문자열 뒤집기와 비교
□ chars().rev().collect() 패턴 암기 - 문자열 역순 표준 방법
□ to_lowercase() 활용 - 대소문자 무시 비교 구현
```

**Day 5 (2시간) - 정렬과 Two Pointers**
```
Hour 1: 정렬 메서드 완전 정복
□ sort() vs sort_unstable() 차이 - 안정성 vs 속도 트레이드오프
□ sort_by로 역순/커스텀 정렬 - 클로저 활용한 정렬 기준
□ reverse() 메서드 - 벡터 뒤집기 간단한 방법
□ Two Pointers 개념과 패턴 학습 - 양쪽 끝에서 접근하는 알고리즘

Hour 2: LeetCode Easy #4 + 정렬 응용
□ 문제: Merge Sorted Array - 정렬된 배열 병합 문제
□ Two Pointers로 효율적 병합 - 뒤에서부터 채우는 트릭
□ sort_unstable() 적용 비교 - 단순 정렬 vs 최적 알고리즘 차이
```

**Day 6 (2시간) - Iterator 마스터**
```
Hour 1: Iterator 기본과 체이닝
□ iter() vs into_iter() vs iter_mut() - 참조/소유권/가변 참조 차이
□ map/filter/collect 체이닝 10번 - 함수형 프로그래밍 패턴 체화
□ sum/max/min 등 집계 메서드 - Iterator 최종 연산들
□ enumerate()로 인덱스+값 동시 순회 - (index, value) 튜플 활용

Hour 2: LeetCode Easy #5 + Iterator 실전
□ 문제: Single Number (XOR 트릭) - 비트 연산 + iterator
□ fold()로 XOR 누적 구현 - reduce 패턴 이해
□ filter().map().collect() 체이닝 연습 - 복합 변환 익히기
```

**Day 7 (2시간) - Week 1 복습 및 실전**
```
Hour 1: Cheat Sheet 작성 및 암기
□ Vec 핵심 메서드 10개 정리 - 손으로 쓰면서 암기
□ HashMap 패턴 3가지 정리 - insert, get, entry 중심
□ String 변환 방법 정리 - from, to_string, chars, split
□ Iterator 체이닝 템플릿 작성 - 자주 쓰는 패턴 문서화

Hour 2: Mini Mock Test (실전 시뮬레이션)
□ 짝수 합 문제 10분 안에 풀기 - 당신이 본 테스트 유형 연습
□ Array 관련 Easy 문제 1개 더 - 시간 압박 속 문제 해결
□ 틀린 부분 분석하고 재풀이 - 실수 패턴 파악
```

### Week 2: 알고리즘 패턴 집중

**Day 8 (2시간) - Two Pointers 심화**
```
Hour 1: Two Pointers 알고리즘 이해
□ left/right 포인터 개념 완벽 이해 - 양쪽 끝에서 만나는 전략
□ Remove Duplicates 패턴 분석 - 정렬 배열에서 중복 제거
□ 종료 조건 3가지 케이스 - left < right, left <= right, 언제 쓰나

Hour 2: Two Pointers 문제 3개 연속
□ Remove Element - 특정 값 제거하며 포인터 이동
□ Move Zeroes - 0을 뒤로 보내는 포인터 활용
□ Container With Most Water - 최대 넓이 찾기 포인터 전략
```

**Day 9 (2시간) - Sliding Window 패턴**
```
Hour 1: Sliding Window 개념 학습
□ 고정 크기 vs 가변 크기 윈도우 - 두 가지 슬라이딩 윈도우 유형
□ 부분 배열 합 최대화 패턴 - 윈도우 이동하며 최적값 찾기
□ current_sum 업데이트 로직 - 효율적인 합 계산 방법

Hour 2: Sliding Window 문제 2개
□ Maximum Subarray (Kadane's Algorithm) - 연속 부분합 최대값
□ Best Time to Buy and Sell Stock - 최저점 찾고 최대 이익 계산
```

**Day 10 (2시간) - HashMap 고급 활용**
```
Hour 1: HashMap 실전 패턴
□ 빈도수 카운팅 완벽 숙달 - *map.entry(k).or_insert(0) += 1
□ Two Sum 패턴 완전 이해 - complement 찾기 전략
□ seen HashSet 활용 - 중복 체크용 자료구조

Hour 2: HashMap 문제 3개 마스터
□ Contains Duplicate - HashSet으로 중복 판별
□ Intersection of Two Arrays - 교집합 구하기
□ First Unique Character - 빈도수 1인 첫 문자 찾기
```

**Day 11 (2시간) - String 알고리즘**
```
Hour 1: String 처리 고급 기법
□ chars() + collect::<String>() 패턴 - 문자 배열을 문자열로 변환
□ 회문 판별 2가지 방법 - Two Pointers vs 문자열 뒤집기
□ ASCII vs Unicode 처리 차이 - bytes() vs chars() 선택 기준

Hour 2: String 문제 3개 풀이
□ Reverse String - chars().rev().collect() 활용
□ Valid Palindrome - 공백/특수문자 제거 후 비교
□ Longest Common Prefix - 모든 문자열의 공통 접두사 찾기
```

**Day 12 (2시간) - Stack 패턴 (Vec 활용)**
```
Hour 1: Stack 자료구조 Rust로 구현
□ Vec를 Stack으로 쓰는 방법 - push/pop이 stack 연산
□ Matching Parentheses 패턴 - 괄호 짝 맞추기 알고리즘
□ Stack 활용 시기 판단 - LIFO가 필요한 문제들

Hour 2: Stack 문제 2개
□ Valid Parentheses - 여러 종류 괄호 매칭
□ Implement Queue using Stacks - 두 스택으로 큐 만들기
```

**Day 13 (2시간) - 수학/비트 연산**
```
Hour 1: 수학과 비트 트릭
□ XOR 연산 특성 3가지 - a^a=0, a^0=a, 교환법칙
□ 2의 거듭제곱 판별법 - n & (n-1) == 0 트릭
□ 나머지 연산 활용 - Fizz Buzz 같은 주기 문제

Hour 2: 수학 문제 3개
□ Fizz Buzz - 3의배수/5의배수/15의배수 구분
□ Power of Two - 비트 연산으로 판별
□ Reverse Bits - 비트 뒤집기 알고리즘
```

**Day 14 (2시간) - Week 2 종합 복습**
```
Hour 1: 패턴별 정리와 암기
□ 6가지 패턴 Cheat Sheet 업데이트 - Two Pointers부터 비트까지
□ 각 패턴의 시간복잡도 정리 - O(n), O(n log n) 등
□ 패턴 선택 기준 정리 - 어떤 문제에 어떤 패턴 쓰나
□ 틀린 문제 3개 재풀이 - 약점 보완

Hour 2: Mock Test #2 (중간 점검)
□ Easy 3문제 60분 제한 - 실전 시간 압박 체험
□ 각 문제 시간 기록 - 15분, 20분, 25분 목표
□ 컴파일 에러 횟수 세기 - 실수 줄이기 연습
```

### Week 3: Medium 도전 + 시스템 설계

**Day 15 (2시간) - 3Sum 완전 정복**
```
Hour 1: 3Sum 알고리즘 완벽 이해
□ 정렬 + Two Pointers 전략 - Medium의 대표 패턴
□ 중복 제거 로직 3곳 파악 - i, left, right 각각 처리
□ 시간복잡도 O(n²) 이해 - 왜 이게 최선인가

Hour 2: 3Sum 문제 완벽 구현
□ 당신이 본 그 문제! 60분 풀기 - [-1,-1,0,1,2,4] 예제
□ 틀리면 해설 보고 이해 - 로직 완전 소화
□ 암기할 때까지 3번 재작성 - 다음엔 15분에 풀기
```

**Day 16 (2시간) - Medium Array 추가**
```
Hour 1: Product of Array 패턴
□ Product of Array Except Self 이해 - 나눗셈 없이 곱 구하기
□ 왼쪽 곱 + 오른쪽 곱 전략 - 두 번 순회로 O(n)
□ 공간 최적화 기법 - O(1) 추가 공간

Hour 2: Medium 문제 2개
□ Product of Array Except Self - 전체 구현
□ Container With Most Water - Two Pointers 고급 응용
```

**Day 17 (2시간) - DP 입문 + String**
```
Hour 1: Dynamic Programming 기초
□ Fibonacci DP 3가지 방법 - 재귀, 메모이제이션, 타뷸레이션
□ DP 테이블 설계 개념 - dp[i]가 무엇을 의미하는가
□ Bottom-up vs Top-down - 각각의 장단점

Hour 2: DP 문제 2개
□ Climbing Stairs - 기본 DP 입문 문제
□ Longest Substring Without Repeating - 슬라이딩 윈도우 + HashMap
```

**Day 18 (2시간) - 시스템 설계 준비**
```
Hour 1: CRUD 앱 설계 연습
□ 메모 앱 데이터 모델 설계 - struct Memo 정의
□ RESTful API 엔드포인트 5개 - POST, GET, PUT, DELETE, LIST
□ 3-tier architecture 이해 - Frontend, Backend, Database 분리
□ Concurrency 문제 3가지 - Race condition, 중복 저장, 충돌

Hour 2: 영상 답변 스크립트 작성 및 연습
□ "메모 앱 설계" 5분 답변 작성 - 구조화된 답변 준비
□ "Concurrency 처리" 5분 답변 작성 - 구체적 해결책 포함
□ 2번 녹화 연습 (각 5분) - 자연스럽게 말하기 연습
□ 시간 체크하며 템포 조절 - 너무 빠르지도 느리지도 않게
```

**Day 19 (2시간) - 최종 점검 및 약점 보완**
```
Hour 1: Rust 문법 최종 암기
□ Cheat Sheet 전체 10분 복습 - Vec, HashMap, String, Iterator
□ 자주 틀리는 문법 10번 쓰기 - collect::<Vec<_>>() 등
□ 컴파일 에러 원인 5가지 정리 - 소유권, 타입, lifetime 등
□ 자주 쓰는 메서드 암송 - push, pop, insert, get, iter

Hour 2: 약점 집중 공략
□ 틀렸던 문제 3개 재풀이 - 이번엔 15분 안에
□ 시간 오래 걸린 문제 1개 - 속도 향상 연습
□ 새로운 Easy 문제 1개 - 10분 안에 풀기 도전
```

**Day 20 (2시간) - Final Mock Test**
```
Hour 1: 코딩 테스트 최종 시뮬레이션
□ 짝수 합 문제 (10분) - 당신이 본 1번 문제 유형
□ 3Sum 또는 유사 문제 (45분) - 당신이 본 2번 문제 유형
□ 시간 엄수하고 제출까지 - 실전처럼 긴장감 유지
□ 풀이 후 리팩토링 - 더 깔끔한 코드로 개선

Hour 2: 영상 면접 최종 리허설
□ 시스템 설계 질문 2개 녹화 - 메모 앱, Concurrency
□ 5분 시간 제한 엄수 - 타이머 켜고 연습
□ 재생해서 피드백 - 말투, 속도, 논리성 체크
□ 자신감 체크리스트 점검 - 내일 시험 준비 완료
```

---

## Rust 코딩테스트 필수 개념 (주석 추가판)

### 1. 변수 & 타입
```rust
// === 불변 변수 (기본) ===
let x = 5;           // let = 변수 선언 키워드, 기본은 불변(immutable)

// === 가변 변수 ===
let mut y = 10;      // mut = mutable의 약자, 값 변경 가능하게 만듦
y = 15;              // mut 없으면 이 줄에서 에러 발생

// === 타입 명시 (보통은 자동 추론) ===
let a: i32 = 5;      // i32 = 32비트 정수 (integer 32-bit)
let b: f64 = 3.14;   // f64 = 64비트 실수 (float 64-bit)
let c: bool = true;  // bool = boolean, true/false만 가능

// === 문자열 2가지 타입 ===
let s: String = String::from("hello");  // String = 소유권 있는 문자열, 힙에 저장
                                        // :: = 연관 함수 호출 (static method와 유사)
                                        // from = String 타입의 연관 함수
let s2: &str = "world";                 // &str = 문자열 슬라이스, 참조만 (불변)
                                        // &(앰퍼샌드) = 참조를 의미하는 기호
```

### 2. 함수
```rust
// === 기본 함수 선언 ===
fn add(a: i32, b: i32) -> i32 {  // fn = function 선언 키워드
                                  // -> i32 = 리턴 타입 (화살표 뒤가 반환 타입)
    a + b  // 세미콜론 없으면 리턴값 (expression)
           // 세미콜론 있으면 statement, 리턴 안 됨
}

// === 함수 사용 ===
fn main() {  // main = 프로그램 시작점
    let result = add(5, 3);        // 함수 호출, 결과를 result에 저장
    println!("{}", result);        // println! = 출력 매크로 (!가 매크로 표시)
                                   // {} = placeholder, result 값이 들어감
}
```

### 3. 조건문
```rust
// === if-else 기본 ===
if x > 5 {                    // 괄호() 없어도 됨 (Python처럼)
    println!("big");          // 중괄호{}는 필수
} else if x > 0 {
    println!("small");
} else {
    println!("zero or negative");
}

// === match (switch문보다 강력) ===
match x {                     // match = 패턴 매칭 키워드
    0 => println!("zero"),    // => = 패턴 매칭 화살표
    1..=5 => println!("1-5"), // ..= = 범위 (1,2,3,4,5 모두 포함)
    _ => println!("other"),   // _ = 나머지 모든 경우 (default)
}                             // match는 모든 경우를 다뤄야 함 (exhaustive)
```

### 4. 반복문
```rust
// === for loop (범위) ===
for i in 0..5 {       // 0..5 = 0,1,2,3,4 (5는 포함 안 됨)
    println!("{}", i);
}

// === for loop (벡터 순회) ===
let vec = vec![1, 2, 3];      // vec! = 벡터 생성 매크로
for num in &vec {             // &vec = 벡터의 참조 (소유권 안 가져감)
    println!("{}", num);      // num은 &i32 타입 (참조)
}

// === enumerate (인덱스 + 값) ===
for (i, val) in vec.iter().enumerate() {  // enumerate() = (인덱스, 값) 튜플 반환
    println!("{}: {}", i, val);           // i = 인덱스, val = 값
}

// === while loop ===
let mut i = 0;        // mut 필수 (값을 변경할 거니까)
while i < 5 {         // 조건이 true인 동안 반복
    i += 1;           // i = i + 1과 같음
}
```

### 5. Vec (배열) - 매우 중요!
```rust
// === 벡터 생성 2가지 방법 ===
let mut v = Vec::new();       // Vec = 동적 배열 (크기 변경 가능)
                              // :: = 연관 함수 호출
                              // new() = 빈 벡터 생성
let v2 = vec![1, 2, 3];       // vec! = 매크로, 값과 함께 초기화
                              // ! = 매크로 표시 (함수 아님)

// === 추가/제거 ===
v.push(4);            // push = 끝에 추가 (append)
let last = v.pop();   // pop = 끝에서 제거하고 반환
                      // Option<T> 타입 반환 (값이 있거나 없을 수 있음)

// === 접근 2가지 방법 ===
let first = v[0];             // [0] = 인덱스 접근 (없으면 panic!)
let second = v.get(1);        // get() = 안전한 접근, Option<&T> 반환
                              // 없어도 panic 안 남

// === 정렬 ===
v.sort();                     // sort = 오름차순 정렬 (기본)
v.sort_unstable();            // sort_unstable = 더 빠름 (순서 보장 안 됨)
v.sort_by(|a, b| b.cmp(a));   // sort_by = 커스텀 정렬
                              // |a, b| = 클로저 (익명 함수)
                              // b.cmp(a) = b와 a 비교 (역순)
v.reverse();                  // reverse = 순서 뒤집기

// === 유용한 메서드들 ===
let length = v.len();         // len = 길이 반환
let empty = v.is_empty();     // is_empty = 비어있는지 확인 (bool)
let has = v.contains(&3);     // contains = 값 포함 여부 (&로 참조 전달)
let total: i32 = v.iter().sum();  // iter() = iterator 생성
                                   // sum() = 모든 요소 합계
                                   // ::<i32> = 타입 명시 (turbofish 문법)
let max_val = v.iter().max(); // max = 최댓값 (Option 반환)
let min_val = v.iter().min(); // min = 최솟값
```

### 6. HashMap - 매우 중요!
```rust
use std::collections::HashMap;  // use = import와 같음
                                // std = standard library
                                // :: = 경로 구분자

let mut map = HashMap::new();   // HashMap = key-value 저장소
                                // new() = 빈 해시맵 생성

// === 삽입 ===
map.insert("key", 10);          // insert(key, value) = 키-값 쌍 추가

// === 조회 (안전한 방법) ===
if let Some(&val) = map.get("key") {  // get() = Option<&V> 반환
                                       // if let = 패턴 매칭으로 값 추출
                                       // Some = Option의 값 있는 경우
                                       // &val = 참조 벗기기 (dereference)
    println!("{}", val);
}

// === 빈도수 카운팅 패턴 (암기 필수!) ===
*map.entry(word).or_insert(0) += 1;   // entry() = 키의 Entry 가져오기
                                      // or_insert(0) = 없으면 0 삽입
                                      // * = 역참조 (값 직접 수정)
                                      // += 1 = 카운트 증가

// === 반복 (모든 key-value 순회) ===
for (key, val) in &map {              // &map = 해시맵 참조로 순회
    println!("{}: {}", key, val);     // key, val = 튜플 구조분해
}
```

### 7. String - 중요!
```rust
// === String 생성 2가지 ===
let s = String::from("hello");        // String::from() = &str을 String으로
let s2 = "world".to_string();         // to_string() = 메서드 체이닝 방식

// === 문자 단위 반복 ===
for ch in s.chars() {                 // chars() = 문자(char) iterator 반환
    println!("{}", ch);               // UTF-8 문자 단위 (한글도 OK)
}

// === 문자열 분할 ===
let words: Vec<&str> = s.split_whitespace().collect();  
                       // split_whitespace() = 공백 기준 분할
                       // collect() = iterator를 collection으로
                       // Vec<&str> = 문자열 슬라이스 벡터
                       // <> = 제네릭 타입 명시

// === 대소문자 변환 ===
let lower = s.to_lowercase();         // to_lowercase() = 모두 소문자로
let upper = s.to_uppercase();         // to_uppercase() = 모두 대문자로

// === 문자열 뒤집기 ===
let rev: String = s.chars().rev().collect();  
                  // chars() = 문자 iterator
                  // rev() = 역순으로
                  // collect() = String으로 모으기

// === 문자열 비교 ===
if s == "hello" {                     // == = 값 비교 (내용 비교)
    println!("match!");
}
```

### 8. Iterator 패턴 - 매우 중요!
```rust
let v = vec![1, 2, 3, 4, 5];

// === map (각 요소 변환) ===
let doubled: Vec<i32> = v.iter()      // iter() = 불변 참조 iterator
    .map(|x| x * 2)                   // map = 각 요소에 함수 적용
                                      // |x| = 클로저 (람다 함수)
    .collect();                       // collect = iterator를 Vec로

// === filter (조건 필터링) ===
let evens: Vec<i32> = v.iter()
    .filter(|x| *x % 2 == 0)          // filter = 조건 만족하는 것만
                                      // *x = 역참조 (x는 &i32니까)
                                      // % = 나머지 연산자
    .copied()                         // copied() = &i32를 i32로 복사
    .collect();

// === enumerate (인덱스 + 값) ===
for (i, val) in v.iter().enumerate() {  // enumerate() = (index, value) 반환
    println!("{}: {}", i, val);
}

// === 체이닝 (여러 연산 연결) ===
let result: i32 = v.iter()
    .filter(|x| **x > 2)              // ** = 이중 역참조 (&&i32 -> i32)
    .map(|x| x * 2)                   // 2보다 큰 것만
    .sum();                           // 2배로 만들고
                                      // 전부 합산
```

### 9. Option & Result (에러 처리)
```rust
// === Option (값이 있거나 없음) ===
fn find(v: &Vec<i32>, target: i32) -> Option<usize> {  
                     // Option<usize> = Some(index) 또는 None
                     // usize = 양수 정수 (배열 인덱스용)
    for (i, &val) in v.iter().enumerate() {
        if val == target {
            return Some(i);           // Some = 값이 있는 경우
        }
    }
    None                              // None = 값이 없는 경우
}

// === Option 사용 (match로 처리) ===
match find(&v, 5) {                   // match = 패턴 매칭
    Some(idx) => println!("Found at {}", idx),  // Some일 때
    None => println!("Not found"),              // None일 때
}

// === unwrap (간단하지만 위험) ===
let val = some_option.unwrap();       // unwrap = Some의 값 꺼내기
                                      // None이면 panic! (프로그램 죽음)
let val = some_option.unwrap_or(0);   // unwrap_or = None이면 기본값 사용
```

### 10. 입출력
```rust
use std::io;                          // io = input/output 모듈

// === 키보드 입력 받기 ===
let mut input = String::new();        // 빈 String 생성
io::stdin()                           // stdin = 표준 입력 (키보드)
    .read_line(&mut input)            // read_line = 한 줄 읽기
                                      // &mut = 가변 참조 (값 수정 가능)
    .unwrap();                        // unwrap = Result 처리

// === 문자열을 숫자로 변환 ===
let num: i32 = input.trim()           // trim = 앞뒤 공백 제거
    .parse()                          // parse = 문자열을 다른 타입으로
    .unwrap();                        // Result<i32, _> 처리

// === 출력 ===
println!("{}", result);               // {} = placeholder
println!("{:?}", vec);                // {:?} = 디버그 출력 (구조체 전체)
```

---

## 알고리즘 패턴 Cheat Sheet (주석 추가판)

### Pattern 1: Two Pointers
```rust
fn two_pointers(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut left = 0;                     // left = 왼쪽 포인터 (시작)
    let mut right = nums.len() - 1;       // right = 오른쪽 포인터 (끝)
                                          // len() - 1 = 마지막 인덱스
    
    while left < right {                  // 둘이 만날 때까지
        // 여기에 로직 작성
        // 보통 합/차이를 비교하고
        left += 1;                        // 왼쪽 전진
        right -= 1;                       // 오른쪽 후진
    }
    vec![]                                // 빈 벡터 반환 (예시)
}
```

### Pattern 2: HashMap 빈도수
```rust
use std::collections::HashMap;

fn count_frequency(arr: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();         // 빈 해시맵 생성
    for num in arr {                      // 배열 순회 (소유권 가져감)
        *map.entry(num).or_insert(0) += 1;  // 핵심 패턴!
                                            // entry = num의 Entry 가져오기
                                            // or_insert(0) = 없으면 0 삽입
                                            // * = 역참조해서 값 수정
                                            // += 1 = 카운트 증가
    }
    map                                   // 해시맵 반환
}
```

### Pattern 3: Sliding Window
```rust
fn max_subarray(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];            // 최댓값 초기화 (첫 요소)
    let mut current_sum = nums[0];        // 현재 윈도우 합

    for i in 1..nums.len() {              // 두 번째 요소부터
        current_sum = current_sum.max(0) + nums[i];  
                      // max(0) = 음수면 버리고 0으로
                      // 현재 수를 더함
                      // Kadane's Algorithm 핵심
        max_sum = max_sum.max(current_sum);  
                  // max() = 두 값 중 큰 값
    }
    max_sum                               // 최댓값 반환
}
```

### Pattern 4: 3Sum (정렬 + Two Pointers)
```rust
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();                 // 먼저 정렬 (필수!)
    let mut result = Vec::new();          // 결과 저장할 벡터
    
    for i in 0..nums.len() {              // 첫 번째 수 고정
        if i > 0 && nums[i] == nums[i-1] {  // 중복 건너뛰기 (1)
            continue;                     // 다음 반복으로
        }
        
        let mut left = i + 1;             // 두 번째 포인터
        let mut right = nums.len() - 1;   // 세 번째 포인터
        
        while left < right {              // Two Pointers 시작
            let sum = nums[i] + nums[left] + nums[right];  // 세 수 합
            
            if sum == 0 {                 // 합이 0이면 정답!
                result.push(vec![nums[i], nums[left], nums[right]]);
                       // push = 벡터에 추가
                       // vec! = 3개 원소 벡터 생성
                left += 1;
                right -= 1;
                
                // 중복 건너뛰기 (2)
                while left < right && nums[left] == nums[left-1] {
                    left += 1;
                }
            } else if sum < 0 {           // 합이 작으면
                left += 1;                // 왼쪽 증가 (합 키우기)
            } else {                      // 합이 크면
                right -= 1;               // 오른쪽 감소 (합 줄이기)
            }
        }
    }
    result                                // 모든 triplet 반환
}
```

### Pattern 5: Stack (Vec 활용)
```rust
fn valid_parentheses(s: String) -> bool {
    let mut stack = Vec::new();           // Vec를 stack으로 사용
    
    for ch in s.chars() {                 // 문자 하나씩 순회
        match ch {                        // 문자 종류에 따라
            '(' | '[' | '{' => stack.push(ch),  
                        // | = or, 여는 괄호면 push
            ')' => if stack.pop() != Some('(') { return false; },
                   // pop() = 마지막 요소 제거+반환
                   // Some('(') = Option으로 감싼 값
                   // 매칭 안 되면 false 반환
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}                       // 다른 문자는 무시
        }
    }
    stack.is_empty()                      // 스택 비었으면 true
                                          // 남았으면 false (짝 안 맞음)
}
```

---

매크로란? (1줄 요약)
"코드를 생성하는 코드" - 함수보다 더 강력하고 유연함

왜 알아야 하나?
rustprintln!("hello");  // ← 이게 매크로
vec![1, 2, 3];      // ← 이것도 매크로
Rust에서 ! 붙은 건 전부 매크로입니다.

왜 매크로를 쓸까?
이유 1: 반복 코드 줄이기
```rust
// vec! 없으면 매번 이렇게
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);

// vec! 있으면
let v = vec![1, 2, 3];  // 끝!
```


이유 2: 타입 무관하게 작동
```rust
// println!은 i32, String, f64 뭐든지 출력
println!("{}", 5);        // i32
println!("{}", "hi");     // &str
println!("{}", 3.14);     // f64
```