pub fn com(n: i64, mut r: i64) -> i64 {
    if r > n {
        return 0;
    }

    if r * 2 > n {
        r = n - r;
    }

    if r == 0 {
        return 1;
    }

    let mut res = 1;
    for i in 1..(r + 1) {
        res *= n - i + 1;
        res /= i;
    }

    res
}

#[cfg(test)]
mod test_com {
    use crate::math::com;

    #[test]
    fn it_works() {
        assert_eq!(com(6, 0), 1);
        assert_eq!(com(6, 1), 6);
        assert_eq!(com(6, 2), 15);
        assert_eq!(com(6, 3), 20);
        assert_eq!(com(6, 4), 15);
        assert_eq!(com(6, 5), 6);
        assert_eq!(com(6, 6), 1);
    }
}

pub fn eratosthenes_sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n];
    s[0] = false;
    s[1] = false;

    for i in 2..n {
        if i * i > n {
            break;
        }
        if s[i] {
            for k in 2..(n + i - 1) / i {
                s[k * i] = false
            }
        }
    }

    s
}

#[cfg(test)]
mod test_eratosthenes_sieve {
    use crate::math::eratosthenes_sieve;

    #[test]
    fn it_works() {
        let primes = eratosthenes_sieve(10);

        assert!(!primes[1]);
        assert!(primes[2]);
        assert!(primes[3]);
        assert!(!primes[4]);
        assert!(primes[5]);
        assert!(!primes[6]);
        assert!(primes[7]);
        assert!(!primes[8]);
        assert!(!primes[9]);
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod test_gcd {
    use crate::math::gcd;

    #[test]
    fn it_works() {
        assert_eq!(gcd(15, 5), 5);
        assert_eq!(gcd(5, 15), 5);
        assert_eq!(gcd(198, 26), 2);
        assert_eq!(gcd(26, 198), 2);
    }
}

pub fn modcom(n: usize, k: usize, div: usize) -> usize {
    let mut fact = Vec::<usize>::new();
    let mut inv = Vec::<usize>::new();
    let mut finv = Vec::<usize>::new();

    let upper = n + 1;

    fact.resize(upper, 0);
    inv.resize(upper, 0);
    finv.resize(upper, 0);

    fact[0] = 1;
    fact[1] = 1;

    finv[0] = 1;
    finv[1] = 1;

    inv[1] = 1;

    for i in 2..upper {
        fact[i] = fact[i - 1] * i % div;
        inv[i] = div - inv[div % i] * (div / i) % div;
        finv[i] = finv[i - 1] * inv[i] % div;
    }

    fact[n] * (finv[k] * finv[n - k] % div) % div
}

#[cfg(test)]
mod test_modcom {
    use crate::math::modcom;

    #[test]
    fn it_works() {
        assert_eq!(modcom(6, 0, 7), 1);
        assert_eq!(modcom(6, 1, 7), 6);
        assert_eq!(modcom(6, 2, 7), 1);
        assert_eq!(modcom(6, 3, 7), 6);
        assert_eq!(modcom(6, 4, 7), 1);
        assert_eq!(modcom(6, 5, 7), 6);
        assert_eq!(modcom(6, 6, 7), 1);
    }
}

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
    use crate::math::modpow;

    #[test]
    fn it_works() {
        assert_eq!(modpow(2, 10, 1_000_000_007), 1024);
        assert_eq!(modpow(2, 3, 1_000_000_007), 8);
        assert_eq!(modpow(5, 8, 1_000_000_007), 390625);
        assert_eq!(modpow(2, 2, 3), 1);
        assert_eq!(modpow(2, 3, 3), 2);
    }
}

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
    use crate::math::modinv;
    use crate::math::modpow;

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

#[derive(Debug, Clone)]
pub struct SequentialPrimeFactorization {
    smallest_prime_factors: Vec<usize>,
}

impl SequentialPrimeFactorization {
    pub fn new(n: usize) -> Self {
        let mut smallest_prime_factors: Vec<usize> = (0..=(n + 1)).collect();
        let mut i = 2;

        while i * i <= n {
            if smallest_prime_factors[i] == i {
                let mut j = i;
                while j * i <= n {
                    smallest_prime_factors[j * i] = i;
                    j += 1;
                }
            }

            i += 1;
        }

        Self { smallest_prime_factors }
    }

    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = vec![];
        while x != 1 {
            ret.push(self.smallest_prime_factors[x]);
            x /= self.smallest_prime_factors[x];
        }

        ret.sort_unstable();
        ret
    }
}

pub fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            let mut k = 0;
            while n % p == 0 {
                k += 1;
                n /= p;
            }
            ret.push((p, k));
        }

        p += 1;
    }

    if n != 1 {
        ret.push((n, 1));
    }
    ret
}

#[cfg(test)]
mod test_prime_factorization {
    use crate::math::prime_factorize;
    use crate::math::SequentialPrimeFactorization;

    #[test]
    fn it_works_sequential_prime_factorization() {
        let prime_factorizer = SequentialPrimeFactorization::new(100);

        assert_eq!(prime_factorizer.factorize(1), vec![]);
        assert_eq!(prime_factorizer.factorize(2), vec![2]);
        assert_eq!(prime_factorizer.factorize(4), vec![2, 2]);
        assert_eq!(prime_factorizer.factorize(7), vec![7]);
        assert_eq!(prime_factorizer.factorize(30), vec![2, 3, 5]);
        assert_eq!(prime_factorizer.factorize(23), vec![23]);
    }

    #[test]
    fn it_works_prime_factorization() {
        assert_eq!(prime_factorize(1), vec![]);
        assert_eq!(prime_factorize(2), vec![(2, 1)]);
        assert_eq!(prime_factorize(4), vec![(2, 2)]);
        assert_eq!(prime_factorize(7), vec![(7, 1)]);
        assert_eq!(prime_factorize(30), vec![(2, 1), (3, 1), (5, 1)]);
        assert_eq!(prime_factorize(23), vec![(23, 1)]);
        assert_eq!(prime_factorize(512), vec![(2, 9)]);
    }
}
