#[derive(Debug, Clone)]
pub struct SegmentTree {
    v: Vec<i64>,
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    RangeUpdate(Op),
    RangeGet(Op),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Max,
    Min,
    Add,
}

// Segment tree implementation. All operations are 0-origin.
// Note that a half-open interval [l, r) is used as a range representation.
impl SegmentTree {
    const SEQ_LEN: usize = 1 << 20;
    const MAX: i64 = 1_000_000_000_000;
    const MIN: i64 = -1_000_000_000_000;

    pub fn new(mode: Mode) -> Self {
        let default = match &mode {
            Mode::RangeGet(op) => match op {
                Op::Add => 0,
                Op::Max => SegmentTree::MIN,
                Op::Min => SegmentTree::MAX,
            },
            Mode::RangeUpdate(op) => match op {
                Op::Add => 0,
                Op::Max => SegmentTree::MIN,
                Op::Min => SegmentTree::MAX,
            },
        };

        Self {
            v: vec![default; 2 * SegmentTree::SEQ_LEN],
            mode,
        }
    }

    /// Get an i-th element of from the tree.
    pub fn get_one(&mut self, mut index: usize) -> i64 {
        index += SegmentTree::SEQ_LEN;

        let mut sum = 0;
        sum += self.v[index];

        while index > 0 {
            index /= 2;
            sum += self.v[index];
        }

        sum
    }

    /// Run a range query.
    pub fn get_range(&self, l: usize, r: usize) -> i64 {
        fn _get_range(
            op: &Op,
            v: &Vec<i64>,
            ql: usize,
            qr: usize,
            sl: usize,
            sr: usize,
            pos: usize,
        ) -> i64 {
            if qr <= sl || sr <= ql {
                return match &op {
                    Op::Add => 0,
                    Op::Max => SegmentTree::MIN,
                    Op::Min => SegmentTree::MAX,
                };
            }
            if ql <= sl && sr <= qr {
                return v[pos];
            }

            let sm = (sl + sr) / 2;
            let lv = _get_range(op, v, ql, qr, sl, sm, pos * 2);
            let rv = _get_range(op, v, ql, qr, sm, sr, pos * 2 + 1);

            match &op {
                Op::Add => lv + rv,
                Op::Max => lv.max(rv),
                Op::Min => lv.min(rv),
            }
        }

        if let Mode::RangeGet(op) = &self.mode {
            _get_range(op, &self.v, l, r, 0, SegmentTree::SEQ_LEN, 1)
        } else {
            panic!("Unsupported");
        }
    }

    /// Update an i-th element to `value`.
    pub fn update_one(&mut self, mut index: usize, value: i64) {
        index += SegmentTree::SEQ_LEN;

        match &self.mode {
            Mode::RangeGet(Op::Add) => {
                self.v[index] += value;
            }
            Mode::RangeGet(Op::Max) => {
                self.v[index] = value;
            }
            Mode::RangeGet(Op::Min) => {
                self.v[index] = value;
            }
            _ => panic!("Unsupported"),
        };

        if let Mode::RangeGet(op) = &self.mode {
            while index > 0 {
                index /= 2;
                let lv = self.v[index * 2];
                let rv = self.v[index * 2 + 1];

                match op {
                    Op::Add => {
                        self.v[index] = lv + rv;
                    }
                    Op::Max => {
                        self.v[index] = lv.max(rv);
                    }
                    Op::Min => {
                        self.v[index] = lv.min(rv);
                    }
                };
            }
        }
    }

    /// Add `value` to the range `[l, r)`.
    pub fn update_range(&mut self, mut l: usize, mut r: usize, value: i64) {
        if let Mode::RangeUpdate(op) = &self.mode {
            l += SegmentTree::SEQ_LEN;
            r += SegmentTree::SEQ_LEN;

            while l < r {
                if l % 2 == 1 {
                    match op {
                        Op::Add => {
                            self.v[l] += value;
                            l += 1;
                        }
                        _ => panic!("Unsupported"),
                    }
                }
                l /= 2;

                if r % 2 == 1 {
                    match op {
                        Op::Add => {
                            self.v[r - 1] += value;
                            r -= 1;
                        }
                        _ => panic!("Unsupported"),
                    }
                }
                r /= 2;
            }
        } else {
            panic!("Unsupported");
        }
    }
}

#[cfg(test)]
mod test_segment_tree {
    use crate::tree::segment_tree::Mode;
    use crate::tree::segment_tree::Op;
    use crate::tree::segment_tree::SegmentTree;

    #[test]
    fn it_works_raq() {
        let mut raq = SegmentTree::new(Mode::RangeUpdate(Op::Add));
        raq.update_range(1, 2, 1);
        raq.update_range(2, 4, 2);
        raq.update_range(3, 4, 3);
        assert_eq!(raq.get_one(0), 0);
        assert_eq!(raq.get_one(2), 2);
        assert_eq!(raq.get_one(3), 5);
    }

    #[test]
    fn it_works_rsq() {
        let mut rsq = SegmentTree::new(Mode::RangeGet(Op::Add));
        rsq.update_one(0, 3);
        rsq.update_one(2, 3);
        rsq.update_one(3, 1);
        rsq.update_one(4, 4);
        assert_eq!(rsq.get_range(0, 3), 6);
        assert_eq!(rsq.get_range(1, 3), 3);
        assert_eq!(rsq.get_range(2, 4), 4);
        assert_eq!(rsq.get_range(3, 4), 1);
        assert_eq!(rsq.get_range(1, 6), 8);
        assert_eq!(rsq.get_range(0, 0), 0);
    }

    #[test]
    fn it_works_rmaxq() {
        let mut rmaxq = SegmentTree::new(Mode::RangeGet(Op::Max));
        rmaxq.update_one(0, 10);
        rmaxq.update_one(2, 101);
        rmaxq.update_one(100, 1001);
        assert_eq!(rmaxq.get_range(0, 1), 10);
        assert_eq!(rmaxq.get_range(0, 2), 10);
        assert_eq!(rmaxq.get_range(0, 3), 101);
        assert_eq!(rmaxq.get_range(0, 100100), 1001);
        assert_eq!(rmaxq.get_range(101, 1000), SegmentTree::MIN);
        assert_eq!(rmaxq.get_range(0, 0), SegmentTree::MIN);
    }

    #[test]
    fn it_works_rminq() {
        let mut rminq = SegmentTree::new(Mode::RangeGet(Op::Min));
        rminq.update_one(0, 101);
        rminq.update_one(2, 10);
        rminq.update_one(100, 1001);
        assert_eq!(rminq.get_range(0, 1), 101);
        assert_eq!(rminq.get_range(0, 2), 101);
        assert_eq!(rminq.get_range(0, 3), 10);
        assert_eq!(rminq.get_range(0, 100100), 10);
        assert_eq!(rminq.get_range(101, 1000), SegmentTree::MAX);
        assert_eq!(rminq.get_range(0, 0), SegmentTree::MAX);
    }
}
