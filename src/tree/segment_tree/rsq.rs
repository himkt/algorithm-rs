#[derive(Debug, Clone)]
pub struct RSQ {
    v: Vec<i64>,
}

impl RSQ {
    const SEQ_LEN: usize = 1 << 18;
}

impl Default for RSQ {
    fn default() -> Self {
        RSQ::new()
    }
}

impl RSQ {
    pub fn new() -> Self {
        Self {
            v: vec![0; 2 * RSQ::SEQ_LEN],
        }
    }

    pub fn add(&mut self, mut index: usize, value: i64) {
        index += RSQ::SEQ_LEN - 1;
        self.v[index] += value;

        while index > 0 {
            index /= 2;
            self.v[index] = self.v[index * 2] + self.v[index * 2 + 1];
        }
    }

    pub fn sum(&self, mut l: usize, mut r: usize) -> i64 {
        l += RSQ::SEQ_LEN - 1;
        r += RSQ::SEQ_LEN;

        let mut ans = 0;

        while l < r {
            if l % 2 == 1 {
                ans += self.v[l];
                l += 1;
            }
            l /= 2;

            if r % 2 == 1 {
                ans += self.v[r - 1];
                r -= 1;
            }
            r /= 2;
        }

        ans
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RSQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RSQ::SEQ_LEN + index;
            rsq.v[idx] = value;
        }

        rsq
    }
}

#[cfg(test)]
mod test_segment_tree {
    use crate::tree::segment_tree::rsq::RSQ;

    #[test]
    fn it_works() {
        let mut rsq = RSQ::new();
        rsq.add(2, 3);
        rsq.add(3, 1);
        rsq.add(4, 4);
        assert_eq!(rsq.sum(1, 2), 3);
        assert_eq!(rsq.sum(2, 3), 4);
        assert_eq!(rsq.sum(3, 3), 1);
        assert_eq!(rsq.sum(1, 5), 8);
    }

    #[test]
    fn it_works_from() {
        let vs = vec![1i64; 1 << 3];
        let mut rsq = RSQ::from(vs);
        rsq.add(2, 3);
        rsq.add(3, 1);
        rsq.add(4, 4);
        assert_eq!(rsq.sum(1, 2), 5);
        assert_eq!(rsq.sum(2, 3), 6);
        assert_eq!(rsq.sum(3, 3), 2);
        assert_eq!(rsq.sum(1, 5), 13);
    }
}
