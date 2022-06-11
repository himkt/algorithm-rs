#[derive(Debug, Clone)]
pub struct RSQ {
    v: Vec<i64>,
}

impl Default for RSQ {
    fn default() -> Self {
        RSQ::new()
    }
}

impl RSQ {
    const SEQ_LEN: usize = 1<<18;

    pub fn new() -> Self {
        Self {
            v: vec![0; 2 * RSQ::SEQ_LEN],
        }
    }

    /// Add `value` to i-th element.
    /// 0-origin.
    pub fn add(&mut self, mut index: usize, value: i64) {
        index += RSQ::SEQ_LEN;
        self.v[index] += value;

        while index > 0 {
            index /= 2;
            self.v[index] = self.v[index * 2] + self.v[index * 2 + 1];
        }
    }

    /// Sum values on `[l, r)`. Note that it is a half-open interval.
    /// 0-origin.
    pub fn sum(&self, l: usize, r: usize) -> i64 {
        self._sum(l, r, 0, RSQ::SEQ_LEN, 1)
    }

    fn _sum(&self, ql: usize, qr: usize, sl: usize, sr: usize, pos: usize) -> i64 {
        if qr <= sl || sr <= ql {
            return 0;
        }
        if ql <= sl && sr <= qr {
            return self.v[pos];
        }

        let sm = (sl + sr) / 2;
        let lv = self._sum(ql, qr, sl, sm, pos * 2);
        let rv = self._sum(ql, qr, sm, sr, pos * 2 + 1);
        lv + rv
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
        rsq.add(0, 3);
        rsq.add(2, 3);
        rsq.add(3, 1);
        rsq.add(4, 4);
        assert_eq!(rsq.sum(0, 3), 6);
        assert_eq!(rsq.sum(1, 3), 3);
        assert_eq!(rsq.sum(2, 4), 4);
        assert_eq!(rsq.sum(3, 4), 1);
        assert_eq!(rsq.sum(1, 6), 8);
    }

    #[test]
    fn it_works_from() {
        let vs = vec![1i64; 1 << 3];
        let mut rsq = RSQ::from(vs);
        rsq.add(2, 3);
        rsq.add(3, 1);
        rsq.add(4, 4);
        assert_eq!(rsq.sum(1, 3), 5);
        assert_eq!(rsq.sum(2, 4), 6);
        assert_eq!(rsq.sum(3, 4), 2);
        assert_eq!(rsq.sum(1, 6), 13);
    }
}
