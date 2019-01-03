use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn main() {
	let mut v = vec![1, 2, 3, 4, 5, 6];

	let r = &mut v[..];

	let (a, b) = r.split_at_mut(3);

	assert_eq!(a, &mut [1, 2, 3]);
	assert_eq!(b, &mut [4, 5, 6]);
}
