use std::{
    mem::{align_of, size_of, MaybeUninit},
    ptr::addr_of,
};

use vma::{
    AllocationCreateInfo, AllocationInfo, AllocatorInfo, Budget, DefragmentationInfo,
    DefragmentationPassMove, DetailedStatistics, PoolCreateInfo, Statistics, TotalStatistics,
};
use vma_sys::{
    VmaAllocationCreateInfo, VmaAllocationInfo, VmaAllocatorInfo, VmaBudget,
    VmaDefragmentationInfo, VmaDefragmentationMove, VmaDetailedStatistics, VmaPoolCreateInfo,
    VmaStatistics, VmaTotalStatistics,
};

macro_rules! check_layout {
    ($struct_a:ident == $struct_b:ident {
        $($field_a:ident == $field_b:ident),* $(,)?
    }) => {
        let uninit_a = MaybeUninit::<$struct_a>::uninit();
        let uninit_b = MaybeUninit::<$struct_b>::uninit();

        let ptr_a = uninit_a.as_ptr();
        let ptr_b = uninit_b.as_ptr();

        assert_eq!(size_of::<$struct_a>(), size_of::<$struct_b>());
        assert_eq!(align_of::<$struct_a>(), align_of::<$struct_b>());

        unsafe {
            $(
                assert_eq!(
                    addr_of!((*ptr_a).$field_a) as usize - ptr_a as usize,
                    addr_of!((*ptr_b).$field_b) as usize - ptr_b as usize,
                );
            )*
        }
    };
}

#[test]
fn allocation_create_info_layout() {
    check_layout!(
        AllocationCreateInfo == VmaAllocationCreateInfo {
            flags == flags,
            usage == usage,
            required_flags == requiredFlags,
            preferred_flags == preferredFlags,
            memory_type_bits == memoryTypeBits,
            pool == pool,
            user_data == pUserData,
            priority == priority,
        }
    );
}

#[test]
fn allocator_info_layout() {
    check_layout!(
        AllocatorInfo == VmaAllocatorInfo {
            instance == instance,
            physical_device == physicalDevice,
            device == device,
        }
    );
}

#[test]
fn total_statistics_layout() {
    check_layout!(
        TotalStatistics == VmaTotalStatistics {
            memory_type == memoryType,
            memory_heap == memoryHeap,
            total == total,
        }
    );
}

#[test]
fn detailed_statistics_layout() {
    check_layout!(
        DetailedStatistics == VmaDetailedStatistics {
            statistics == statistics,
            unused_range_count == unusedRangeCount,
            allocation_size_min == allocationSizeMin,
            allocation_size_max == allocationSizeMax,
            unused_range_size_min == unusedRangeSizeMin,
            unused_range_size_max == unusedRangeSizeMax,
        }
    );
}

#[test]
fn statistics_layout() {
    check_layout!(
        Statistics == VmaStatistics {
            block_count == blockCount,
            allocation_count == allocationCount,
            block_bytes == blockBytes,
            allocation_bytes == allocationBytes,
        }
    );
}

#[test]
fn budget_layout() {
    check_layout!(
        Budget == VmaBudget {
            statistics == statistics,
            usage == usage,
            budget == budget,
        }
    );
}

#[test]
fn pool_create_info_layout() {
    check_layout!(
        PoolCreateInfo == VmaPoolCreateInfo {
            memory_type_index == memoryTypeIndex,
            flags == flags,
            block_size == blockSize,
            min_block_count == minBlockCount,
            max_block_count == maxBlockCount,
            priority == priority,
            min_allocation_alignment == minAllocationAlignment,
            memory_allocate_next == pMemoryAllocateNext,
        }
    );
}

#[test]
fn allocation_info_layout() {
    check_layout!(
        AllocationInfo == VmaAllocationInfo {
            memory_type == memoryType,
            device_memory == deviceMemory,
            offset == offset,
            size == size,
            mapped_data == pMappedData,
            user_data == pUserData,
            name == pName,
        }
    );
}

#[test]
fn defragmentation_info_layout() {
    check_layout!(
        DefragmentationInfo == VmaDefragmentationInfo {
            flags == flags,
            pool == pool,
            max_bytes_per_pass == maxBytesPerPass,
            max_allocations_per_pass == maxAllocationsPerPass,
        }
    );
}

#[test]
fn defragmentation_pass_move_layout() {
    check_layout!(
        DefragmentationPassMove == VmaDefragmentationMove {
            operation == operation,
            src_allocation == srcAllocation,
            dst_tmp_allocation == dstTmpAllocation,
        }
    );
}
