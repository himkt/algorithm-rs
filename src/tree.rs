#[derive(Debug, Clone)]
pub struct SegmentTree {
    data: Vec<i64>,
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
            data: vec![default; 2 * SegmentTree::SEQ_LEN],
            mode,
        }
    }

    pub fn default(op: &Op) -> i64 {
        match op {
            Op::Add => 0,
            Op::Max => SegmentTree::MIN,
            Op::Min => SegmentTree::MAX,
        }
    }

    pub fn get_one(&mut self, mut index: usize) -> i64 {
        index += SegmentTree::SEQ_LEN;
        let mut ret = 0;

        if let Mode::RangeUpdate(op) = &self.mode {
            let operator = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                _ => panic!(),
            };

            operator(&mut ret, self.data[index]);
            while index > 0 {
                index /= 2;
                operator(&mut ret, self.data[index]);
            }
        } else {
            panic!("Unsupported");
        }

        ret
    }

    fn range_query_recursive(
        &self,
        op: &Op,
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
            return self.data[pos];
        }

        fn add(l: i64, r: i64) -> i64 {
            l + r
        }
        fn max(l: i64, r: i64) -> i64 {
            l.max(r)
        }
        fn min(l: i64, r: i64) -> i64 {
            l.min(r)
        }

        let sm = (sl + sr) / 2;
        let lv = self.range_query_recursive(op, ql, qr, sl, sm, pos * 2);
        let rv = self.range_query_recursive(op, ql, qr, sm, sr, pos * 2 + 1);
        let operate = match op {
            Op::Add => add,
            Op::Max => max,
            Op::Min => min,
        };
        operate(lv, rv)
    }

    pub fn get_range(&self, l: usize, r: usize) -> i64 {
        if let Mode::RangeGet(op) = &self.mode {
            self.range_query_recursive(op, l, r, 0, SegmentTree::SEQ_LEN, 1)
        } else {
            panic!("Unsupported");
        }
    }

    pub fn update_one(&mut self, mut index: usize, value: i64) {
        index += SegmentTree::SEQ_LEN;

        fn add_assign_one(ret: &mut i64, v: i64) {
            *ret += v;
        }
        fn max_assign_one(ret: &mut i64, v: i64) {
            *ret = v;
        }
        fn min_assign_one(ret: &mut i64, v: i64) {
            *ret = v;
        }
        fn add_assign(ret: &mut i64, l: i64, r: i64) {
            *ret = l + r;
        }
        fn max_assign(ret: &mut i64, l: i64, r: i64) {
            *ret = l.max(r);
        }
        fn min_assign(ret: &mut i64, l: i64, r: i64) {
            *ret = l.min(r);
        }

        if let Mode::RangeGet(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => add_assign_one,
                Op::Max => max_assign_one,
                Op::Min => min_assign_one,
            };
            operate_and_assign_one(&mut self.data[index], value);

            let operate_and_assign = match op {
                Op::Add => add_assign,
                Op::Max => max_assign,
                Op::Min => min_assign,
            };

            while index > 0 {
                index /= 2;
                let lv = self.data[index * 2];
                let rv = self.data[index * 2 + 1];
                operate_and_assign(&mut self.data[index], lv, rv);
            }
        }
    }

    pub fn update_range(&mut self, mut l: usize, mut r: usize, value: i64) {
        if let Mode::RangeUpdate(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                _ => panic!(),
            };

            l += SegmentTree::SEQ_LEN;
            r += SegmentTree::SEQ_LEN;

            while l < r {
                if l % 2 == 1 {
                    operate_and_assign_one(&mut self.data[l], value);
                    l += 1;
                }
                l /= 2;

                if r % 2 == 1 {
                    operate_and_assign_one(&mut self.data[r - 1], value);
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
    use crate::tree::Mode;
    use crate::tree::Op;
    use crate::tree::SegmentTree;

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
