use core::ptr::NonNull;

pub struct RingBuf<T, const N: usize> {
    head: usize,
    tail: usize,
    data: NonNull<T>
}

impl<T, const N: usize> RingBuf<T, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        assert!(N.is_power_of_two(), "N must be a power of two");

        Self {
            head: 0,
            tail: 0,
            data: unsafe { crate::alloc_array::<T, N>() }
        }
    }

    #[inline(always)]
    pub const fn push(&mut self, val: T) -> Result<(), T> {
        if self.head.wrapping_sub(self.tail) == N {
            Err(val)
        } else {
            Ok(unsafe { self.push_unchecked(val) })
        }
    }

    #[inline(always)]
    pub const unsafe fn push_unchecked(&mut self, val: T) {
        self.data.as_ptr()
            .offset(Self::to_physical_idx(self.head))
            .write(val);

        self.head = self.head.wrapping_add(1);
    }

    #[inline(always)]
    pub const fn pop(&mut self) -> Option<T> {
        if self.tail == self.head {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    #[inline(always)]
    pub const unsafe fn pop_unchecked(&mut self) -> T {
        let r = self.data.as_ptr()
            .offset(Self::to_physical_idx(self.tail))
            .read();
        self.tail = self.tail.wrapping_add(1);
        r
    }

    #[inline(always)]
    const fn to_physical_idx(idx: usize) -> isize {
        (idx & (N - 1)) as _
    }
}

#[cfg(test)]
mod tests {
    use crate::ring_buf::*;

    #[test]
    fn test() {
        static mut RING_BUF: RingBuf<u8, 2> = RingBuf::new();

        for _ in 0..4 {
            unsafe {
                assert!(matches!(RING_BUF.pop(), None));
                assert!(matches!(RING_BUF.push(1), Ok(())));
                assert!(matches!(RING_BUF.push(2), Ok(())));
                assert!(matches!(RING_BUF.push(3), Err(3)));
                assert!(matches!(RING_BUF.pop(), Some(1)));
                assert!(matches!(RING_BUF.pop(), Some(2)));
                assert!(matches!(RING_BUF.pop(), None));
            }
        }
    }
}