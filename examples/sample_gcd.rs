use rust_lib::gcd::gcd;

fn main() {
    let a = 48;
    let b = 18;
    println!("gcd({}, {}) = {}", a, b, gcd(a, b));
}
