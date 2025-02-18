# Ord vs PartialOrd
- "total order"를 형성하는 타입을 위한 trait이 Ord라고 표준문서 첫 줄에 명시되어 있다.
- 쉽게 말해 Ord를 구현하는 타입은 "항상" 비교가 가능하고 PartialOrd만 구현하는 타입은 "종종" 비교가 가능하다.
- 각 트레이트에 포함된 비교 메서드의 시그니처를 비교해보면 더 명확하다.
    ```rust
    impl PartialOrd for f64
    // 항상 비교가 가능하지 않으므로 비교 불가능한 경우를 대비해 Option으로 Ordering을 감싸서 return
    fn partial_cmp(&self, other: &f64) -> Option<Ordering>
    
    impl Ord for i64
    fn cmp(&self, other: &i64) -> Ordering
    ```
- 또다른 예시로 f64는 Ord가 아닌 PartialOrd만 구현하고 있는데,  
이는 f64가 NaN의 값을 가지는 경우 비교 연산이 불가능  

    ```rust
    let a = f64::NAN;
    let b = 2.0_f64;
    
    assert_eq!(a.partial_cmp(&b), None);
    ```