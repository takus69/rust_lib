use rust_lib::gcd::gcd;

#[test]
fn test_basic_cases() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(100, 25), 25);
    assert_eq!(gcd(17, 13), 1);
}

#[test]
fn test_zero_cases() {
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(0, 0), 0); // 定義によるけど、ここは0にしてるよ
}
