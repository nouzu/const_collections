use core::alloc::Layout;
use core::ptr::NonNull;

#[inline(always)]
pub const unsafe fn alloc_array<T, const N: usize>() -> NonNull<T> {
    match Layout::array::<T>(N) {
        Ok(layout) => {
            NonNull::new_unchecked(alloc(layout) as *mut T)
        }
        Err(_) => {
            panic!("layout error")
        }
    }
}

#[inline(always)]
pub const unsafe fn alloc(layout: Layout) -> *mut u8 {
    core::intrinsics::const_eval_select(((), layout), __ct_alloc, __rt_alloc)
}

#[inline(always)]
const fn __ct_alloc(_: (), layout: Layout) -> *mut u8 {
    unsafe { core::intrinsics::const_allocate(layout.size(), layout.align()) }
}

#[inline(always)]
fn __rt_alloc(_: (), _: Layout) -> *mut u8 {
    panic!("runtime allocation is not supported")
}
