#![feature(test)]
#![feature(vec_push_within_capacity)]

extern crate test;

use std::collections::VecDeque;
use test::Bencher;

type T = u8;

const L: usize = 4096;

#[bench]
fn vec_deque(b: &mut Bencher) {
    let mut buf = VecDeque::<T>::with_capacity(L);

    b.iter(|| {
        for i in 0..L {
            buf.push_back(i as _);
        }

        for _ in 0..L {
            buf.pop_front().unwrap();
        }
    });
}

#[bench]
fn ring_buf(b: &mut Bencher) {
    static mut BUF: const_collections::ring_buf::RingBuf<T, L> = const_collections::ring_buf::RingBuf::new();

    b.iter(|| unsafe {
        for i in 0..L {
            BUF.push(i as _).unwrap();
        }

        for _ in 0..L {
            BUF.pop().unwrap();
        }
    });
}