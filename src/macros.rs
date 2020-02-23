/// Create new sortnet implementation.
///
/// Takes the following arguments:
///
/// - `fsort`: name of the public function that gets created
/// - `n`: number of elements in the array
/// - `swaps`: elements to be compared in order. Must be grouped by operations that can be executed
///    in parallel.
///
/// # Example
/// We want to create a new sortnet for 4 elements that looks like this:
///
/// ```text
/// o o o o
/// | | | |
/// +-+ +-+
/// | | | |
/// +---+ |
/// | | | |
/// | +---+
/// | | | |
/// | +-+ |
/// | | | |
/// V V V V
/// ```
///
/// There are 5 compare-and-swap operations:
///
/// - 0 and 1
/// - 2 and 3
/// - 0 and 2
/// - 1 and 3
/// - 1 and 2
///
/// These can be grouped in the following way so that they that elements within a group can be executed in parallel:
///
/// - (0 and 1), (2 and 3)
/// - (0 and 2), (1 and 3)
/// - (1 and 2)
///
/// In code, this looks like this:
/// ```
/// use sortnet::gen_sortnet;
///
/// gen_sortnet!(my_sortnet, 4, [[(0, 1), (2, 3)], [(0, 2), (1, 3)], [(1, 2)]]);
///
/// let mut data = [0, 13, -1, 2];
/// my_sortnet(&mut data);
/// assert_eq!(data, [-1, 0, 2, 13]);
/// ```
///
/// # Requirements
/// Ther are a few compile-time requirements to use the `gen_sortnet` macrol
///
/// ## Non-empty Groups
/// Groups must not be empty:
///
/// ```compile_fail
/// use sortnet::gen_sortnet;
///
/// gen_sortnet!(my_sortnet, 3, [[], [(1, 2)], [(0, 2)], [(0, 1)]]);
/// ```
///
/// ## Non-negative Size
/// The array size must not be negative:
///
/// ```compile_fail
/// use sortnet::gen_sortnet;
///
/// gen_sortnet!(my_sortnet, -1, []);
/// ```
///
/// ## Non-negative Indices
/// Sortnet indices must not be negative:
///
/// ```compile_fail
/// use sortnet::gen_sortnet;
///
/// gen_sortnet!(my_sortnet, 3, [[(1, 2)], [(0, 2)], [(0, -1)]]);
/// ```
///
/// ## In-bounds Indices
/// Sortnet indices must not be out-of bounds:
///
/// ```compile_fail
/// use sortnet::gen_sortnet;
///
/// gen_sortnet!(my_sortnet, 3, [[(1, 2)], [(0, 2)], [(0, 3)]]);
/// ```
///
/// ```compile_fail
/// use sortnet::gen_sortnet;
///
/// gen_sortnet!(my_sortnet, 3, [[(1, 2)], [(0, 2)], [(3, 1)]]);
/// ```
#[macro_export]
macro_rules! gen_sortnet {
    ($fsort:ident, $n:expr, [$([$(($swap_a:expr, $swap_b:expr)),+]),*]) => {
        $crate::__gen_sortnet_inner!($fsort, $n, stringify!($n), [$([$(($swap_a, $swap_b)),+]),*]);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __gen_sortnet_inner {
    ($fsort:ident, $n:expr, $n_str:expr, [$([$(($swap_a:expr, $swap_b:expr)),+]),*]) => {
        #[doc="Sortnet-based sorting for arrays with "]
        #[doc=$n_str]
        #[doc=" elements."]
        pub fn $fsort<T>(arr: &mut [T; $n])
        where
            T: PartialOrd,
        {
            #[allow(unused_imports)]
            use static_assertions::const_assert;

            #[allow(dead_code)]
            #[inline(always)]
            pub fn maybe_swap<T>(arr: &mut [T; $n], a: usize, b: usize)
            where
                T: PartialOrd,
            {
                if arr[a] > arr[b] {
                    arr.swap(a, b)
                }
            }

            #[allow(unused_mut, unused_variables)] let mut arr = arr;

            $($(
            const_assert!($swap_a < $n);
            const_assert!($swap_b < $n);
            maybe_swap(&mut arr, $swap_a, $swap_b);
            )+)*
        }
    };
}
