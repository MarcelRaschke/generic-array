#![no_std]
use core::cell::Cell;
use core::ops::{Add, Drop};
use generic_array::arr;
use generic_array::functional::*;
use generic_array::sequence::*;
use generic_array::typenum::{U0, U3, U4, U97};
use generic_array::GenericArray;

#[cfg(feature = "alloc")]
extern crate alloc;

#[test]
fn test() {
    let mut list97 = [0; 97];
    for (i, elem) in list97.iter_mut().enumerate() {
        *elem = i as i32;
    }
    let l: GenericArray<i32, U97> = *GenericArray::from_slice(&list97);
    assert_eq!(l[0], 0);
    assert_eq!(l[1], 1);
    assert_eq!(l[32], 32);
    assert_eq!(l[56], 56);
}

#[test]
fn test_drop() {
    #[derive(Clone)]
    struct TestDrop<'a>(&'a Cell<u32>);

    impl<'a> Drop for TestDrop<'a> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let drop_counter = Cell::new(0);
    {
        let _: GenericArray<TestDrop, U3> = arr![
            TestDrop(&drop_counter),
            TestDrop(&drop_counter),
            TestDrop(&drop_counter)
        ];
    }
    assert_eq!(drop_counter.get(), 3);
}

#[test]
fn test_arr() {
    let test: GenericArray<u32, U3> = arr![1, 2, 3];
    assert_eq!(test[1], 2);
}

#[test]
fn test_copy() {
    let test = arr![1, 2, 3];
    let test2 = test;
    // if GenericArray is not copy, this should fail as a use of a moved value
    assert_eq!(test[1], 2);
    assert_eq!(test2[0], 1);
}

#[derive(Debug, PartialEq, Eq)]
struct NoClone<T>(T);

#[test]
fn test_from_slice() {
    let arr = [1, 2, 3, 4];
    let gen_arr = GenericArray::<_, U3>::from_slice(&arr[..3]);
    assert_eq!(&arr[..3], gen_arr.as_slice());
    let arr = [NoClone(1u32), NoClone(2), NoClone(3), NoClone(4)];
    let gen_arr = GenericArray::<_, U3>::from_slice(&arr[..3]);
    assert_eq!(&arr[..3], gen_arr.as_slice());
}

#[test]
fn test_from_mut_slice() {
    let mut arr = [1, 2, 3, 4];
    {
        let gen_arr = GenericArray::<_, U3>::from_mut_slice(&mut arr[..3]);
        gen_arr[2] = 10;
    }
    assert_eq!(arr, [1, 2, 10, 4]);
    let mut arr = [NoClone(1u32), NoClone(2), NoClone(3), NoClone(4)];
    {
        let gen_arr = GenericArray::<_, U3>::from_mut_slice(&mut arr[..3]);
        gen_arr[2] = NoClone(10);
    }
    assert_eq!(arr, [NoClone(1), NoClone(2), NoClone(10), NoClone(4)]);
}

#[test]
fn test_default() {
    let arr = GenericArray::<u8, U4>::default();
    assert_eq!(arr.as_slice(), &[0, 0, 0, 0]);
}

#[test]
fn test_from() {
    let data = [(1, 2, 3), (4, 5, 6), (7, 8, 9)];
    let garray: GenericArray<(usize, usize, usize), U3> = data.into();
    assert_eq!(&data, garray.as_slice());
}

#[test]
fn test_unit_macro() {
    let arr = arr![5.81];
    assert_eq!(arr[0], 5.81);
}

#[test]
fn test_empty_macro() {
    let _arr: GenericArray<(), _> = arr![];
}

#[test]
fn test_cmp() {
    let _ = arr![0x00u8].cmp(&arr![0x00]);
}

/// This test should cause a helpful compile error if uncommented.
// #[test]
// fn test_empty_macro2(){
//     let arr = arr![];
// }
#[cfg(feature = "serde")]
mod impl_serde {
    extern crate serde_json;

