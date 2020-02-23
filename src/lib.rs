use arrayref::array_mut_ref;

macro_rules! gen_sortnet {
    ($fsort:ident, $n:expr, []) => {
        #[inline(always)]
        pub fn $fsort<T>(_arr: &mut [T; $n])
        where
            T: PartialOrd,
        {}
    };

    ($fsort:ident, $n:expr, [$([$(($swap_a:expr, $swap_b:expr)),+]),*]) => {
        pub fn $fsort<T>(arr: &mut [T; $n])
        where
            T: PartialOrd,
        {
            #[inline(always)]
            pub fn maybe_swap<T>(arr: &mut [T; $n], a: usize, b: usize)
            where
                T: PartialOrd,
            {
                if arr[a] > arr[b] {
                    arr.swap(a, b)
                }
            }

            let mut arr = arr;
            $($(maybe_swap(&mut arr, $swap_a, $swap_b);)+)*
        }
    };
}

// see http://pages.ripco.net/~jgamble/nw.html
gen_sortnet!(sortnet0, 0, []);
gen_sortnet!(sortnet1, 1, []);
gen_sortnet!(sortnet2, 2, [[(0, 1)]]);
gen_sortnet!(sortnet3, 3, [[(1, 2)], [(0, 2)], [(0, 1)]]);
gen_sortnet!(sortnet4, 4, [[(0, 1), (2, 3)], [(0, 2), (1, 3)], [(1, 2)]]);
gen_sortnet!(
    sortnet5,
    5,
    [
        [(0, 1), (3, 4)],
        [(2, 4)],
        [(2, 3), (1, 4)],
        [(0, 3)],
        [(0, 2), (1, 3)],
        [(1, 2)]
    ]
);
gen_sortnet!(
    sortnet6,
    6,
    [
        [(1, 2), (4, 5)],
        [(0, 2), (3, 5)],
        [(0, 1), (3, 4), (2, 5)],
        [(0, 3), (1, 4)],
        [(2, 4), (1, 3)],
        [(2, 3)]
    ]
);
gen_sortnet!(
    sortnet7,
    7,
    [
        [(1, 2), (3, 4), (5, 6)],
        [(0, 2), (3, 5), (4, 6)],
        [(0, 1), (4, 5), (2, 6)],
        [(0, 4), (1, 5)],
        [(0, 3), (2, 5)],
        [(1, 3), (2, 4)],
        [(2, 3)]
    ]
);
gen_sortnet!(
    sortnet8,
    8,
    [
        [(0, 1), (2, 3), (4, 5), (6, 7)],
        [(0, 2), (1, 3), (4, 6), (5, 7)],
        [(1, 2), (5, 6), (0, 4), (3, 7)],
        [(1, 5), (2, 6)],
        [(1, 4), (3, 6)],
        [(2, 4), (3, 5)],
        [(3, 4)]
    ]
);
gen_sortnet!(
    sortnet9,
    9,
    [
        [(0, 1), (3, 4), (6, 7)],
        [(1, 2), (4, 5), (7, 8)],
        [(0, 1), (3, 4), (6, 7), (2, 5)],
        [(0, 3), (1, 4), (5, 8)],
        [(3, 6), (4, 7), (2, 5)],
        [(0, 3), (1, 4), (5, 7), (2, 6)],
        [(1, 3), (4, 6)],
        [(2, 4), (5, 6)],
        [(2, 3)]
    ]
);
gen_sortnet!(
    sortnet10,
    10,
    [
        [(4, 9), (3, 8), (2, 7), (1, 6), (0, 5)],
        [(1, 4), (6, 9), (0, 3), (5, 8)],
        [(0, 2), (3, 6), (7, 9)],
        [(0, 1), (2, 4), (5, 7), (8, 9)],
        [(1, 2), (4, 6), (7, 8), (3, 5)],
        [(2, 5), (6, 8), (1, 3), (4, 7)],
        [(2, 3), (6, 7)],
        [(3, 4), (5, 6)],
        [(4, 5)]
    ]
);
gen_sortnet!(
    sortnet11,
    11,
    [
        [(0, 1), (2, 3), (4, 5), (6, 7), (8, 9)],
        [(1, 3), (5, 7), (0, 2), (4, 6), (8, 10)],
        [(1, 2), (5, 6), (9, 10), (0, 4), (3, 7)],
        [(1, 5), (6, 10), (4, 8)],
        [(5, 9), (2, 6), (0, 4), (3, 8)],
        [(1, 5), (6, 10), (2, 3), (8, 9)],
        [(1, 4), (7, 10), (3, 5), (6, 8)],
        [(2, 4), (7, 9), (5, 6)],
        [(3, 4), (7, 8)]
    ]
);
gen_sortnet!(
    sortnet12,
    12,
    [
        [(0, 1), (2, 3), (4, 5), (6, 7), (8, 9), (10, 11)],
        [(1, 3), (5, 7), (9, 11), (0, 2), (4, 6), (8, 10)],
        [(1, 2), (5, 6), (9, 10), (0, 4), (7, 11)],
        [(1, 5), (6, 10), (3, 7), (4, 8)],
        [(5, 9), (2, 6), (0, 4), (7, 11), (3, 8)],
        [(1, 5), (6, 10), (2, 3), (8, 9)],
        [(1, 4), (7, 10), (3, 5), (6, 8)],
        [(2, 4), (7, 9), (5, 6)],
        [(3, 4), (7, 8)]
    ]
);
gen_sortnet!(
    sortnet13,
    13,
    [
        [(1, 7), (9, 11), (3, 4), (5, 8), (0, 12), (2, 6)],
        [(0, 1), (2, 3), (4, 6), (8, 11), (7, 12), (5, 9)],
        [(0, 2), (3, 7), (10, 11), (1, 4), (6, 12)],
        [(7, 8), (11, 12), (4, 9), (6, 10)],
        [(3, 4), (5, 6), (8, 9), (10, 11), (1, 7)],
        [(2, 6), (9, 11), (1, 3), (4, 7), (8, 10), (0, 5)],
        [(2, 5), (6, 8), (9, 10)],
        [(1, 2), (3, 5), (7, 8), (4, 6)],
        [(2, 3), (4, 5), (6, 7), (8, 9)],
        [(3, 4), (5, 6)]
    ]
);
gen_sortnet!(
    sortnet14,
    14,
    [
        [(0, 1), (2, 3), (4, 5), (6, 7), (8, 9), (10, 11), (12, 13)],
        [(0, 2), (4, 6), (8, 10), (1, 3), (5, 7), (9, 11)],
        [(0, 4), (8, 12), (1, 5), (9, 13), (2, 6), (3, 7)],
        [(0, 8), (1, 9), (2, 10), (3, 11), (4, 12), (5, 13)],
        [(5, 10), (6, 9), (3, 12), (7, 11), (1, 2), (4, 8)],
        [(1, 4), (7, 13), (2, 8), (5, 6), (9, 10)],
        [(2, 4), (11, 13), (3, 8), (7, 12)],
        [(6, 8), (10, 12), (3, 5), (7, 9)],
        [(3, 4), (5, 6), (7, 8), (9, 10), (11, 12)],
        [(6, 7), (8, 9)]
    ]
);
gen_sortnet!(
    sortnet15,
    15,
    [
        [(0, 1), (2, 3), (4, 5), (6, 7), (8, 9), (10, 11), (12, 13)],
        [(0, 2), (4, 6), (8, 10), (12, 14), (1, 3), (5, 7), (9, 11)],
        [(0, 4), (8, 12), (1, 5), (9, 13), (2, 6), (10, 14), (3, 7)],
        [(0, 8), (1, 9), (2, 10), (3, 11), (4, 12), (5, 13), (6, 14)],
        [(5, 10), (6, 9), (3, 12), (13, 14), (7, 11), (1, 2), (4, 8)],
        [(1, 4), (7, 13), (2, 8), (11, 14), (5, 6), (9, 10)],
        [(2, 4), (11, 13), (3, 8), (7, 12)],
        [(6, 8), (10, 12), (3, 5), (7, 9)],
        [(3, 4), (5, 6), (7, 8), (9, 10), (11, 12)],
        [(6, 7), (8, 9)]
    ]
);
gen_sortnet!(
    sortnet16,
    16,
    [
        [
            (0, 1),
            (2, 3),
            (4, 5),
            (6, 7),
            (8, 9),
            (10, 11),
            (12, 13),
            (14, 15)
        ],
        [
            (0, 2),
            (4, 6),
            (8, 10),
            (12, 14),
            (1, 3),
            (5, 7),
            (9, 11),
            (13, 15)
        ],
        [
            (0, 4),
            (8, 12),
            (1, 5),
            (9, 13),
            (2, 6),
            (10, 14),
            (3, 7),
            (11, 15)
        ],
        [
            (0, 8),
            (1, 9),
            (2, 10),
            (3, 11),
            (4, 12),
            (5, 13),
            (6, 14),
            (7, 15)
        ],
        [(5, 10), (6, 9), (3, 12), (13, 14), (7, 11), (1, 2), (4, 8)],
        [(1, 4), (7, 13), (2, 8), (11, 14), (5, 6), (9, 10)],
        [(2, 4), (11, 13), (3, 8), (7, 12)],
        [(6, 8), (10, 12), (3, 5), (7, 9)],
        [(3, 4), (5, 6), (7, 8), (9, 10), (11, 12)],
        [(6, 7), (8, 9)]
    ]
);

