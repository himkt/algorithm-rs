pub fn mod_combination(n: usize, k: usize, div: usize) -> usize {
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
        fact[i] = fact[i-1] * i % div;
        inv[i] = div - inv[div % i] * (div / i) % div;
        finv[i] = finv[i-1] * inv[i] % div;
    }

    fact[n] * (finv[k] * finv[n - k] % div) % div
}


#[cfg(test)]
mod test_combination {
    #[test]
    fn it_works() {
        use crate::misc::combination::combination;
        {
            assert_eq!(combination(6, 0), 1);
            assert_eq!(combination(6, 1), 6);
            assert_eq!(combination(6, 2), 15);
            assert_eq!(combination(6, 3), 20);
            assert_eq!(combination(6, 4), 15);
            assert_eq!(combination(6, 5), 6);
            assert_eq!(combination(6, 6), 1);
        }
    }
}
