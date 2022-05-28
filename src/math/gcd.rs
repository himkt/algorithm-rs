pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod test_gcd {
    use crate::math::gcd::gcd;

    #[test]
    fn it_works() {
        assert_eq!(gcd(15, 5), 5);
        assert_eq!(gcd(5, 15), 5);
        assert_eq!(gcd(198, 26), 2);
        assert_eq!(gcd(26, 198), 2);
    }
}
