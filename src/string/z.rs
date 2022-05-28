#[allow(clippy::needless_range_loop)]
pub fn z(s: &str) -> Vec<usize> {
    let len = s.len();
    let s: Vec<char> = s.chars().collect();

    let mut ans = vec![0; len];
    ans[0] = len;

    let mut i = 1;
    let mut j = 0;

    while i < len {
        while i + j < len && s[j] == s[i + j] {
            j += 1
        }
        ans[i] = j;

        if j == 0 {
            i += 1;
            continue;
        }

        let mut k = 1;
        while i + k < len && k + ans[k] < j {
            ans[i + k] = ans[k];
            k += 1
        }

        i += k;
        j -= k;
    }

    ans
}


#[cfg(test)]
mod test_z_algorithm {
    use crate::string::z::z;

    #[test]
    fn it_works() {

        assert_eq!(z("ISSISSIPPIM"), vec![11, 0, 0, 4, 0, 0, 1, 0, 0, 1, 0]);
    }
}
