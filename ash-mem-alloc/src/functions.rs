#![allow(warnings)]
use ash::vk;
#[doc = "Creates #VmaAllocator object."]
pub unsafe fn create_allocator(
    p_create_info: &crate::vma::AllocatorCreateInfo,
) -> Result<crate::vma::Allocator, vk::Result> {
    extern "C" {
        fn vmaCreateAllocator(
            p_create_info: *const crate::vma::AllocatorCreateInfo,
            p_allocator: *mut crate::vma::Allocator,
        ) -> vk::Result;
    }
    let mut p_allocator = ::std::mem::zeroed();
    let result = vmaCreateAllocator(p_create_info, &mut p_allocator);
    if result == vk::Result::SUCCESS {
        Ok(p_allocator)
    } else {
        Err(result)
    }
}
#[doc = "Destroys allocator object."]
pub unsafe fn destroy_allocator(allocator: crate::vma::Allocator) {
    extern "C" {
        fn vmaDestroyAllocator(allocator: crate::vma::Allocator);
    }
    vmaDestroyAllocator(allocator);
}
#[doc = "\\brief Returns information about existing #VmaAllocator object - handle to Vulkan device etc.\n\nIt might be useful if you want to keep just the #VmaAllocator handle and fetch other required handles to\n`VkPhysicalDevice`, `VkDevice` etc. every time using this function."]
pub unsafe fn get_allocator_info(allocator: crate::vma::Allocator) -> crate::vma::AllocatorInfo {
    extern "C" {
        fn vmaGetAllocatorInfo(
            allocator: crate::vma::Allocator,
            p_allocator_info: *mut crate::vma::AllocatorInfo,
        );
    }
    let mut p_allocator_info = ::std::mem::zeroed();
    vmaGetAllocatorInfo(allocator, &mut p_allocator_info);
    p_allocator_info
}
#[doc = "PhysicalDeviceProperties are fetched from physicalDevice by the allocator.\nYou can access it here, without fetching it again on your own."]
pub unsafe fn get_physical_device_properties(
    allocator: crate::vma::Allocator,
) -> *const vk::PhysicalDeviceProperties {
    extern "C" {
        fn vmaGetPhysicalDeviceProperties(
            allocator: crate::vma::Allocator,
            pp_physical_device_properties: *mut *const vk::PhysicalDeviceProperties,
        );
    }
    let mut pp_physical_device_properties = ::std::mem::zeroed();
    vmaGetPhysicalDeviceProperties(allocator, &mut pp_physical_device_properties);
    pp_physical_device_properties
}
#[doc = "PhysicalDeviceMemoryProperties are fetched from physicalDevice by the allocator.\nYou can access it here, without fetching it again on your own."]
pub unsafe fn get_memory_properties(
    allocator: crate::vma::Allocator,
) -> *const vk::PhysicalDeviceMemoryProperties {
    extern "C" {
        fn vmaGetMemoryProperties(
            allocator: crate::vma::Allocator,
            pp_physical_device_memory_properties: *mut *const vk::PhysicalDeviceMemoryProperties,
        );
    }
    let mut pp_physical_device_memory_properties = ::std::mem::zeroed();
    vmaGetMemoryProperties(allocator, &mut pp_physical_device_memory_properties);
    pp_physical_device_memory_properties
}
#[doc = "\\brief Given Memory Type Index, returns Property Flags of this memory type.\n\nThis is just a convenience function. Same information can be obtained using\nvmaGetMemoryProperties()."]
pub unsafe fn get_memory_type_properties(
    allocator: crate::vma::Allocator,
    memory_type_index: u32,
) -> vk::MemoryPropertyFlags {
    extern "C" {
        fn vmaGetMemoryTypeProperties(
            allocator: crate::vma::Allocator,
            memory_type_index: u32,
            p_flags: *mut vk::MemoryPropertyFlags,
        );
    }
    let mut p_flags = ::std::mem::zeroed();
    vmaGetMemoryTypeProperties(allocator, memory_type_index, &mut p_flags);
    p_flags
}
#[doc = "\\brief Sets index of the current frame."]
pub unsafe fn set_current_frame_index(allocator: crate::vma::Allocator, frame_index: u32) {
    extern "C" {
        fn vmaSetCurrentFrameIndex(allocator: crate::vma::Allocator, frame_index: u32);
    }
    vmaSetCurrentFrameIndex(allocator, frame_index);
}
#[doc = "\\brief Retrieves statistics from current state of the Allocator.\n\nThis function is called \"calculate\" not \"get\" because it has to traverse all\ninternal data structures, so it may be quite slow. Use it for debugging purposes.\nFor faster but more brief statistics suitable to be called every frame or every allocation,\nuse vmaGetHeapBudgets().\n\nNote that when using allocator from multiple threads, returned information may immediately\nbecome outdated."]
pub unsafe fn calculate_statistics(
    allocator: crate::vma::Allocator,
) -> crate::vma::TotalStatistics {
    extern "C" {
        fn vmaCalculateStatistics(
            allocator: crate::vma::Allocator,
            p_stats: *mut crate::vma::TotalStatistics,
        );
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaCalculateStatistics(allocator, &mut p_stats);
    p_stats
}
#[doc = "\\brief Retrieves information about current memory usage and budget for all memory heaps.\n\n\\param allocator\n\\param[out] pBudgets Must point to array with number of elements at least equal to number of memory heaps in physical device used.\n\nThis function is called \"get\" not \"calculate\" because it is very fast, suitable to be called\nevery frame or every allocation. For more detailed statistics use vmaCalculateStatistics().\n\nNote that when using allocator from multiple threads, returned information may immediately\nbecome outdated."]
pub unsafe fn get_heap_budgets(allocator: crate::vma::Allocator) -> Vec<crate::vma::Budget> {
    extern "C" {
        fn vmaGetHeapBudgets(allocator: crate::vma::Allocator, p_budgets: *mut crate::vma::Budget);
    }
    let p_budgets_len = (*get_memory_properties(allocator)).memory_heap_count;
    let mut p_budgets = vec![::std::mem::zeroed(); p_budgets_len as _];
    vmaGetHeapBudgets(allocator, p_budgets.as_mut_ptr());
    p_budgets
}
#[doc = "\\brief Helps to find memoryTypeIndex, given memoryTypeBits and VmaAllocationCreateInfo.\n\nThis algorithm tries to find a memory type that:\n\n- Is allowed by memoryTypeBits.\n- Contains all the flags from pAllocationCreateInfo->requiredFlags.\n- Matches intended usage.\n- Has as many flags from pAllocationCreateInfo->preferredFlags as possible.\n\n\\return Returns VK_ERROR_FEATURE_NOT_PRESENT if not found. Receiving such result\nfrom this function or any other allocating function probably means that your\ndevice doesn't support any memory type with requested features for the specific\ntype of resource you want to use it for. Please check parameters of your\nresource, like image layout (OPTIMAL versus LINEAR) or mip level count."]
pub unsafe fn find_memory_type_index(
    allocator: crate::vma::Allocator,
    memory_type_bits: u32,
    p_allocation_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<u32, vk::Result> {
    extern "C" {
        fn vmaFindMemoryTypeIndex(
            allocator: crate::vma::Allocator,
            memory_type_bits: u32,
            p_allocation_create_info: *const crate::vma::AllocationCreateInfo,
            p_memory_type_index: *mut u32,
        ) -> vk::Result;
    }
    let mut p_memory_type_index = ::std::mem::zeroed();
    let result = vmaFindMemoryTypeIndex(
        allocator,
        memory_type_bits,
        p_allocation_create_info,
        &mut p_memory_type_index,
    );
    if result == vk::Result::SUCCESS {
        Ok(p_memory_type_index)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Helps to find memoryTypeIndex, given VkBufferCreateInfo and VmaAllocationCreateInfo.\n\nIt can be useful e.g. to determine value to be used as VmaPoolCreateInfo::memoryTypeIndex.\nIt internally creates a temporary, dummy buffer that never has memory bound."]
pub unsafe fn find_memory_type_index_for_buffer_info(
    allocator: crate::vma::Allocator,
    p_buffer_create_info: &vk::BufferCreateInfo,
    p_allocation_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<u32, vk::Result> {
    extern "C" {
        fn vmaFindMemoryTypeIndexForBufferInfo(
            allocator: crate::vma::Allocator,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_allocation_create_info: *const crate::vma::AllocationCreateInfo,
            p_memory_type_index: *mut u32,
        ) -> vk::Result;
    }
    let mut p_memory_type_index = ::std::mem::zeroed();
    let result = vmaFindMemoryTypeIndexForBufferInfo(
        allocator,
        p_buffer_create_info,
        p_allocation_create_info,
        &mut p_memory_type_index,
    );
    if result == vk::Result::SUCCESS {
        Ok(p_memory_type_index)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Helps to find memoryTypeIndex, given VkImageCreateInfo and VmaAllocationCreateInfo.\n\nIt can be useful e.g. to determine value to be used as VmaPoolCreateInfo::memoryTypeIndex.\nIt internally creates a temporary, dummy image that never has memory bound."]
pub unsafe fn find_memory_type_index_for_image_info(
    allocator: crate::vma::Allocator,
    p_image_create_info: &vk::ImageCreateInfo,
    p_allocation_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<u32, vk::Result> {
    extern "C" {
        fn vmaFindMemoryTypeIndexForImageInfo(
            allocator: crate::vma::Allocator,
            p_image_create_info: *const vk::ImageCreateInfo,
            p_allocation_create_info: *const crate::vma::AllocationCreateInfo,
            p_memory_type_index: *mut u32,
        ) -> vk::Result;
    }
    let mut p_memory_type_index = ::std::mem::zeroed();
    let result = vmaFindMemoryTypeIndexForImageInfo(
        allocator,
        p_image_create_info,
        p_allocation_create_info,
        &mut p_memory_type_index,
    );
    if result == vk::Result::SUCCESS {
        Ok(p_memory_type_index)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Allocates Vulkan device memory and creates #VmaPool object.\n\n\\param allocator Allocator object.\n\\param pCreateInfo Parameters of pool to create.\n\\param[out] pPool Handle to created pool."]
pub unsafe fn create_pool(
    allocator: crate::vma::Allocator,
    p_create_info: &crate::vma::PoolCreateInfo,
) -> Result<crate::vma::Pool, vk::Result> {
    extern "C" {
        fn vmaCreatePool(
            allocator: crate::vma::Allocator,
            p_create_info: *const crate::vma::PoolCreateInfo,
            p_pool: *mut crate::vma::Pool,
        ) -> vk::Result;
    }
    let mut p_pool = ::std::mem::zeroed();
    let result = vmaCreatePool(allocator, p_create_info, &mut p_pool);
    if result == vk::Result::SUCCESS {
        Ok(p_pool)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Destroys #VmaPool object and frees Vulkan device memory."]
pub unsafe fn destroy_pool(allocator: crate::vma::Allocator, pool: crate::vma::Pool) {
    extern "C" {
        fn vmaDestroyPool(allocator: crate::vma::Allocator, pool: crate::vma::Pool);
    }
    vmaDestroyPool(allocator, pool);
}
#[doc = "\\brief Retrieves statistics of existing #VmaPool object.\n\n\\param allocator Allocator object.\n\\param pool Pool object.\n\\param[out] pPoolStats Statistics of specified pool."]
pub unsafe fn get_pool_statistics(
    allocator: crate::vma::Allocator,
    pool: crate::vma::Pool,
) -> crate::vma::Statistics {
    extern "C" {
        fn vmaGetPoolStatistics(
            allocator: crate::vma::Allocator,
            pool: crate::vma::Pool,
            p_pool_stats: *mut crate::vma::Statistics,
        );
    }
    let mut p_pool_stats = ::std::mem::zeroed();
    vmaGetPoolStatistics(allocator, pool, &mut p_pool_stats);
    p_pool_stats
}
#[doc = "\\brief Retrieves detailed statistics of existing #VmaPool object.\n\n\\param allocator Allocator object.\n\\param pool Pool object.\n\\param[out] pPoolStats Statistics of specified pool."]
pub unsafe fn calculate_pool_statistics(
    allocator: crate::vma::Allocator,
    pool: crate::vma::Pool,
) -> crate::vma::DetailedStatistics {
    extern "C" {
        fn vmaCalculatePoolStatistics(
            allocator: crate::vma::Allocator,
            pool: crate::vma::Pool,
            p_pool_stats: *mut crate::vma::DetailedStatistics,
        );
    }
    let mut p_pool_stats = ::std::mem::zeroed();
    vmaCalculatePoolStatistics(allocator, pool, &mut p_pool_stats);
    p_pool_stats
}
#[doc = "\\brief Checks magic number in margins around all allocations in given memory pool in search for corruptions.\n\nCorruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to nonzero,\n`VMA_DEBUG_MARGIN` is defined to nonzero and the pool is created in memory type that is\n`HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref debugging_memory_usage_corruption_detection).\n\nPossible return values:\n\n- `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for specified pool.\n- `VK_SUCCESS` - corruption detection has been performed and succeeded.\n- `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions around one of the allocations.\n  `VMA_ASSERT` is also fired in that case.\n- Other value: Error returned by Vulkan, e.g. memory mapping failure."]
pub unsafe fn check_pool_corruption(
    allocator: crate::vma::Allocator,
    pool: crate::vma::Pool,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaCheckPoolCorruption(
            allocator: crate::vma::Allocator,
            pool: crate::vma::Pool,
        ) -> vk::Result;
    }
    let result = vmaCheckPoolCorruption(allocator, pool);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Retrieves name of a custom pool.\n\nAfter the call `ppName` is either null or points to an internally-owned null-terminated string\ncontaining name of the pool that was previously set. The pointer becomes invalid when the pool is\ndestroyed or its name is changed using vmaSetPoolName()."]
pub unsafe fn get_pool_name(
    allocator: crate::vma::Allocator,
    pool: crate::vma::Pool,
) -> Option<::std::ffi::CString> {
    extern "C" {
        fn vmaGetPoolName(
            allocator: crate::vma::Allocator,
            pool: crate::vma::Pool,
            pp_name: *mut *const ::std::ffi::c_char,
        );
    }
    let mut pp_name = ::std::mem::zeroed();
    vmaGetPoolName(allocator, pool, &mut pp_name);
    if !pp_name.is_null() {
        Some(::std::ffi::CStr::from_ptr(pp_name).to_owned())
    } else {
        None
    }
}
#[doc = "\\brief Sets name of a custom pool.\n\n`pName` can be either null or pointer to a null-terminated string with new name for the pool.\nFunction makes internal copy of the string, so it can be changed or freed immediately after this call."]
pub unsafe fn set_pool_name(
    allocator: crate::vma::Allocator,
    pool: crate::vma::Pool,
    p_name: Option<&::std::ffi::CStr>,
) {
    extern "C" {
        fn vmaSetPoolName(
            allocator: crate::vma::Allocator,
            pool: crate::vma::Pool,
            p_name: *const ::std::ffi::c_char,
        );
    }
    vmaSetPoolName(
        allocator,
        pool,
        p_name.map_or(::std::ptr::null(), |s| s.as_ptr()),
    );
}
#[doc = "\\brief General purpose memory allocation.\n\n\\param allocator\n\\param pVkMemoryRequirements\n\\param pCreateInfo\n\\param[out] pAllocation Handle to allocated memory.\n\\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().\n\nYou should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().\n\nIt is recommended to use vmaAllocateMemoryForBuffer(), vmaAllocateMemoryForImage(),\nvmaCreateBuffer(), vmaCreateImage() instead whenever possible."]
pub unsafe fn allocate_memory<'a>(
    allocator: crate::vma::Allocator,
    p_vk_memory_requirements: &vk::MemoryRequirements,
    p_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<(crate::vma::Allocation, crate::vma::AllocationInfo<'a>), vk::Result> {
    extern "C" {
        fn vmaAllocateMemory<'a>(
            allocator: crate::vma::Allocator,
            p_vk_memory_requirements: *const vk::MemoryRequirements,
            p_create_info: *const crate::vma::AllocationCreateInfo,
            p_allocation: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_allocation_info = ::std::mem::zeroed();
    let result = vmaAllocateMemory(
        allocator,
        p_vk_memory_requirements,
        p_create_info,
        &mut p_allocation,
        &mut p_allocation_info,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_allocation, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "\\brief General purpose memory allocation for multiple allocation objects at once.\n\n\\param allocator Allocator object.\n\\param pVkMemoryRequirements Memory requirements for each allocation.\n\\param pCreateInfo Creation parameters for each allocation.\n\\param allocationCount Number of allocations to make.\n\\param[out] pAllocations Pointer to array that will be filled with handles to created allocations.\n\\param[out] pAllocationInfo Optional. Pointer to array that will be filled with parameters of created allocations.\n\nYou should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().\n\nWord \"pages\" is just a suggestion to use this function to allocate pieces of memory needed for sparse binding.\nIt is just a general purpose allocation function able to make multiple allocations at once.\nIt may be internally optimized to be more efficient than calling vmaAllocateMemory() `allocationCount` times.\n\nAll allocations are made using same parameters. All of them are created out of the same memory pool and type.\nIf any allocation fails, all allocations already made within this function call are also freed, so that when\nreturned result is not `VK_SUCCESS`, `pAllocation` array is always entirely filled with `VK_NULL_HANDLE`."]
pub unsafe fn allocate_memory_pages<'a>(
    allocator: crate::vma::Allocator,
    p_vk_memory_requirements: &[vk::MemoryRequirements],
    p_create_info: &[crate::vma::AllocationCreateInfo],
) -> Result<
    (
        Vec<crate::vma::Allocation>,
        Vec<crate::vma::AllocationInfo<'a>>,
    ),
    vk::Result,
> {
    extern "C" {
        fn vmaAllocateMemoryPages<'a>(
            allocator: crate::vma::Allocator,
            p_vk_memory_requirements: *const vk::MemoryRequirements,
            p_create_info: *const crate::vma::AllocationCreateInfo,
            allocation_count: usize,
            p_allocations: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let p_allocations_len = p_vk_memory_requirements.len();
    let mut p_allocations = vec![::std::mem::zeroed(); p_allocations_len as _];
    let p_allocation_info_len = p_vk_memory_requirements.len();
    let mut p_allocation_info = vec![::std::mem::zeroed(); p_allocation_info_len as _];
    let result = vmaAllocateMemoryPages(
        allocator,
        p_vk_memory_requirements.as_ptr(),
        p_create_info.as_ptr(),
        p_vk_memory_requirements.len() as _,
        p_allocations.as_mut_ptr(),
        p_allocation_info.as_mut_ptr(),
    );
    if result == vk::Result::SUCCESS {
        Ok((p_allocations, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "\\brief Allocates memory suitable for given `VkBuffer`.\n\n\\param allocator\n\\param buffer\n\\param pCreateInfo\n\\param[out] pAllocation Handle to allocated memory.\n\\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().\n\nIt only creates #VmaAllocation. To bind the memory to the buffer, use vmaBindBufferMemory().\n\nThis is a special-purpose function. In most cases you should use vmaCreateBuffer().\n\nYou must free the allocation using vmaFreeMemory() when no longer needed."]
pub unsafe fn allocate_memory_for_buffer<'a>(
    allocator: crate::vma::Allocator,
    buffer: vk::Buffer,
    p_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<(crate::vma::Allocation, crate::vma::AllocationInfo<'a>), vk::Result> {
    extern "C" {
        fn vmaAllocateMemoryForBuffer<'a>(
            allocator: crate::vma::Allocator,
            buffer: vk::Buffer,
            p_create_info: *const crate::vma::AllocationCreateInfo,
            p_allocation: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_allocation_info = ::std::mem::zeroed();
    let result = vmaAllocateMemoryForBuffer(
        allocator,
        buffer,
        p_create_info,
        &mut p_allocation,
        &mut p_allocation_info,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_allocation, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "\\brief Allocates memory suitable for given `VkImage`.\n\n\\param allocator\n\\param image\n\\param pCreateInfo\n\\param[out] pAllocation Handle to allocated memory.\n\\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().\n\nIt only creates #VmaAllocation. To bind the memory to the buffer, use vmaBindImageMemory().\n\nThis is a special-purpose function. In most cases you should use vmaCreateImage().\n\nYou must free the allocation using vmaFreeMemory() when no longer needed."]
pub unsafe fn allocate_memory_for_image<'a>(
    allocator: crate::vma::Allocator,
    image: vk::Image,
    p_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<(crate::vma::Allocation, crate::vma::AllocationInfo<'a>), vk::Result> {
    extern "C" {
        fn vmaAllocateMemoryForImage<'a>(
            allocator: crate::vma::Allocator,
            image: vk::Image,
            p_create_info: *const crate::vma::AllocationCreateInfo,
            p_allocation: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_allocation_info = ::std::mem::zeroed();
    let result = vmaAllocateMemoryForImage(
        allocator,
        image,
        p_create_info,
        &mut p_allocation,
        &mut p_allocation_info,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_allocation, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "\\brief Frees memory previously allocated using vmaAllocateMemory(), vmaAllocateMemoryForBuffer(), or vmaAllocateMemoryForImage().\n\nPassing `VK_NULL_HANDLE` as `allocation` is valid. Such function call is just skipped."]
pub unsafe fn free_memory(allocator: crate::vma::Allocator, allocation: crate::vma::Allocation) {
    extern "C" {
        fn vmaFreeMemory(allocator: crate::vma::Allocator, allocation: crate::vma::Allocation);
    }
    vmaFreeMemory(allocator, allocation);
}
#[doc = "\\brief Frees memory and destroys multiple allocations.\n\nWord \"pages\" is just a suggestion to use this function to free pieces of memory used for sparse binding.\nIt is just a general purpose function to free memory and destroy allocations made using e.g. vmaAllocateMemory(),\nvmaAllocateMemoryPages() and other functions.\nIt may be internally optimized to be more efficient than calling vmaFreeMemory() `allocationCount` times.\n\nAllocations in `pAllocations` array can come from any memory pools and types.\nPassing `VK_NULL_HANDLE` as elements of `pAllocations` array is valid. Such entries are just skipped."]
pub unsafe fn free_memory_pages(
    allocator: crate::vma::Allocator,
    p_allocations: &[crate::vma::Allocation],
) {
    extern "C" {
        fn vmaFreeMemoryPages(
            allocator: crate::vma::Allocator,
            allocation_count: usize,
            p_allocations: *const crate::vma::Allocation,
        );
    }
    vmaFreeMemoryPages(allocator, p_allocations.len() as _, p_allocations.as_ptr());
}
#[doc = "\\brief Returns current information about specified allocation.\n\nCurrent parameters of given allocation are returned in `pAllocationInfo`.\n\nAlthough this function doesn't lock any mutex, so it should be quite efficient,\nyou should avoid calling it too often.\nYou can retrieve same VmaAllocationInfo structure while creating your resource, from function\nvmaCreateBuffer(), vmaCreateImage(). You can remember it if you are sure parameters don't change\n(e.g. due to defragmentation).\n\nThere is also a new function vmaGetAllocationInfo2() that offers extended information\nabout the allocation, returned using new structure #VmaAllocationInfo2."]
pub unsafe fn get_allocation_info<'a>(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
) -> crate::vma::AllocationInfo<'a> {
    extern "C" {
        fn vmaGetAllocationInfo<'a>(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        );
    }
    let mut p_allocation_info = ::std::mem::zeroed();
    vmaGetAllocationInfo(allocator, allocation, &mut p_allocation_info);
    p_allocation_info
}
#[doc = "\\brief Returns extended information about specified allocation.\n\nCurrent parameters of given allocation are returned in `pAllocationInfo`.\nExtended parameters in structure #VmaAllocationInfo2 include memory block size\nand a flag telling whether the allocation has dedicated memory.\nIt can be useful e.g. for interop with OpenGL."]
pub unsafe fn get_allocation_info_2<'a>(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
) -> crate::vma::AllocationInfo2<'a> {
    extern "C" {
        fn vmaGetAllocationInfo2<'a>(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo2<'a>,
        );
    }
    let mut p_allocation_info = ::std::mem::zeroed();
    vmaGetAllocationInfo2(allocator, allocation, &mut p_allocation_info);
    p_allocation_info
}
#[doc = "\\brief Sets pUserData in given allocation to new value.\n\nThe value of pointer `pUserData` is copied to allocation's `pUserData`.\nIt is opaque, so you can use it however you want - e.g.\nas a pointer, ordinal number or some handle to you own data."]
pub unsafe fn set_allocation_user_data(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    p_user_data: *mut ::std::ffi::c_void,
) {
    extern "C" {
        fn vmaSetAllocationUserData(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_user_data: *mut ::std::ffi::c_void,
        );
    }
    vmaSetAllocationUserData(allocator, allocation, p_user_data);
}
#[doc = "\\brief Sets pName in given allocation to new value.\n\n`pName` must be either null, or pointer to a null-terminated string. The function\nmakes local copy of the string and sets it as allocation's `pName`. String\npassed as pName doesn't need to be valid for whole lifetime of the allocation -\nyou can free it after this call. String previously pointed by allocation's\n`pName` is freed from memory."]
pub unsafe fn set_allocation_name(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    p_name: Option<&::std::ffi::CStr>,
) {
    extern "C" {
        fn vmaSetAllocationName(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_name: *const ::std::ffi::c_char,
        );
    }
    vmaSetAllocationName(
        allocator,
        allocation,
        p_name.map_or(::std::ptr::null(), |s| s.as_ptr()),
    );
}
#[doc = "\\brief Given an allocation, returns Property Flags of its memory type.\n\nThis is just a convenience function. Same information can be obtained using\nvmaGetAllocationInfo() + vmaGetMemoryProperties()."]
pub unsafe fn get_allocation_memory_properties(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
) -> vk::MemoryPropertyFlags {
    extern "C" {
        fn vmaGetAllocationMemoryProperties(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_flags: *mut vk::MemoryPropertyFlags,
        );
    }
    let mut p_flags = ::std::mem::zeroed();
    vmaGetAllocationMemoryProperties(allocator, allocation, &mut p_flags);
    p_flags
}
#[doc = "\\brief Maps memory represented by given allocation and returns pointer to it.\n\nMaps memory represented by given allocation to make it accessible to CPU code.\nWhen succeeded, `*ppData` contains pointer to first byte of this memory.\n\n\\warning\nIf the allocation is part of a bigger `VkDeviceMemory` block, returned pointer is\ncorrectly offsetted to the beginning of region assigned to this particular allocation.\nUnlike the result of `vkMapMemory`, it points to the allocation, not to the beginning of the whole block.\nYou should not add VmaAllocationInfo::offset to it!\n\nMapping is internally reference-counted and synchronized, so despite raw Vulkan\nfunction `vkMapMemory()` cannot be used to map same block of `VkDeviceMemory`\nmultiple times simultaneously, it is safe to call this function on allocations\nassigned to the same memory block. Actual Vulkan memory will be mapped on first\nmapping and unmapped on last unmapping.\n\nIf the function succeeded, you must call vmaUnmapMemory() to unmap the\nallocation when mapping is no longer needed or before freeing the allocation, at\nthe latest.\n\nIt also safe to call this function multiple times on the same allocation. You\nmust call vmaUnmapMemory() same number of times as you called vmaMapMemory().\n\nIt is also safe to call this function on allocation created with\n#VMA_ALLOCATION_CREATE_MAPPED_BIT flag. Its memory stays mapped all the time.\nYou must still call vmaUnmapMemory() same number of times as you called\nvmaMapMemory(). You must not call vmaUnmapMemory() additional time to free the\n\"0-th\" mapping made automatically due to #VMA_ALLOCATION_CREATE_MAPPED_BIT flag.\n\nThis function fails when used on allocation made in memory type that is not\n`HOST_VISIBLE`.\n\nThis function doesn't automatically flush or invalidate caches.\nIf the allocation is made from a memory types that is not `HOST_COHERENT`,\nyou also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification."]
pub unsafe fn map_memory(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
) -> Result<*mut ::std::ffi::c_void, vk::Result> {
    extern "C" {
        fn vmaMapMemory(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            pp_data: *mut *mut ::std::ffi::c_void,
        ) -> vk::Result;
    }
    let mut pp_data = ::std::mem::zeroed();
    let result = vmaMapMemory(allocator, allocation, &mut pp_data);
    if result == vk::Result::SUCCESS {
        Ok(pp_data)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Unmaps memory represented by given allocation, mapped previously using vmaMapMemory().\n\nFor details, see description of vmaMapMemory().\n\nThis function doesn't automatically flush or invalidate caches.\nIf the allocation is made from a memory types that is not `HOST_COHERENT`,\nyou also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification."]
pub unsafe fn unmap_memory(allocator: crate::vma::Allocator, allocation: crate::vma::Allocation) {
    extern "C" {
        fn vmaUnmapMemory(allocator: crate::vma::Allocator, allocation: crate::vma::Allocation);
    }
    vmaUnmapMemory(allocator, allocation);
}
#[doc = "\\brief Flushes memory of given allocation.\n\nCalls `vkFlushMappedMemoryRanges()` for memory associated with given range of given allocation.\nIt needs to be called after writing to a mapped memory for memory types that are not `HOST_COHERENT`.\nUnmap operation doesn't do that automatically.\n\n- `offset` must be relative to the beginning of allocation.\n- `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given allocation.\n- `offset` and `size` don't have to be aligned.\n  They are internally rounded down/up to multiply of `nonCoherentAtomSize`.\n- If `size` is 0, this call is ignored.\n- If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is `HOST_COHERENT`,\n  this call is ignored.\n\nWarning! `offset` and `size` are relative to the contents of given `allocation`.\nIf you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.\nDo not pass allocation's offset as `offset`!!!\n\nThis function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is\ncalled, otherwise `VK_SUCCESS`."]
pub unsafe fn flush_allocation(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaFlushAllocation(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            offset: vk::DeviceSize,
            size: vk::DeviceSize,
        ) -> vk::Result;
    }
    let result = vmaFlushAllocation(allocator, allocation, offset, size);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Invalidates memory of given allocation.\n\nCalls `vkInvalidateMappedMemoryRanges()` for memory associated with given range of given allocation.\nIt needs to be called before reading from a mapped memory for memory types that are not `HOST_COHERENT`.\nMap operation doesn't do that automatically.\n\n- `offset` must be relative to the beginning of allocation.\n- `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given allocation.\n- `offset` and `size` don't have to be aligned.\n  They are internally rounded down/up to multiply of `nonCoherentAtomSize`.\n- If `size` is 0, this call is ignored.\n- If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is `HOST_COHERENT`,\n  this call is ignored.\n\nWarning! `offset` and `size` are relative to the contents of given `allocation`.\nIf you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.\nDo not pass allocation's offset as `offset`!!!\n\nThis function returns the `VkResult` from `vkInvalidateMappedMemoryRanges` if\nit is called, otherwise `VK_SUCCESS`."]
pub unsafe fn invalidate_allocation(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaInvalidateAllocation(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            offset: vk::DeviceSize,
            size: vk::DeviceSize,
        ) -> vk::Result;
    }
    let result = vmaInvalidateAllocation(allocator, allocation, offset, size);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Flushes memory of given set of allocations.\n\nCalls `vkFlushMappedMemoryRanges()` for memory associated with given ranges of given allocations.\nFor more information, see documentation of vmaFlushAllocation().\n\n\\param allocator\n\\param allocationCount\n\\param allocations\n\\param offsets If not null, it must point to an array of offsets of regions to flush, relative to the beginning of respective allocations. Null means all offsets are zero.\n\\param sizes If not null, it must point to an array of sizes of regions to flush in respective allocations. Null means `VK_WHOLE_SIZE` for all allocations.\n\nThis function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is\ncalled, otherwise `VK_SUCCESS`."]
pub unsafe fn flush_allocations(
    allocator: crate::vma::Allocator,
    allocations: &[crate::vma::Allocation],
    offsets: &[vk::DeviceSize],
    sizes: &[vk::DeviceSize],
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaFlushAllocations(
            allocator: crate::vma::Allocator,
            allocation_count: u32,
            allocations: *const crate::vma::Allocation,
            offsets: *const vk::DeviceSize,
            sizes: *const vk::DeviceSize,
        ) -> vk::Result;
    }
    let result = vmaFlushAllocations(
        allocator,
        allocations.len() as _,
        allocations.as_ptr(),
        offsets.as_ptr(),
        sizes.as_ptr(),
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Invalidates memory of given set of allocations.\n\nCalls `vkInvalidateMappedMemoryRanges()` for memory associated with given ranges of given allocations.\nFor more information, see documentation of vmaInvalidateAllocation().\n\n\\param allocator\n\\param allocationCount\n\\param allocations\n\\param offsets If not null, it must point to an array of offsets of regions to flush, relative to the beginning of respective allocations. Null means all offsets are zero.\n\\param sizes If not null, it must point to an array of sizes of regions to flush in respective allocations. Null means `VK_WHOLE_SIZE` for all allocations.\n\nThis function returns the `VkResult` from `vkInvalidateMappedMemoryRanges` if it is\ncalled, otherwise `VK_SUCCESS`."]
pub unsafe fn invalidate_allocations(
    allocator: crate::vma::Allocator,
    allocations: &[crate::vma::Allocation],
    offsets: &[vk::DeviceSize],
    sizes: &[vk::DeviceSize],
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaInvalidateAllocations(
            allocator: crate::vma::Allocator,
            allocation_count: u32,
            allocations: *const crate::vma::Allocation,
            offsets: *const vk::DeviceSize,
            sizes: *const vk::DeviceSize,
        ) -> vk::Result;
    }
    let result = vmaInvalidateAllocations(
        allocator,
        allocations.len() as _,
        allocations.as_ptr(),
        offsets.as_ptr(),
        sizes.as_ptr(),
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Maps the allocation temporarily if needed, copies data from specified host pointer to it, and flushes the memory from the host caches if needed.\n\n\\param allocator\n\\param pSrcHostPointer Pointer to the host data that become source of the copy.\n\\param dstAllocation   Handle to the allocation that becomes destination of the copy.\n\\param dstAllocationLocalOffset  Offset within `dstAllocation` where to write copied data, in bytes.\n\\param size   Number of bytes to copy.\n\nThis is a convenience function that allows to copy data from a host pointer to an allocation easily.\nSame behavior can be achieved by calling vmaMapMemory(), `memcpy()`, vmaUnmapMemory(), vmaFlushAllocation().\n\nThis function can be called only for allocations created in a memory type that has `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` flag.\nIt can be ensured e.g. by using #VMA_MEMORY_USAGE_AUTO and #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or\n#VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT.\nOtherwise, the function will fail and generate a Validation Layers error.\n\n`dstAllocationLocalOffset` is relative to the contents of given `dstAllocation`.\nIf you mean whole allocation, you should pass 0.\nDo not pass allocation's offset within device memory block this parameter!"]
pub unsafe fn copy_memory_to_allocation(
    allocator: crate::vma::Allocator,
    p_src_host_pointer: &[u8],
    dst_allocation: crate::vma::Allocation,
    dst_allocation_local_offset: vk::DeviceSize,
    size: vk::DeviceSize,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaCopyMemoryToAllocation(
            allocator: crate::vma::Allocator,
            p_src_host_pointer: *const ::std::ffi::c_void,
            dst_allocation: crate::vma::Allocation,
            dst_allocation_local_offset: vk::DeviceSize,
            size: vk::DeviceSize,
        ) -> vk::Result;
    }
    let result = vmaCopyMemoryToAllocation(
        allocator,
        p_src_host_pointer.as_ptr().cast(),
        dst_allocation,
        dst_allocation_local_offset,
        size,
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Invalidates memory in the host caches if needed, maps the allocation temporarily if needed, and copies data from it to a specified host pointer.\n\n\\param allocator\n\\param srcAllocation   Handle to the allocation that becomes source of the copy.\n\\param srcAllocationLocalOffset  Offset within `srcAllocation` where to read copied data, in bytes.\n\\param pDstHostPointer Pointer to the host memory that become destination of the copy.\n\\param size   Number of bytes to copy.\n\nThis is a convenience function that allows to copy data from an allocation to a host pointer easily.\nSame behavior can be achieved by calling vmaInvalidateAllocation(), vmaMapMemory(), `memcpy()`, vmaUnmapMemory().\n\nThis function should be called only for allocations created in a memory type that has `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`\nand `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` flag.\nIt can be ensured e.g. by using #VMA_MEMORY_USAGE_AUTO and #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT.\nOtherwise, the function may fail and generate a Validation Layers error.\nIt may also work very slowly when reading from an uncached memory.\n\n`srcAllocationLocalOffset` is relative to the contents of given `srcAllocation`.\nIf you mean whole allocation, you should pass 0.\nDo not pass allocation's offset within device memory block as this parameter!"]
pub unsafe fn copy_allocation_to_memory(
    allocator: crate::vma::Allocator,
    src_allocation: crate::vma::Allocation,
    src_allocation_local_offset: vk::DeviceSize,
    p_dst_host_pointer: &mut [u8],
    size: vk::DeviceSize,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaCopyAllocationToMemory(
            allocator: crate::vma::Allocator,
            src_allocation: crate::vma::Allocation,
            src_allocation_local_offset: vk::DeviceSize,
            p_dst_host_pointer: *mut ::std::ffi::c_void,
            size: vk::DeviceSize,
        ) -> vk::Result;
    }
    let result = vmaCopyAllocationToMemory(
        allocator,
        src_allocation,
        src_allocation_local_offset,
        p_dst_host_pointer.as_mut_ptr().cast(),
        size,
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Checks magic number in margins around all allocations in given memory types (in both default and custom pools) in search for corruptions.\n\n\\param allocator\n\\param memoryTypeBits Bit mask, where each bit set means that a memory type with that index should be checked.\n\nCorruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to nonzero,\n`VMA_DEBUG_MARGIN` is defined to nonzero and only for memory types that are\n`HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref debugging_memory_usage_corruption_detection).\n\nPossible return values:\n\n- `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for any of specified memory types.\n- `VK_SUCCESS` - corruption detection has been performed and succeeded.\n- `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions around one of the allocations.\n  `VMA_ASSERT` is also fired in that case.\n- Other value: Error returned by Vulkan, e.g. memory mapping failure."]
pub unsafe fn check_corruption(
    allocator: crate::vma::Allocator,
    memory_type_bits: u32,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaCheckCorruption(
            allocator: crate::vma::Allocator,
            memory_type_bits: u32,
        ) -> vk::Result;
    }
    let result = vmaCheckCorruption(allocator, memory_type_bits);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Begins defragmentation process.\n\n\\param allocator Allocator object.\n\\param pInfo Structure filled with parameters of defragmentation.\n\\param[out] pContext Context object that must be passed to vmaEndDefragmentation() to finish defragmentation.\n\\returns\n- `VK_SUCCESS` if defragmentation can begin.\n- `VK_ERROR_FEATURE_NOT_PRESENT` if defragmentation is not supported.\n\nFor more information about defragmentation, see documentation chapter:\n[Defragmentation](@ref defragmentation)."]
pub unsafe fn begin_defragmentation(
    allocator: crate::vma::Allocator,
    p_info: &crate::vma::DefragmentationInfo,
) -> Result<crate::vma::DefragmentationContext, vk::Result> {
    extern "C" {
        fn vmaBeginDefragmentation(
            allocator: crate::vma::Allocator,
            p_info: *const crate::vma::DefragmentationInfo,
            p_context: *mut crate::vma::DefragmentationContext,
        ) -> vk::Result;
    }
    let mut p_context = ::std::mem::zeroed();
    let result = vmaBeginDefragmentation(allocator, p_info, &mut p_context);
    if result == vk::Result::SUCCESS {
        Ok(p_context)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Ends defragmentation process.\n\n\\param allocator Allocator object.\n\\param context Context object that has been created by vmaBeginDefragmentation().\n\\param[out] pStats Optional stats for the defragmentation. Can be null.\n\nUse this function to finish defragmentation started by vmaBeginDefragmentation()."]
pub unsafe fn end_defragmentation(
    allocator: crate::vma::Allocator,
    context: crate::vma::DefragmentationContext,
) -> crate::vma::DefragmentationStats {
    extern "C" {
        fn vmaEndDefragmentation(
            allocator: crate::vma::Allocator,
            context: crate::vma::DefragmentationContext,
            p_stats: *mut crate::vma::DefragmentationStats,
        );
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaEndDefragmentation(allocator, context, &mut p_stats);
    p_stats
}
#[doc = "\\brief Starts single defragmentation pass.\n\n\\param allocator Allocator object.\n\\param context Context object that has been created by vmaBeginDefragmentation().\n\\param[out] pPassInfo Computed information for current pass.\n\\returns\n- `VK_SUCCESS` if no more moves are possible. Then you can omit call to vmaEndDefragmentationPass() and simply end whole defragmentation.\n- `VK_INCOMPLETE` if there are pending moves returned in `pPassInfo`. You need to perform them, call vmaEndDefragmentationPass(),\n  and then preferably try another pass with vmaBeginDefragmentationPass()."]
pub unsafe fn begin_defragmentation_pass<'a>(
    allocator: crate::vma::Allocator,
    context: crate::vma::DefragmentationContext,
) -> Result<crate::vma::DefragmentationPassMoveInfo<'a>, vk::Result> {
    extern "C" {
        fn vmaBeginDefragmentationPass<'a>(
            allocator: crate::vma::Allocator,
            context: crate::vma::DefragmentationContext,
            p_pass_info: *mut crate::vma::DefragmentationPassMoveInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_pass_info = ::std::mem::zeroed();
    let result = vmaBeginDefragmentationPass(allocator, context, &mut p_pass_info);
    if result == vk::Result::SUCCESS {
        Ok(p_pass_info)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Ends single defragmentation pass.\n\n\\param allocator Allocator object.\n\\param context Context object that has been created by vmaBeginDefragmentation().\n\\param pPassInfo Computed information for current pass filled by vmaBeginDefragmentationPass() and possibly modified by you.\n\nReturns `VK_SUCCESS` if no more moves are possible or `VK_INCOMPLETE` if more defragmentations are possible.\n\nEnds incremental defragmentation pass and commits all defragmentation moves from `pPassInfo`.\nAfter this call:\n\n- Allocations at `pPassInfo[i].srcAllocation` that had `pPassInfo[i].operation ==` #VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY\n  (which is the default) will be pointing to the new destination place.\n- Allocation at `pPassInfo[i].srcAllocation` that had `pPassInfo[i].operation ==` #VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY\n  will be freed.\n\nIf no more moves are possible you can end whole defragmentation."]
pub unsafe fn end_defragmentation_pass<'a>(
    allocator: crate::vma::Allocator,
    context: crate::vma::DefragmentationContext,
) -> Result<crate::vma::DefragmentationPassMoveInfo<'a>, vk::Result> {
    extern "C" {
        fn vmaEndDefragmentationPass<'a>(
            allocator: crate::vma::Allocator,
            context: crate::vma::DefragmentationContext,
            p_pass_info: *mut crate::vma::DefragmentationPassMoveInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_pass_info = ::std::mem::zeroed();
    let result = vmaEndDefragmentationPass(allocator, context, &mut p_pass_info);
    if result == vk::Result::SUCCESS {
        Ok(p_pass_info)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Binds buffer to allocation.\n\nBinds specified buffer to region of memory represented by specified allocation.\nGets `VkDeviceMemory` handle and offset from the allocation.\nIf you want to create a buffer, allocate memory for it and bind them together separately,\nyou should use this function for binding instead of standard `vkBindBufferMemory()`,\nbecause it ensures proper synchronization so that when a `VkDeviceMemory` object is used by multiple\nallocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple threads simultaneously\n(which is illegal in Vulkan).\n\nIt is recommended to use function vmaCreateBuffer() instead of this one."]
pub unsafe fn bind_buffer_memory(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    buffer: vk::Buffer,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindBufferMemory(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            buffer: vk::Buffer,
        ) -> vk::Result;
    }
    let result = vmaBindBufferMemory(allocator, allocation, buffer);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Binds buffer to allocation with additional parameters.\n\n\\param allocator\n\\param allocation\n\\param allocationLocalOffset Additional offset to be added while binding, relative to the beginning of the `allocation`. Normally it should be 0.\n\\param buffer\n\\param pNext A chain of structures to be attached to `VkBindBufferMemoryInfoKHR` structure used internally. Normally it should be null.\n\nThis function is similar to vmaBindBufferMemory(), but it provides additional parameters.\n\nIf `pNext` is not null, #VmaAllocator object must have been created with #VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT flag\nor with VmaAllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call fails."]
pub unsafe fn bind_buffer_memory_2(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    allocation_local_offset: vk::DeviceSize,
    buffer: vk::Buffer,
    p_next: Option<&impl vk::ExtendsBindBufferMemoryInfo>,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindBufferMemory2(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            allocation_local_offset: vk::DeviceSize,
            buffer: vk::Buffer,
            p_next: *const ::std::ffi::c_void,
        ) -> vk::Result;
    }
    let result = vmaBindBufferMemory2(
        allocator,
        allocation,
        allocation_local_offset,
        buffer,
        p_next.map_or(::std::ptr::null(), |p| p as *const _ as *const _),
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Binds image to allocation.\n\nBinds specified image to region of memory represented by specified allocation.\nGets `VkDeviceMemory` handle and offset from the allocation.\nIf you want to create an image, allocate memory for it and bind them together separately,\nyou should use this function for binding instead of standard `vkBindImageMemory()`,\nbecause it ensures proper synchronization so that when a `VkDeviceMemory` object is used by multiple\nallocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple threads simultaneously\n(which is illegal in Vulkan).\n\nIt is recommended to use function vmaCreateImage() instead of this one."]
pub unsafe fn bind_image_memory(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    image: vk::Image,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindImageMemory(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            image: vk::Image,
        ) -> vk::Result;
    }
    let result = vmaBindImageMemory(allocator, allocation, image);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Binds image to allocation with additional parameters.\n\n\\param allocator\n\\param allocation\n\\param allocationLocalOffset Additional offset to be added while binding, relative to the beginning of the `allocation`. Normally it should be 0.\n\\param image\n\\param pNext A chain of structures to be attached to `VkBindImageMemoryInfoKHR` structure used internally. Normally it should be null.\n\nThis function is similar to vmaBindImageMemory(), but it provides additional parameters.\n\nIf `pNext` is not null, #VmaAllocator object must have been created with #VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT flag\nor with VmaAllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call fails."]
pub unsafe fn bind_image_memory_2(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    allocation_local_offset: vk::DeviceSize,
    image: vk::Image,
    p_next: Option<&impl vk::ExtendsBindImageMemoryInfo>,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindImageMemory2(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            allocation_local_offset: vk::DeviceSize,
            image: vk::Image,
            p_next: *const ::std::ffi::c_void,
        ) -> vk::Result;
    }
    let result = vmaBindImageMemory2(
        allocator,
        allocation,
        allocation_local_offset,
        image,
        p_next.map_or(::std::ptr::null(), |p| p as *const _ as *const _),
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Creates a new `VkBuffer`, allocates and binds memory for it.\n\n\\param allocator\n\\param pBufferCreateInfo\n\\param pAllocationCreateInfo\n\\param[out] pBuffer Buffer that was created.\n\\param[out] pAllocation Allocation that was created.\n\\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().\n\nThis function automatically:\n\n-# Creates buffer.\n-# Allocates appropriate memory for it.\n-# Binds the buffer with the memory.\n\nIf any of these operations fail, buffer and allocation are not created,\nreturned value is negative error code, `*pBuffer` and `*pAllocation` are null.\n\nIf the function succeeded, you must destroy both buffer and allocation when you\nno longer need them using either convenience function vmaDestroyBuffer() or\nseparately, using `vkDestroyBuffer()` and vmaFreeMemory().\n\nIf #VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT flag was used,\nVK_KHR_dedicated_allocation extension is used internally to query driver whether\nit requires or prefers the new buffer to have dedicated allocation. If yes,\nand if dedicated allocation is possible\n(#VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT is not used), it creates dedicated\nallocation for this buffer, just like when using\n#VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.\n\n\\note This function creates a new `VkBuffer`. Sub-allocation of parts of one large buffer,\nalthough recommended as a good practice, is out of scope of this library and could be implemented\nby the user as a higher-level logic on top of VMA."]
pub unsafe fn create_buffer<'a>(
    allocator: crate::vma::Allocator,
    p_buffer_create_info: &vk::BufferCreateInfo,
    p_allocation_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<
    (
        vk::Buffer,
        crate::vma::Allocation,
        crate::vma::AllocationInfo<'a>,
    ),
    vk::Result,
> {
    extern "C" {
        fn vmaCreateBuffer<'a>(
            allocator: crate::vma::Allocator,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_allocation_create_info: *const crate::vma::AllocationCreateInfo,
            p_buffer: *mut vk::Buffer,
            p_allocation: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_buffer = ::std::mem::zeroed();
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_allocation_info = ::std::mem::zeroed();
    let result = vmaCreateBuffer(
        allocator,
        p_buffer_create_info,
        p_allocation_create_info,
        &mut p_buffer,
        &mut p_allocation,
        &mut p_allocation_info,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_buffer, p_allocation, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "\\brief Creates a buffer with additional minimum alignment.\n\nSimilar to vmaCreateBuffer() but provides additional parameter `minAlignment` which allows to specify custom,\nminimum alignment to be used when placing the buffer inside a larger memory block, which may be needed e.g.\nfor interop with OpenGL."]
pub unsafe fn create_buffer_with_alignment<'a>(
    allocator: crate::vma::Allocator,
    p_buffer_create_info: &vk::BufferCreateInfo,
    p_allocation_create_info: &crate::vma::AllocationCreateInfo,
    min_alignment: vk::DeviceSize,
) -> Result<
    (
        vk::Buffer,
        crate::vma::Allocation,
        crate::vma::AllocationInfo<'a>,
    ),
    vk::Result,
> {
    extern "C" {
        fn vmaCreateBufferWithAlignment<'a>(
            allocator: crate::vma::Allocator,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_allocation_create_info: *const crate::vma::AllocationCreateInfo,
            min_alignment: vk::DeviceSize,
            p_buffer: *mut vk::Buffer,
            p_allocation: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_buffer = ::std::mem::zeroed();
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_allocation_info = ::std::mem::zeroed();
    let result = vmaCreateBufferWithAlignment(
        allocator,
        p_buffer_create_info,
        p_allocation_create_info,
        min_alignment,
        &mut p_buffer,
        &mut p_allocation,
        &mut p_allocation_info,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_buffer, p_allocation, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "\\brief Creates a new `VkBuffer`, binds already created memory for it.\n\n\\param allocator\n\\param allocation Allocation that provides memory to be used for binding new buffer to it.\n\\param pBufferCreateInfo\n\\param[out] pBuffer Buffer that was created.\n\nThis function automatically:\n\n-# Creates buffer.\n-# Binds the buffer with the supplied memory.\n\nIf any of these operations fail, buffer is not created,\nreturned value is negative error code and `*pBuffer` is null.\n\nIf the function succeeded, you must destroy the buffer when you\nno longer need it using `vkDestroyBuffer()`. If you want to also destroy the corresponding\nallocation you can use convenience function vmaDestroyBuffer().\n\n\\note There is a new version of this function augmented with parameter `allocationLocalOffset` - see vmaCreateAliasingBuffer2()."]
pub unsafe fn create_aliasing_buffer(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    p_buffer_create_info: &vk::BufferCreateInfo,
) -> Result<vk::Buffer, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingBuffer(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_buffer: *mut vk::Buffer,
        ) -> vk::Result;
    }
    let mut p_buffer = ::std::mem::zeroed();
    let result =
        vmaCreateAliasingBuffer(allocator, allocation, p_buffer_create_info, &mut p_buffer);
    if result == vk::Result::SUCCESS {
        Ok(p_buffer)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Creates a new `VkBuffer`, binds already created memory for it.\n\n\\param allocator\n\\param allocation Allocation that provides memory to be used for binding new buffer to it.\n\\param allocationLocalOffset Additional offset to be added while binding, relative to the beginning of the allocation. Normally it should be 0.\n\\param pBufferCreateInfo \n\\param[out] pBuffer Buffer that was created.\n\nThis function automatically:\n\n-# Creates buffer.\n-# Binds the buffer with the supplied memory.\n\nIf any of these operations fail, buffer is not created,\nreturned value is negative error code and `*pBuffer` is null.\n\nIf the function succeeded, you must destroy the buffer when you\nno longer need it using `vkDestroyBuffer()`. If you want to also destroy the corresponding\nallocation you can use convenience function vmaDestroyBuffer().\n\n\\note This is a new version of the function augmented with parameter `allocationLocalOffset`."]
pub unsafe fn create_aliasing_buffer_2(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    allocation_local_offset: vk::DeviceSize,
    p_buffer_create_info: &vk::BufferCreateInfo,
) -> Result<vk::Buffer, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingBuffer2(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            allocation_local_offset: vk::DeviceSize,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_buffer: *mut vk::Buffer,
        ) -> vk::Result;
    }
    let mut p_buffer = ::std::mem::zeroed();
    let result = vmaCreateAliasingBuffer2(
        allocator,
        allocation,
        allocation_local_offset,
        p_buffer_create_info,
        &mut p_buffer,
    );
    if result == vk::Result::SUCCESS {
        Ok(p_buffer)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Destroys Vulkan buffer and frees allocated memory.\n\nThis is just a convenience function equivalent to:\n\n\\code\nvkDestroyBuffer(device, buffer, allocationCallbacks);\nvmaFreeMemory(allocator, allocation);\n\\endcode\n\nIt is safe to pass null as buffer and/or allocation."]
pub unsafe fn destroy_buffer(
    allocator: crate::vma::Allocator,
    buffer: vk::Buffer,
    allocation: crate::vma::Allocation,
) {
    extern "C" {
        fn vmaDestroyBuffer(
            allocator: crate::vma::Allocator,
            buffer: vk::Buffer,
            allocation: crate::vma::Allocation,
        );
    }
    vmaDestroyBuffer(allocator, buffer, allocation);
}
#[doc = "Function similar to vmaCreateBuffer()."]
pub unsafe fn create_image<'a>(
    allocator: crate::vma::Allocator,
    p_image_create_info: &vk::ImageCreateInfo,
    p_allocation_create_info: &crate::vma::AllocationCreateInfo,
) -> Result<
    (
        vk::Image,
        crate::vma::Allocation,
        crate::vma::AllocationInfo<'a>,
    ),
    vk::Result,
> {
    extern "C" {
        fn vmaCreateImage<'a>(
            allocator: crate::vma::Allocator,
            p_image_create_info: *const vk::ImageCreateInfo,
            p_allocation_create_info: *const crate::vma::AllocationCreateInfo,
            p_image: *mut vk::Image,
            p_allocation: *mut crate::vma::Allocation,
            p_allocation_info: *mut crate::vma::AllocationInfo<'a>,
        ) -> vk::Result;
    }
    let mut p_image = ::std::mem::zeroed();
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_allocation_info = ::std::mem::zeroed();
    let result = vmaCreateImage(
        allocator,
        p_image_create_info,
        p_allocation_create_info,
        &mut p_image,
        &mut p_allocation,
        &mut p_allocation_info,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_image, p_allocation, p_allocation_info))
    } else {
        Err(result)
    }
}
#[doc = "Function similar to vmaCreateAliasingBuffer() but for images."]
pub unsafe fn create_aliasing_image(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    p_image_create_info: &vk::ImageCreateInfo,
) -> Result<vk::Image, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingImage(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            p_image_create_info: *const vk::ImageCreateInfo,
            p_image: *mut vk::Image,
        ) -> vk::Result;
    }
    let mut p_image = ::std::mem::zeroed();
    let result = vmaCreateAliasingImage(allocator, allocation, p_image_create_info, &mut p_image);
    if result == vk::Result::SUCCESS {
        Ok(p_image)
    } else {
        Err(result)
    }
}
#[doc = "Function similar to vmaCreateAliasingBuffer2() but for images."]
pub unsafe fn create_aliasing_image_2(
    allocator: crate::vma::Allocator,
    allocation: crate::vma::Allocation,
    allocation_local_offset: vk::DeviceSize,
    p_image_create_info: &vk::ImageCreateInfo,
) -> Result<vk::Image, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingImage2(
            allocator: crate::vma::Allocator,
            allocation: crate::vma::Allocation,
            allocation_local_offset: vk::DeviceSize,
            p_image_create_info: *const vk::ImageCreateInfo,
            p_image: *mut vk::Image,
        ) -> vk::Result;
    }
    let mut p_image = ::std::mem::zeroed();
    let result = vmaCreateAliasingImage2(
        allocator,
        allocation,
        allocation_local_offset,
        p_image_create_info,
        &mut p_image,
    );
    if result == vk::Result::SUCCESS {
        Ok(p_image)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Destroys Vulkan image and frees allocated memory.\n\nThis is just a convenience function equivalent to:\n\n\\code\nvkDestroyImage(device, image, allocationCallbacks);\nvmaFreeMemory(allocator, allocation);\n\\endcode\n\nIt is safe to pass null as image and/or allocation."]
pub unsafe fn destroy_image(
    allocator: crate::vma::Allocator,
    image: vk::Image,
    allocation: crate::vma::Allocation,
) {
    extern "C" {
        fn vmaDestroyImage(
            allocator: crate::vma::Allocator,
            image: vk::Image,
            allocation: crate::vma::Allocation,
        );
    }
    vmaDestroyImage(allocator, image, allocation);
}
#[doc = "\\brief Creates new #VmaVirtualBlock object.\n\n\\param pCreateInfo Parameters for creation.\n\\param[out] pVirtualBlock Returned virtual block object or `VMA_NULL` if creation failed."]
pub unsafe fn create_virtual_block(
    p_create_info: &crate::vma::VirtualBlockCreateInfo,
) -> Result<crate::vma::VirtualBlock, vk::Result> {
    extern "C" {
        fn vmaCreateVirtualBlock(
            p_create_info: *const crate::vma::VirtualBlockCreateInfo,
            p_virtual_block: *mut crate::vma::VirtualBlock,
        ) -> vk::Result;
    }
    let mut p_virtual_block = ::std::mem::zeroed();
    let result = vmaCreateVirtualBlock(p_create_info, &mut p_virtual_block);
    if result == vk::Result::SUCCESS {
        Ok(p_virtual_block)
    } else {
        Err(result)
    }
}
#[doc = "\\brief Destroys #VmaVirtualBlock object.\n\nPlease note that you should consciously handle virtual allocations that could remain unfreed in the block.\nYou should either free them individually using vmaVirtualFree() or call vmaClearVirtualBlock()\nif you are sure this is what you want. If you do neither, an assert is called.\n\nIf you keep pointers to some additional metadata associated with your virtual allocations in their `pUserData`,\ndon't forget to free them."]
pub unsafe fn destroy_virtual_block(virtual_block: crate::vma::VirtualBlock) {
    extern "C" {
        fn vmaDestroyVirtualBlock(virtual_block: crate::vma::VirtualBlock);
    }
    vmaDestroyVirtualBlock(virtual_block);
}
#[doc = "\\brief Returns true of the #VmaVirtualBlock is empty - contains 0 virtual allocations and has all its space available for new allocations."]
pub unsafe fn is_virtual_block_empty(
    virtual_block: crate::vma::VirtualBlock,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaIsVirtualBlockEmpty(virtual_block: crate::vma::VirtualBlock) -> vk::Result;
    }
    let result = vmaIsVirtualBlockEmpty(virtual_block);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Returns information about a specific virtual allocation within a virtual block, like its size and `pUserData` pointer."]
pub unsafe fn get_virtual_allocation_info(
    virtual_block: crate::vma::VirtualBlock,
    allocation: crate::vma::VirtualAllocation,
) -> crate::vma::VirtualAllocationInfo {
    extern "C" {
        fn vmaGetVirtualAllocationInfo(
            virtual_block: crate::vma::VirtualBlock,
            allocation: crate::vma::VirtualAllocation,
            p_virtual_alloc_info: *mut crate::vma::VirtualAllocationInfo,
        );
    }
    let mut p_virtual_alloc_info = ::std::mem::zeroed();
    vmaGetVirtualAllocationInfo(virtual_block, allocation, &mut p_virtual_alloc_info);
    p_virtual_alloc_info
}
#[doc = "\\brief Allocates new virtual allocation inside given #VmaVirtualBlock.\n\nIf the allocation fails due to not enough free space available, `VK_ERROR_OUT_OF_DEVICE_MEMORY` is returned\n(despite the function doesn't ever allocate actual GPU memory).\n`pAllocation` is then set to `VK_NULL_HANDLE` and `pOffset`, if not null, it set to `UINT64_MAX`.\n\n\\param virtualBlock Virtual block\n\\param pCreateInfo Parameters for the allocation\n\\param[out] pAllocation Returned handle of the new allocation\n\\param[out] pOffset Returned offset of the new allocation. Optional, can be null."]
pub unsafe fn virtual_allocate(
    virtual_block: crate::vma::VirtualBlock,
    p_create_info: &crate::vma::VirtualAllocationCreateInfo,
) -> Result<(crate::vma::VirtualAllocation, vk::DeviceSize), vk::Result> {
    extern "C" {
        fn vmaVirtualAllocate(
            virtual_block: crate::vma::VirtualBlock,
            p_create_info: *const crate::vma::VirtualAllocationCreateInfo,
            p_allocation: *mut crate::vma::VirtualAllocation,
            p_offset: *mut vk::DeviceSize,
        ) -> vk::Result;
    }
    let mut p_allocation = ::std::mem::zeroed();
    let mut p_offset = ::std::mem::zeroed();
    let result = vmaVirtualAllocate(
        virtual_block,
        p_create_info,
        &mut p_allocation,
        &mut p_offset,
    );
    if result == vk::Result::SUCCESS {
        Ok((p_allocation, p_offset))
    } else {
        Err(result)
    }
}
#[doc = "\\brief Frees virtual allocation inside given #VmaVirtualBlock.\n\nIt is correct to call this function with `allocation == VK_NULL_HANDLE` - it does nothing."]
pub unsafe fn virtual_free(
    virtual_block: crate::vma::VirtualBlock,
    allocation: crate::vma::VirtualAllocation,
) {
    extern "C" {
        fn vmaVirtualFree(
            virtual_block: crate::vma::VirtualBlock,
            allocation: crate::vma::VirtualAllocation,
        );
    }
    vmaVirtualFree(virtual_block, allocation);
}
#[doc = "\\brief Frees all virtual allocations inside given #VmaVirtualBlock.\n\nYou must either call this function or free each virtual allocation individually with vmaVirtualFree()\nbefore destroying a virtual block. Otherwise, an assert is called.\n\nIf you keep pointer to some additional metadata associated with your virtual allocation in its `pUserData`,\ndon't forget to free it as well."]
pub unsafe fn clear_virtual_block(virtual_block: crate::vma::VirtualBlock) {
    extern "C" {
        fn vmaClearVirtualBlock(virtual_block: crate::vma::VirtualBlock);
    }
    vmaClearVirtualBlock(virtual_block);
}
#[doc = "\\brief Changes custom pointer associated with given virtual allocation."]
pub unsafe fn set_virtual_allocation_user_data(
    virtual_block: crate::vma::VirtualBlock,
    allocation: crate::vma::VirtualAllocation,
    p_user_data: *mut ::std::ffi::c_void,
) {
    extern "C" {
        fn vmaSetVirtualAllocationUserData(
            virtual_block: crate::vma::VirtualBlock,
            allocation: crate::vma::VirtualAllocation,
            p_user_data: *mut ::std::ffi::c_void,
        );
    }
    vmaSetVirtualAllocationUserData(virtual_block, allocation, p_user_data);
}
#[doc = "\\brief Calculates and returns statistics about virtual allocations and memory usage in given #VmaVirtualBlock.\n\nThis function is fast to call. For more detailed statistics, see vmaCalculateVirtualBlockStatistics()."]
pub unsafe fn get_virtual_block_statistics(
    virtual_block: crate::vma::VirtualBlock,
) -> crate::vma::Statistics {
    extern "C" {
        fn vmaGetVirtualBlockStatistics(
            virtual_block: crate::vma::VirtualBlock,
            p_stats: *mut crate::vma::Statistics,
        );
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaGetVirtualBlockStatistics(virtual_block, &mut p_stats);
    p_stats
}
#[doc = "\\brief Calculates and returns detailed statistics about virtual allocations and memory usage in given #VmaVirtualBlock.\n\nThis function is slow to call. Use for debugging purposes.\nFor less detailed statistics, see vmaGetVirtualBlockStatistics()."]
pub unsafe fn calculate_virtual_block_statistics(
    virtual_block: crate::vma::VirtualBlock,
) -> crate::vma::DetailedStatistics {
    extern "C" {
        fn vmaCalculateVirtualBlockStatistics(
            virtual_block: crate::vma::VirtualBlock,
            p_stats: *mut crate::vma::DetailedStatistics,
        );
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaCalculateVirtualBlockStatistics(virtual_block, &mut p_stats);
    p_stats
}
#[doc = "\\brief Builds and returns a null-terminated string in JSON format with information about given #VmaVirtualBlock.\n\\param virtualBlock Virtual block.\n\\param[out] ppStatsString Returned string.\n\\param detailedMap Pass `VK_FALSE` to only obtain statistics as returned by vmaCalculateVirtualBlockStatistics(). Pass `VK_TRUE` to also obtain full list of allocations and free spaces.\n\nReturned string must be freed using vmaFreeVirtualBlockStatsString()."]
pub unsafe fn build_virtual_block_stats_string(
    virtual_block: crate::vma::VirtualBlock,
    detailed_map: vk::Bool32,
) -> *mut ::std::ffi::c_char {
    extern "C" {
        fn vmaBuildVirtualBlockStatsString(
            virtual_block: crate::vma::VirtualBlock,
            pp_stats_string: *mut *mut ::std::ffi::c_char,
            detailed_map: vk::Bool32,
        );
    }
    let mut pp_stats_string = ::std::mem::zeroed();
    vmaBuildVirtualBlockStatsString(virtual_block, &mut pp_stats_string, detailed_map);
    pp_stats_string
}
#[doc = "Frees a string returned by vmaBuildVirtualBlockStatsString()."]
pub unsafe fn free_virtual_block_stats_string(
    virtual_block: crate::vma::VirtualBlock,
) -> ::std::ffi::c_char {
    extern "C" {
        fn vmaFreeVirtualBlockStatsString(
            virtual_block: crate::vma::VirtualBlock,
            p_stats_string: *mut ::std::ffi::c_char,
        );
    }
    let mut p_stats_string = ::std::mem::zeroed();
    vmaFreeVirtualBlockStatsString(virtual_block, &mut p_stats_string);
    p_stats_string
}
#[doc = "\\brief Builds and returns statistics as a null-terminated string in JSON format.\n\\param allocator\n\\param[out] ppStatsString Must be freed using vmaFreeStatsString() function.\n\\param detailedMap"]
pub unsafe fn build_stats_string(
    allocator: crate::vma::Allocator,
    detailed_map: vk::Bool32,
) -> *mut ::std::ffi::c_char {
    extern "C" {
        fn vmaBuildStatsString(
            allocator: crate::vma::Allocator,
            pp_stats_string: *mut *mut ::std::ffi::c_char,
            detailed_map: vk::Bool32,
        );
    }
    let mut pp_stats_string = ::std::mem::zeroed();
    vmaBuildStatsString(allocator, &mut pp_stats_string, detailed_map);
    pp_stats_string
}
pub unsafe fn free_stats_string(allocator: crate::vma::Allocator) -> ::std::ffi::c_char {
    extern "C" {
        fn vmaFreeStatsString(
            allocator: crate::vma::Allocator,
            p_stats_string: *mut ::std::ffi::c_char,
        );
    }
    let mut p_stats_string = ::std::mem::zeroed();
    vmaFreeStatsString(allocator, &mut p_stats_string);
    p_stats_string
}