pub fn sortnet<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    match arr.len() {
        0 => {
            sortnet0(array_mut_ref![arr, 0, 0]);
        }
        1 => {
            sortnet1(array_mut_ref![arr, 0, 1]);
        }
        2 => {
            sortnet2(array_mut_ref![arr, 0, 2]);
        }
        3 => {
            sortnet3(array_mut_ref![arr, 0, 3]);
        }
        4 => {
            sortnet4(array_mut_ref![arr, 0, 4]);
        }
        5 => {
            sortnet5(array_mut_ref![arr, 0, 5]);
        }
        6 => {
            sortnet6(array_mut_ref![arr, 0, 6]);
        }
        7 => {
            sortnet7(array_mut_ref![arr, 0, 7]);
        }
        8 => {
            sortnet8(array_mut_ref![arr, 0, 8]);
        }
        9 => {
            sortnet9(array_mut_ref![arr, 0, 9]);
        }
        10 => {
            sortnet10(array_mut_ref![arr, 0, 10]);
        }
        11 => {
            sortnet11(array_mut_ref![arr, 0, 11]);
        }
        12 => {
            sortnet12(array_mut_ref![arr, 0, 12]);
        }
        13 => {
            sortnet13(array_mut_ref![arr, 0, 13]);
        }
        14 => {
            sortnet14(array_mut_ref![arr, 0, 14]);
        }
        15 => {
            sortnet15(array_mut_ref![arr, 0, 15]);
        }
        16 => {
            sortnet16(array_mut_ref![arr, 0, 16]);
        }
        _ => {
            panic!("Unsupported size");
        }
    }
}

#[cfg(test)]
mod tests {
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
                use super::{sortnet, $fsort};
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
}
