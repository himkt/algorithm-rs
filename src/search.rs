pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> Option<usize> {
    if prop(range.start) {
        return Some(range.start);
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

    match ok.eq(&range.end) {
        true  => None,
        false => Some(ok),
    }
}

#[cfg(test)]
mod test_lower_bound {
    use crate::search::lower_bound;

    #[test]
    fn it_works() {
        let vs: Vec<usize> = vec![1, 2, 3, 5, 7, 10];
        assert_eq!(lower_bound(0..vs.len(), &|x| 1 <= vs[x]), Some(0));
        assert_eq!(lower_bound(0..vs.len(), &|x| 2 <= vs[x]), Some(1));
        assert_eq!(lower_bound(0..vs.len(), &|x| 3 <= vs[x]), Some(2));
        assert_eq!(lower_bound(0..vs.len(), &|x| 7 <= vs[x]), Some(4));
        assert_eq!(lower_bound(0..vs.len(), &|x| 10 <= vs[x]), Some(5));
        assert_eq!(lower_bound(0..vs.len(), &|x| 100 <= vs[x]), None);
    }
}

pub fn upper_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> Option<usize> {
    if !prop(range.start) {
        return None;
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

    Some(ok)
}

#[cfg(test)]
mod test_upper_bound {
    use crate::search::upper_bound;

    #[test]
    fn it_works() {
        let vs: Vec<usize> = vec![1, 2, 3, 5, 7, 10];
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 1), None);
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 2), Some(0));
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 3), Some(1));
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 7), Some(3));
        assert_eq!(upper_bound(0..vs.len(), &|x| vs[x] < 10), Some(4));
    }
}
