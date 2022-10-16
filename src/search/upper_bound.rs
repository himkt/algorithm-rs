pub fn upper_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if !prop(range.start) {
        return range.end;
    }

    let mut ok = range.start;
    let mut ng = range.end;

    while ng - ok > 1 {
        let middle = ok + (ng - ok) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

#[cfg(test)]
mod test_upper_bound {
    use crate::search::upper_bound::upper_bound;

    #[test]
    fn it_works() {
        let vs: Vec<usize> = vec![1, 2, 3, 5, 7, 10];
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 1), vs.len());
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 2), 0);
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 3), 1);
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 7), 3);
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 10), 4);
    }
}
