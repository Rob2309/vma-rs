use std::ffi::c_void;

use ash::vk;
use vma_sys::{VmaPool, VmaAllocation};

use crate::{AllocationCreateFlags, MemoryUsage};


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AllocationCreateInfo {
    pub flags: AllocationCreateFlags,
    pub usage: MemoryUsage,
    pub required_flags: vk::MemoryPropertyFlags,
    pub preferred_flags: vk::MemoryPropertyFlags,
    pub memory_type_bits: u32,
    pub pool: VmaPool,
    pub user_data: *mut c_void,
    pub priority: f32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Allocation(pub(crate) VmaAllocation);
