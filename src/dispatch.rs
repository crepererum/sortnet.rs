use crate::networks::*;
use arrayref::array_mut_ref;

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
