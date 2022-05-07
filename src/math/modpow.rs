pub fn modpow(a: usize, b: usize, m: usize) -> usize {
    let mut ans = 1;
    let mut p = a;

    for i in 0..30 {
        if b & (1 << i) != 0 {
            ans *= p;
            ans %= m;
        }

        p *= p;
        p %= m;
    }

    ans
}

#[cfg(test)]
mod test_modpow {
    #[test]
    fn it_works() {
        use crate::math::modpow::modpow;

        {
            assert_eq!(modpow(2, 10, 1_000_000_007), 1024);
            assert_eq!(modpow(2, 3, 1_000_000_007), 8);
            assert_eq!(modpow(5, 8, 1_000_000_007), 390625)
        }
    }
}
