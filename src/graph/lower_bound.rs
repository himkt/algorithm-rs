pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) { return range.start; }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => { ok = middle },
            false => { ng = middle },
        }
    }

    ok
}


#[cfg(test)]
mod test_lower_bound {
    #[test]
    fn it_works() {
        use crate::search::lower_bound::lower_bound;
        {
            let vs: Vec<usize> = vec![0, 1, 2, 3, 5, 7, 10];
            assert_eq!(lower_bound(0..vs.len(), &|x| 1 <= vs[x]), 1);
            assert_eq!(lower_bound(0..vs.len(), &|x| 3 <= vs[x]), 3);
            assert_eq!(lower_bound(0..vs.len(), &|x| 7 <= vs[x]), 5);
        }
    }
}
