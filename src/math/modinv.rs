use crate::math::modpow::modpow;

pub fn phi(mut n: usize) -> usize {
    let mut res = n;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            res = res / i * (i - 1);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 1;
    }

    if n != 1 {
        res = res / n * (n - 1);
    }

    res
}

pub fn modinv(a: usize, p: usize) -> usize {
    let m = phi(p);
    modpow(a, m - 1, p)
}

#[cfg(test)]
mod test_modinv {
    use crate::math::modinv::modinv;
    use crate::math::modpow::modpow;

    fn inv_euler(a: usize, p: usize) -> usize {
        modpow(a, p - 2, p)
    }

    fn inv_simple(a: usize, p: usize) -> usize {
        let mut ret = 1;
        loop {
            if (a * ret) % p == 1 {
                return ret;
            }
            ret += 1;
        }
    }

    #[test]
    fn it_works_prime() {
        // 10^-1 mod 7
        assert_eq!(modinv(10, 7), inv_euler(10, 7));
    }

    #[test]
    fn it_works_composite() {
        // 10^-1 mod 2019
        assert_eq!(modinv(10, 2019), inv_simple(10, 2019));
    }
}
