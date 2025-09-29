/// 2つの非負整数の最大公約数（GCD）を求めます。
///
/// この関数は [ユークリッドの互除法](https://ja.wikipedia.org/wiki/%E4%BA%92%E9%99%A4%E6%B3%95) を使って計算します。
///
/// # 使用例
///
/// ```
/// use your_repo::gcd::gcd;
///
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(100, 25), 25);
/// ```
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
