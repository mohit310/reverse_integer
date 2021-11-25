fn main() {
    println!("Hello, world!");
}

pub fn reverse(x: i32) -> i32 {
    let mut x = x.clone();
    let mut rev = 0;
    while x != 0 {
        let pop = x % 10;
        x /= 10;
        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) { return 0; }
        if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) { return 0; }
        rev = rev * 10 + pop;
    }
    rev
}

#[test]
fn test_rev_int() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(0), 0);
}
