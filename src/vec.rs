use core::ptr::NonNull;

pub struct Vec<T, const N: usize> {
    len: usize,
    mem: NonNull<T>
}

impl<T, const N: usize> Vec<T, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            len: 0,
            mem: unsafe { crate::alloc_array::<T, N>() }
        }
    }

    #[inline(always)]
    pub const unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    #[inline(always)]
    pub const fn push(&mut self, val: T) -> Result<(), T> {
        if self.len == N {
            Err(val)
        } else {
            Ok(unsafe { self.push_unchecked(val) })
        }
    }

    #[inline(always)]
    pub const unsafe fn push_unchecked(&mut self, val: T) {
        self.mem.as_ptr()
            .offset(self.len as _)
            .write(val);

        self.len += 1;
    }

    #[inline(always)]
    pub const fn pop(&mut self) -> Option<T> {
        if self.len != 0 {
            Some(unsafe { self.pop_unchecked() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub const unsafe fn pop_unchecked(&mut self) -> T {
        let r = self.mem.as_ptr()
            .offset(self.len as isize - 1)
            .read();
        self.len -= 1;
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::*;

    #[test]
    fn test() {
        static mut VEC: Vec<u8, 2> = Vec::new();

        for _ in 0..4 {
            unsafe {
                assert!(matches!(VEC.pop(), None));
                assert!(matches!(VEC.push(1), Ok(())));
                assert!(matches!(VEC.push(2), Ok(())));
                assert!(matches!(VEC.push(3), Err(3)));
                assert!(matches!(VEC.pop(), Some(2)));
                assert!(matches!(VEC.pop(), Some(1)));
                assert!(matches!(VEC.pop(), None));
            }
        }
    }
}