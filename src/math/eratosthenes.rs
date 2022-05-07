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
    #[test]
    fn it_works() {
        use crate::math::eratosthenes::eratosthenes_sieve;
        let primes = eratosthenes_sieve(10);

        println!("{:?}", primes);

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
