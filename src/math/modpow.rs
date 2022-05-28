pub fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut ans = 1;

    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % m;
        }

        a = a * a % m;
        n >>= 1;
    }

    ans
}

#[cfg(test)]
mod test_modpow {
    use crate::math::modpow::modpow;

    #[test]
    fn it_works() {
        assert_eq!(modpow(2, 10, 1_000_000_007), 1024);
        assert_eq!(modpow(2, 3, 1_000_000_007), 8);
        assert_eq!(modpow(5, 8, 1_000_000_007), 390625);
        assert_eq!(modpow(2, 2, 3), 1);
        assert_eq!(modpow(2, 3, 3), 2);
    }
}
