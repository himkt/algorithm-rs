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
    for i in 1..(r+1) {
        res *= n - i + 1;
        res /= i;
    }

    res
}


#[cfg(test)]
mod test_combination {
    #[test]
    fn it_works() {
        use crate::misc::com::com;
        {
            assert_eq!(com(6, 0), 1);
            assert_eq!(com(6, 1), 6);
            assert_eq!(com(6, 2), 15);
            assert_eq!(com(6, 3), 20);
            assert_eq!(com(6, 4), 15);
            assert_eq!(com(6, 5), 6);
            assert_eq!(com(6, 6), 1);
        }
    }
}
