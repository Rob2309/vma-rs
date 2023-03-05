use std::ffi::{c_void, c_char};

use ash::vk;
use vma_sys::{VmaPool, VmaAllocation, VmaDefragmentationContext};

use crate::{AllocationCreateFlags, MemoryUsage, PoolCreateFlags, DefragmentationFlags, DefragmentationMoveOperation};


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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllocatorInfo {
    pub instance: vk::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TotalStatistics {
    pub memory_type: [DetailedStatistics; 32],
    pub memory_heap: [DetailedStatistics; 16],
    pub total: DetailedStatistics,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetailedStatistics {
    pub statistics: Statistics,
    pub unused_range_count: u32,
    pub allocation_size_min: vk::DeviceSize,
    pub allocation_size_max: vk::DeviceSize,
    pub unused_range_size_min: vk::DeviceSize,
    pub unused_range_size_max: vk::DeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statistics {
    pub block_count: u32,
    pub allocation_count: u32,
    pub block_bytes: vk::DeviceSize,
    pub allocation_bytes: vk::DeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Budget {
    pub statistics: Statistics,
    pub usage: vk::DeviceSize,
    pub budget: vk::DeviceSize,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pool(pub(crate) VmaPool);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PoolCreateInfo {
    pub memory_type_index: u32,
    pub flags: PoolCreateFlags,
    pub block_size: vk::DeviceSize,
    pub min_block_count: usize,
    pub max_block_count: usize,
    pub priority: f32,
    pub min_allocation_alignment: vk::DeviceSize,
    pub memory_allocate_next: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllocationInfo {
    pub memory_type: u32,
    pub device_memory: vk::DeviceMemory,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub mapped_data: *mut c_void,
    pub user_data: *mut c_void,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefragmentationInfo {
    pub flags: DefragmentationFlags,
    pub pool: Pool,
    pub max_bytes_per_pass: vk::DeviceSize,
    pub max_allocations_per_pass: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefragmentationContext(pub(crate) VmaDefragmentationContext);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefragmentationPassMove {
    pub operation: DefragmentationMoveOperation,
    pub src_allocation: Allocation,
    pub dst_tmp_allocation: Allocation,
}
