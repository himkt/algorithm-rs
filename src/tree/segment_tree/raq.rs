#[derive(Debug, Clone)]
pub struct RAQ {
    v: Vec<i64>,
}

impl RAQ {
    const SEQ_LEN: usize = 1 << 18;
}

impl Default for RAQ {
    fn default() -> Self {
        RAQ::new()
    }
}

impl RAQ {
    pub fn new() -> Self {
        Self {
            v: vec![0; 2 * RAQ::SEQ_LEN],
        }
    }

    pub fn get(&mut self, mut index: usize) -> i64 {
        index += RAQ::SEQ_LEN - 1;

        let mut sum = 0;
        sum += self.v[index];

        while index > 0 {
            index /= 2;
            sum += self.v[index];
        }

        sum
    }

    pub fn add(&mut self, mut l: usize, mut r: usize, value: i64) {
        l += RAQ::SEQ_LEN - 1;
        r += RAQ::SEQ_LEN;

        while l < r {
            if l % 2 == 1 {
                self.v[l] += value;
                l += 1;
            }
            l /= 2;

            if r % 2 == 1 {
                self.v[r - 1] += value;
                r -= 1;
            }
            r /= 2;
        }
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RAQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RAQ::SEQ_LEN + index;
            rsq.v[idx] = value;
        }

        rsq
    }
}

#[cfg(test)]
mod test_segment_tree {
    use crate::tree::segment_tree::raq::RAQ;

    #[test]
    fn it_works() {
        let mut rsq = RAQ::new();
        rsq.add(1, 2, 1);
        rsq.add(2, 3, 2);
        rsq.add(3, 3, 3);
        assert_eq!(rsq.get(2), 3);
        assert_eq!(rsq.get(3), 5);
    }

    #[test]
    fn it_works_from() {
        let vs = vec![1i64; 1 << 2];
        let mut rsq = RAQ::from(vs);
        rsq.add(1, 2, 1);
        rsq.add(2, 3, 2);
        rsq.add(3, 3, 3);
        assert_eq!(rsq.get(2), 4);
        assert_eq!(rsq.get(3), 6);
    }
}
