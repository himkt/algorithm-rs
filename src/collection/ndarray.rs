#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

#[cfg(test)]
mod test_ndarray {

    #[test]
    fn it_works() {
        // ndarray!(val; 1) => [val]
        assert_eq!(ndarray!(5; 1), vec![5]);
        // ndarray!(val; 1, 2) => [[val, val]]
        assert_eq!(ndarray!(5; 1, 2), vec![vec![5, 5]]);
        // ndarray!(val; 2, 1) => [[val], [val]]
        assert_eq!(ndarray!(5; 2, 1), vec![vec![5], vec![5]]);
    }
}
