#![no_std]

#![feature(core_intrinsics)]
#![feature(const_heap)]
#![feature(const_mut_refs)]
#![feature(const_ptr_read)]
#![feature(const_ptr_write)]
#![feature(const_eval_select)]
#![feature(const_alloc_layout)]

pub mod alloc;
pub mod vec;
pub mod ring_buf;