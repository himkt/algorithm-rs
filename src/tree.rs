use std::ops::Add;

pub trait Zero {
    fn zero() -> Self;
}

pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! impl_monoid_traits {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }
        impl Bounded for $t {
            fn min_value() -> Self {
                <$t>::MIN
            }
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

impl_monoid_traits!(i8);
impl_monoid_traits!(i16);
impl_monoid_traits!(i32);
impl_monoid_traits!(i64);
impl_monoid_traits!(i128);
impl_monoid_traits!(u8);
impl_monoid_traits!(u16);
impl_monoid_traits!(u32);
impl_monoid_traits!(u64);
impl_monoid_traits!(u128);
impl_monoid_traits!(isize);
impl_monoid_traits!(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Max,
    Min,
    Add,
}

#[derive(Debug, Clone)]
pub struct RangeGetTree<T> {
    data: Vec<T>,
    op: Op,
}

impl<T> RangeGetTree<T>
where
    T: Clone + Add<Output = T> + Ord + Zero + Bounded,
{
    const SEQ_LEN: usize = 1 << 20;

    pub fn new(op: Op) -> Self {
        let identity = Self::identity_for(&op);
        Self {
            data: vec![identity; 2 * Self::SEQ_LEN],
            op,
        }
    }

    fn identity_for(op: &Op) -> T {
        match op {
            Op::Add => T::zero(),
            Op::Max => T::min_value(),
            Op::Min => T::max_value(),
        }
    }

    pub fn identity(&self) -> T {
        Self::identity_for(&self.op)
    }

    fn operate(&self, a: T, b: T) -> T {
        match &self.op {
            Op::Add => a + b,
            Op::Max => a.max(b),
            Op::Min => a.min(b),
        }
    }

    pub fn get(&self, l: usize, r: usize) -> T {
        self.range_query_recursive(l, r, 0, Self::SEQ_LEN, 1)
    }

    pub fn update(&mut self, mut index: usize, value: T) {
        index += Self::SEQ_LEN;
        self.data[index] = self.operate(self.data[index].clone(), value);
        while index > 1 {
            index /= 2;
            let lv = self.data[index * 2].clone();
            let rv = self.data[index * 2 + 1].clone();
            self.data[index] = self.operate(lv, rv);
        }
    }

    fn range_query_recursive(&self, ql: usize, qr: usize, sl: usize, sr: usize, pos: usize) -> T {
        if qr <= sl || sr <= ql {
            return self.identity();
        }
        if ql <= sl && sr <= qr {
            return self.data[pos].clone();
        }
        let sm = (sl + sr) / 2;
        let lv = self.range_query_recursive(ql, qr, sl, sm, pos * 2);
        let rv = self.range_query_recursive(ql, qr, sm, sr, pos * 2 + 1);
        self.operate(lv, rv)
    }
}

#[cfg(test)]
mod test_range_get_tree {
    use super::{Op, RangeGetTree};

    #[test]
    fn it_works_rsq() {
        let mut rsq: RangeGetTree<i64> = RangeGetTree::new(Op::Add);
        rsq.update(0, 3);
        rsq.update(2, 3);
        rsq.update(3, 1);
        rsq.update(4, 4);
        assert_eq!(rsq.get(0, 3), 6);
        assert_eq!(rsq.get(1, 3), 3);
        assert_eq!(rsq.get(2, 4), 4);
        assert_eq!(rsq.get(3, 4), 1);
        assert_eq!(rsq.get(1, 6), 8);
        assert_eq!(rsq.get(0, 0), rsq.identity());
    }

    #[test]
    fn it_works_rmaxq() {
        let mut rmaxq: RangeGetTree<i64> = RangeGetTree::new(Op::Max);
        rmaxq.update(0, 10);
        rmaxq.update(2, 101);
        rmaxq.update(100, 1001);
        assert_eq!(rmaxq.get(0, 1), 10);
        assert_eq!(rmaxq.get(0, 2), 10);
        assert_eq!(rmaxq.get(0, 3), 101);
        assert_eq!(rmaxq.get(0, 100100), 1001);
        assert_eq!(rmaxq.get(0, 0), rmaxq.identity());
    }

    #[test]
    fn it_works_rminq() {
        let mut rminq: RangeGetTree<i64> = RangeGetTree::new(Op::Min);
        rminq.update(0, 101);
        rminq.update(2, 10);
        rminq.update(100, 1001);
        assert_eq!(rminq.get(0, 1), 101);
        assert_eq!(rminq.get(0, 2), 101);
        assert_eq!(rminq.get(0, 3), 10);
        assert_eq!(rminq.get(0, 100100), 10);
        assert_eq!(rminq.get(0, 0), rminq.identity());
    }
}

#[derive(Debug, Clone)]
pub struct RangeUpdateTree<T> {
    data: Vec<T>,
    op: Op,
}

impl<T> RangeUpdateTree<T>
where
    T: Clone + Add<Output = T> + Ord + Zero + Bounded,
{
    const SEQ_LEN: usize = 1 << 20;

    pub fn new(op: Op) -> Self {
        let identity = Self::identity_for(&op);
        Self {
            data: vec![identity; 2 * Self::SEQ_LEN],
            op,
        }
    }

    fn identity_for(op: &Op) -> T {
        match op {
            Op::Add => T::zero(),
            _ => panic!("Unsupported op for RangeUpdateTree: {:?}", op),
        }
    }

    pub fn identity(&self) -> T {
        Self::identity_for(&self.op)
    }

    fn operate(&self, a: T, b: T) -> T {
        match &self.op {
            Op::Add => a + b,
            _ => panic!("Unsupported op for RangeUpdateTree: {:?}", &self.op),
        }
    }

    pub fn get(&self, mut index: usize) -> T {
        index += Self::SEQ_LEN;
        let mut ret = self.identity();
        ret = self.operate(ret, self.data[index].clone());
        while index > 1 {
            index /= 2;
            ret = self.operate(ret, self.data[index].clone());
        }
        ret
    }

    pub fn update(&mut self, mut l: usize, mut r: usize, value: T) {
        l += Self::SEQ_LEN;
        r += Self::SEQ_LEN;
        while l < r {
            if l % 2 == 1 {
                self.data[l] = self.operate(self.data[l].clone(), value.clone());
                l += 1;
            }
            l /= 2;
            if r % 2 == 1 {
                self.data[r - 1] = self.operate(self.data[r - 1].clone(), value.clone());
                r -= 1;
            }
            r /= 2;
        }
    }
}

#[cfg(test)]
mod test_range_update_tree {
    use super::{Op, RangeUpdateTree};

    #[test]
    fn it_works_raq() {
        let mut raq: RangeUpdateTree<i64> = RangeUpdateTree::new(Op::Add);
        raq.update(1, 2, 1);
        raq.update(2, 4, 2);
        raq.update(3, 4, 3);
        assert_eq!(raq.get(0), raq.identity());
        assert_eq!(raq.get(2), 2);
        assert_eq!(raq.get(3), 5);
    }
}
