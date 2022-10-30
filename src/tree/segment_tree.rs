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
            Mode::RangeGet(op) => SegmentTree::default(op),
            Mode::RangeUpdate(op) => SegmentTree::default(op),
        };

        Self {
            v: vec![default; 2 * SegmentTree::SEQ_LEN],
            mode,
        }
    }

    /// Return an appropriate default value for the given operation.
    pub fn default(op: &Op) -> i64 {
        match op {
            Op::Add => 0,
            Op::Max => SegmentTree::MIN,
            Op::Min => SegmentTree::MAX,
        }
    }

    /// Get an i-th element of from the tree.
    pub fn get_one(&mut self, mut index: usize) -> i64 {
        index += SegmentTree::SEQ_LEN;
        let mut ret = 0;

        if let Mode::RangeUpdate(op) = &self.mode {
            let operator = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                _ => panic!(),
            };

            operator(&mut ret, self.v[index]);
            while index > 0 {
                index /= 2;
                operator(&mut ret, self.v[index]);
            }
        } else {
            panic!("Unsupported");
        }

        ret
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
                return SegmentTree::default(op);
            }

            if ql <= sl && sr <= qr {
                return v[pos];
            }

            let sm = (sl + sr) / 2;
            let lv = _get_range(op, v, ql, qr, sl, sm, pos * 2);
            let rv = _get_range(op, v, ql, qr, sm, sr, pos * 2 + 1);
            let operate = match op {
                Op::Add => |l: i64, r: i64| l + r,
                Op::Max => |l: i64, r: i64| l.max(r),
                Op::Min => |l: i64, r: i64| l.min(r),
            };
            operate(lv, rv)
        }

        if let Mode::RangeGet(op) = &self.mode {
            let data = &self.v;
            _get_range(op, data, l, r, 0, SegmentTree::SEQ_LEN, 1)
        } else {
            panic!("Unsupported");
        }
    }

    /// Update an i-th element to `value`.
    pub fn update_one(&mut self, mut index: usize, value: i64) {
        index += SegmentTree::SEQ_LEN;

        if let Mode::RangeGet(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                Op::Max => |ret: &mut i64, v: i64| *ret = v,
                Op::Min => |ret: &mut i64, v: i64| *ret = v,
            };
            operate_and_assign_one(&mut self.v[index], value);

            let operate_and_assign = match op {
                Op::Add => |ret: &mut i64, l: i64, r: i64| *ret = l + r,
                Op::Max => |ret: &mut i64, l: i64, r: i64| *ret = l.max(r),
                Op::Min => |ret: &mut i64, l: i64, r: i64| *ret = l.min(r),
            };

            while index > 0 {
                index /= 2;
                let lv = self.v[index * 2];
                let rv = self.v[index * 2 + 1];
                operate_and_assign(&mut self.v[index], lv, rv);
            }
        }
    }

    /// Add `value` to the range `[l, r)`.
    pub fn update_range(&mut self, mut l: usize, mut r: usize, value: i64) {
        if let Mode::RangeUpdate(op) = &self.mode {
            l += SegmentTree::SEQ_LEN;
            r += SegmentTree::SEQ_LEN;

            let operate_and_assign_one = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                _ => panic!(),
            };

            while l < r {
                if l % 2 == 1 {
                    operate_and_assign_one(&mut self.v[l], value);
                    l += 1;
                }
                l /= 2;

                if r % 2 == 1 {
                    operate_and_assign_one(&mut self.v[r - 1], value);
                    r -= 1;
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
