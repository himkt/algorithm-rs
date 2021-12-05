pub fn z_algorithm(s: &str) -> Vec<usize> {
    fn common_length(s: &str, t: &str) -> usize {
        let mut ans = 0;
        let mut cur = 0;

        let cs: Vec<char> = s.chars().collect();
        let ct: Vec<char> = t.chars().collect();

        while cur < ct.len() {
            if cs[cur] == ct[cur] {
                ans += 1;
            }
            else {
                break;
            }
            cur += 1;
        }

        ans
    }

    let n = s.len();
    let mut ans = vec![0; n];

    for i in 0..n {
        ans[i] = common_length(s, &s[i..]);
    }

    ans
}


#[cfg(test)]
mod test_z_algorithm {
    #[test]
    fn it_works() {
        use crate::string::z_algorithm::z_algorithm;

        {
            assert_eq!(z_algorithm("ISSISSIPPIM"), vec![11, 0, 0, 4, 0, 0, 1, 0, 0, 1, 0]);
        }
    }
}
