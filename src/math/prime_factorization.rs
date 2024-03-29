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

        Self {
            smallest_prime_factors,
        }
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
    use crate::math::prime_factorization::prime_factorize;
    use crate::math::prime_factorization::SequentialPrimeFactorization;

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
