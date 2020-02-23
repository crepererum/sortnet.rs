#[macro_export]
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