    use generic_array::arr;
    use generic_array::typenum::U6;
    use generic_array::GenericArray;

    #[test]
    fn test_serde_implementation() {
        let array: GenericArray<f64, U6> = arr![0.0, 5.0, 3.0, 7.07192, 76.0, -9.0];
        let string = serde_json::to_string(&array).unwrap();
        assert_eq!(string, "[0.0,5.0,3.0,7.07192,76.0,-9.0]");

        let test_array: GenericArray<f64, U6> = serde_json::from_str(&string).unwrap();
        assert_eq!(test_array, array);
    }
}

#[test]
fn test_map() {
    let b: GenericArray<i32, U4> = GenericArray::generate(|i| i as i32 * 4).map(|x| x - 3);

    assert_eq!(b, arr![-3, 1, 5, 9]);
}

#[test]
fn test_zip() {
    let a: GenericArray<_, U4> = GenericArray::generate(|i| i + 1);
    let b: GenericArray<_, U4> = GenericArray::generate(|i| i as i32 * 4);

    // Uses reference and non-reference arguments
    let c = (&a).zip(b, |r, l| *r as i32 + l);

    assert_eq!(c, arr![1, 6, 11, 16]);
}

#[test]
#[should_panic]
fn test_from_iter_short() {
    use core::iter::repeat;

    let a: GenericArray<_, U4> = repeat(11).take(3).collect();

    assert_eq!(a, arr![11, 11, 11, 0]);
}

