macro_rules! test_all_inner {
    ($fsort_specific:ident, $fsort_generic:ident, 0) => {
        assert_eq!($fsort_specific::<i64>([]), []);
        assert_eq!($fsort_generic::<i64>([]), []);
    };
    ($fsort_specific:ident, $fsort_generic:ident, $n:expr) => {
        use itertools::Itertools;
        use std::convert::TryInto;

        for perm in (0i64..$n).permutations($n) {
            let input: [i64; $n] = (&perm[..]).clone().try_into().unwrap();

            let expected = {
                let mut tmp = input.clone();
                tmp.sort();
                tmp
            };

            let actual_specific = {
                let mut tmp = input.clone();
                $fsort_specific(&mut tmp);
                tmp
            };

            let actual_generic = {
                let mut tmp = input.clone();
                $fsort_generic(&mut tmp);
                tmp
            };

            assert_eq!(actual_specific, expected);
            assert_eq!(actual_generic, expected);
        }
    };
}

macro_rules! test_all {
    ($fsort:ident, $n:expr) => {
        #[test]
        fn $fsort() {
            use sortnet::{sortnet, $fsort};
            test_all_inner!($fsort, sortnet, $n);
        }
    };
}

test_all!(sortnet0, 0);
test_all!(sortnet1, 1);
test_all!(sortnet2, 2);
test_all!(sortnet3, 3);
test_all!(sortnet4, 4);
test_all!(sortnet5, 5);
test_all!(sortnet6, 6);
test_all!(sortnet7, 7);
test_all!(sortnet8, 8);
test_all!(sortnet9, 9);
test_all!(sortnet10, 10);
test_all!(sortnet11, 11);
// test_all!(sortnet12, 12);
// test_all!(sortnet13, 13);
// test_all!(sortnet14, 14);
// test_all!(sortnet15, 15);
// test_all!(sortnet16, 16);
