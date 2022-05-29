pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

#[cfg(test)]
mod test_lower_bound {
    use crate::search::lower_bound::lower_bound;

    #[test]
    fn it_works() {
        let vs: Vec<usize> = vec![1, 2, 3, 5, 7, 10];
        assert_eq!(lower_bound(0..vs.len(), &|x| 1 <= vs[x]), 0);
        assert_eq!(lower_bound(0..vs.len(), &|x| 2 <= vs[x]), 1);
        assert_eq!(lower_bound(0..vs.len(), &|x| 3 <= vs[x]), 2);
        assert_eq!(lower_bound(0..vs.len(), &|x| 7 <= vs[x]), 4);
        assert_eq!(lower_bound(0..vs.len(), &|x| 10 <= vs[x]), 5);
        assert_eq!(lower_bound(0..vs.len(), &|x| 100 <= vs[x]), vs.len());
    }
}