#[test]
fn test_from_iter() {
    use core::iter::{once, repeat};

    let a: GenericArray<_, U4> = repeat(11).take(3).chain(once(0)).collect();

    assert_eq!(a, arr![11, 11, 11, 0]);
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
enum E {
    V,
    V2(i32),
    V3 { h: bool, i: i32 },
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
struct Test {
    t: u16,
    s: u32,
    mm: bool,
    r: u16,
    f: u16,
    p: (),
    o: u32,
    ff: *const extern "C" fn(*const char) -> *const core::ffi::c_void,
    l: *const core::ffi::c_void,
    w: bool,
    q: bool,
    v: E,
}

#[test]
fn test_sizes() {
    use core::mem::{size_of, size_of_val};

    assert_eq!(size_of::<E>(), 8);

    assert_eq!(size_of::<Test>(), 25 + size_of::<usize>() * 2);

    assert_eq!(size_of_val(&arr![1u8, 2, 3]), size_of::<u8>() * 3);
    assert_eq!(size_of_val(&arr![1u32]), size_of::<u32>() * 1);
    assert_eq!(size_of_val(&arr![1u64, 2, 3, 4]), size_of::<u64>() * 4);

    assert_eq!(size_of::<GenericArray<Test, U97>>(), size_of::<Test>() * 97);
}

#[test]
fn test_alignment() {
    use core::mem::align_of;

    assert_eq!(
        align_of::<GenericArray::<u32, U0>>(),
        align_of::<[u32; 0]>()
    );
    assert_eq!(
        align_of::<GenericArray::<u32, U3>>(),
        align_of::<[u32; 3]>()
    );
    assert_eq!(
        align_of::<GenericArray::<Test, U3>>(),
        align_of::<[Test; 3]>()
    );
}

#[test]
fn test_append() {
    let a = arr![1, 2, 3];

    let b = a.append(4);

    assert_eq!(b, arr![1, 2, 3, 4]);
}

#[test]
fn test_prepend() {
    let a = arr![1, 2, 3];

    let b = a.prepend(4);

    assert_eq!(b, arr![4, 1, 2, 3]);
}

#[test]
fn test_pop() {
    let a = arr![1, 2, 3, 4];

    let (init, last) = a.pop_back();

    assert_eq!(init, arr![1, 2, 3]);
    assert_eq!(last, 4);

    let (head, tail) = a.pop_front();

    assert_eq!(head, 1);
    assert_eq!(tail, arr![2, 3, 4]);
}

#[test]
fn test_split() {
    let a = arr![1, 2, 3, 4];

    let (b, c) = a.split();

    assert_eq!(b, arr![1]);
    assert_eq!(c, arr![2, 3, 4]);

    let (e, f) = a.split();

    assert_eq!(e, arr![1, 2]);
    assert_eq!(f, arr![3, 4]);
}

#[test]
fn test_split_ref() {
    let a = arr![1, 2, 3, 4];
    let a_ref = &a;

    let (b_ref, c_ref) = a_ref.split();

    assert_eq!(b_ref, &arr![1]);
    assert_eq!(c_ref, &arr![2, 3, 4]);

    let (e_ref, f_ref) = a_ref.split();

    assert_eq!(e_ref, &arr![1, 2]);
    assert_eq!(f_ref, &arr![3, 4]);
}

#[test]
fn test_split_mut() {
    let mut a = arr![1, 2, 3, 4];
    let a_ref = &mut a;

    let (b_ref, c_ref) = a_ref.split();

    assert_eq!(b_ref, &mut arr![1]);
    assert_eq!(c_ref, &mut arr![2, 3, 4]);

    let (e_ref, f_ref) = a_ref.split();

    assert_eq!(e_ref, &mut arr![1, 2]);
    assert_eq!(f_ref, &mut arr![3, 4]);
}

#[test]
fn test_concat() {
    let a = arr![1, 2];
    let b = arr![3, 4, 5];

    let c = a.concat(b);

    assert_eq!(c, arr![1, 2, 3, 4, 5]);

    let (d, e) = c.split();

    assert_eq!(d, arr![1, 2]);
    assert_eq!(e, arr![3, 4, 5]);
}

#[test]
fn test_removes() {
    let a = arr![1, 2, 3, 4];

    for i in 0..4 {
        let (b, c) = a.remove(i);

        assert_eq!(b, i + 1);
        assert_eq!(
            c,
            match i {
                0 => arr![2, 3, 4],
                1 => arr![1, 3, 4],
                2 => arr![1, 2, 4],
                3 => arr![1, 2, 3],
                _ => unreachable!(),
            }
        );

        let (b, c) = a.swap_remove(i);

        assert_eq!(b, i + 1);
        assert_eq!(
            c,
            match i {
                0 => arr![4, 2, 3],
                1 => arr![1, 4, 3],
                2 => arr![1, 2, 4],
                3 => arr![1, 2, 3],
                _ => unreachable!(),
            }
        );
    }
}

#[test]
fn test_fold() {
    let a = arr![1, 2, 3, 4];

    assert_eq!(10, a.fold(0, |a, x| a + x));
}

fn sum_generic<S>(s: S) -> i32
where
    S: FunctionalSequence<i32>,
    S::Item: Add<i32, Output = i32>, // `+`
    i32: Add<S::Item, Output = i32>, // reflexive
{
    s.fold(0, |a, x| a + x)
}

#[test]
fn test_sum() {
    let a = sum_generic(arr![1, 2, 3, 4]);

    assert_eq!(a, 10);
}

#[test]
fn test_as_ref() {
    let a = arr![1, 2, 3, 4];
    let a_ref: &[i32; 4] = a.as_ref();
    assert_eq!(a_ref, &[1, 2, 3, 4]);
}

#[test]
fn test_as_mut() {
    let mut a = arr![1, 2, 3, 4];
    let a_mut: &mut [i32; 4] = a.as_mut();
    assert_eq!(a_mut, &mut [1, 2, 3, 4]);
    a_mut[2] = 0;
    assert_eq!(a_mut, &mut [1, 2, 0, 4]);
    assert_eq!(a, arr![1, 2, 0, 4]);
}

#[test]
fn test_from_array_ref() {
    let a = arr![1, 2, 3, 4];
    let a_ref: &[i32; 4] = a.as_ref();
    let a_from: &GenericArray<i32, U4> = a_ref.into();
    assert_eq!(&a, a_from);
}

#[test]
fn test_from_array_mut() {
    let mut a = arr![1, 2, 3, 4];
    let mut a_copy = a;
    let a_mut: &mut [i32; 4] = a.as_mut();
    let a_from: &mut GenericArray<i32, U4> = a_mut.into();
    assert_eq!(&mut a_copy, a_from);
}

#[cfg(feature = "alloc")]
#[test]
fn test_try_from_vec() {
    let a = alloc::vec![1, 2, 3, 4];
    let _ = GenericArray::<_, U4>::try_from(a).unwrap();
}

#[cfg(feature = "alloc")]
#[test]
fn test_alloc() {
    use alloc::{boxed::Box, vec::Vec};
    use generic_array::box_arr;

    let x: Box<[i32]> = arr![1, 2, 3, 4, 5].into();
    assert_eq!(x.len(), 5);
    let y: GenericArray<i32, typenum::U5> = x.clone().try_into().unwrap();
    assert_eq!(&x[..], &y[..]);

    let x: Vec<i32> = arr![1, 2, 3, 4, 5].into();
    assert_eq!(x.len(), 5);
    let y: GenericArray<i32, typenum::U5> = x.clone().try_into().unwrap();
    assert_eq!(&x[..], &y[..]);

    let x: Vec<i32> = Box::new(arr![1, 2, 3, 4, 5]).into_vec();
    assert_eq!(x.len(), 5);
    let y: Box<GenericArray<i32, typenum::U5>> = GenericArray::try_from_vec(x.clone()).unwrap();
    assert_eq!(&x[..], &y[..]);

    let z =
        Box::<GenericArray<_, typenum::U5>>::from_iter(y.into_iter() as alloc::vec::IntoIter<_>);

    let _: Box<_> = z.clone().zip(Box::new(arr![1, 2, 3, 4, 5]), |a, b| a + b);

    let _ = z.map(|x| x + 1);

    let _ = arr![1, 2, 3, 4].zip(*box_arr![1, 2, 3, 4], |a, b| a + b);

    let _ = box_arr!(1, 2, 3, 4, 5);

    #[cfg(not(miri))]
    {
        // 128-bit * 10^6 = 16MB, large enough to overflow the stack, but not this
        let _ = box_arr![1u128; typenum::Exp<typenum::U10, typenum::U6>];

        let _ = GenericArray::<i128, typenum::Exp<typenum::U10, typenum::U6>>::default_boxed();
    }
}

#[test]
fn test_chunks() {
    // intended usage
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice(&[1, 2, 3, 4, 5, 6, 7]);

    assert_eq!(chunks[0], arr![1, 2, 3]);
    assert_eq!(chunks[1], arr![4, 5, 6]);
    assert_eq!(rem, &[7]);

    // zero-length input
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice(&[]);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());

    // zero-length output with zero-length input
    let (chunks, rem) = GenericArray::<u8, U0>::chunks_from_slice(&[]);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());

    // only remainder
    let (chunks, rem) = GenericArray::<u8, U3>::chunks_from_slice(&[1, 2]);

    assert!(chunks.is_empty());
    assert_eq!(rem, &[1, 2]);
}

#[test]
#[should_panic]
fn test_chunks_fail() {
    // zero-length output with input
    let (chunks, rem) = GenericArray::<u8, U0>::chunks_from_slice(&[1, 2, 3]);
    assert!(chunks.is_empty());
    assert!(rem.is_empty());
}

#[test]
fn test_try_map() {
    let a = arr![1, 2, 3, 4];

    let b = a.try_map(|x| {
        if x % 2 == 0 {
            Ok(x * 2)
        } else {
            Err("odd number")
        }
    });

    assert!(b.is_err());
}
