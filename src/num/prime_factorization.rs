pub fn eratosthenes_sieve_smallest_prime_factors(n: usize) -> Vec<usize> {
    let mut smallest_prime_factors: Vec<usize> = (0..=n+1).into_iter().collect();
    let mut i = 2;

    while i*i <= n {
        if smallest_prime_factors[i] == i {
            let mut j = i;
            while j*i <= n {
                smallest_prime_factors[j*i] = i;
                j += 1;
            }
        }

        i += 1;
    }

    smallest_prime_factors
}


#[derive(Debug,Clone)]
pub struct SequentialPrimeFactorization {
    smallest_prime_factors: Vec<usize>,
}


impl SequentialPrimeFactorization {
    pub fn new(n: usize) -> Self {
        let smallest_prime_factors = eratosthenes_sieve_smallest_prime_factors(n);
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


#[cfg(test)]
mod test_eratosthenes_sieve {
    #[test]
    fn it_works () {
        use crate::num::prime_factorization::SequentialPrimeFactorization;
        let prime_factorizer = SequentialPrimeFactorization::new(100);

        assert_eq!(prime_factorizer.factorize(1), vec![]);
        assert_eq!(prime_factorizer.factorize(2), vec![2]);
        assert_eq!(prime_factorizer.factorize(4), vec![2, 2]);
        assert_eq!(prime_factorizer.factorize(7), vec![7]);
        assert_eq!(prime_factorizer.factorize(30), vec![2, 3, 5]);
        assert_eq!(prime_factorizer.factorize(23), vec![23]);
    }
}
