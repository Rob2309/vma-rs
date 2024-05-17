use crate::function_ptrs::*;
use ash::vk;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Set of callbacks that the library will call for `vkAllocateMemory` and `vkFreeMemory`.\n\nProvided for informative purpose, e.g. to gather statistics about number of\nallocations or total amount of memory allocated in Vulkan.\n\nUsed in VmaAllocatorCreateInfo::pDeviceMemoryCallbacks."]
pub struct DeviceMemoryCallbacks {
    #[doc = "Optional, can be null."]
    pub pfn_allocate: PFN_vmaAllocateDeviceMemoryFunction,
    #[doc = "Optional, can be null."]
    pub pfn_free: PFN_vmaFreeDeviceMemoryFunction,
    #[doc = "Optional, can be null."]
    pub p_user_data: *mut ::std::ffi::c_void,
}
impl Default for DeviceMemoryCallbacks {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DeviceMemoryCallbacks {
    pub fn allocate(mut self, pfn_allocate: PFN_vmaAllocateDeviceMemoryFunction) -> Self {
        self.pfn_allocate = pfn_allocate;
        self
    }
    pub fn free(mut self, pfn_free: PFN_vmaFreeDeviceMemoryFunction) -> Self {
        self.pfn_free = pfn_free;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.p_user_data = p_user_data;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Pointers to some Vulkan functions - a subset used by the library.\n\nUsed in VmaAllocatorCreateInfo::pVulkanFunctions."]
pub struct VulkanFunctions {
    #[doc = "Required when using VMA_DYNAMIC_VULKAN_FUNCTIONS."]
    pub vk_get_instance_proc_addr: Option<vk::PFN_vkGetInstanceProcAddr>,
    #[doc = "Required when using VMA_DYNAMIC_VULKAN_FUNCTIONS."]
    pub vk_get_device_proc_addr: Option<vk::PFN_vkGetDeviceProcAddr>,
    pub vk_get_physical_device_properties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    pub vk_get_physical_device_memory_properties:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub vk_allocate_memory: Option<vk::PFN_vkAllocateMemory>,
    pub vk_free_memory: Option<vk::PFN_vkFreeMemory>,
    pub vk_map_memory: Option<vk::PFN_vkMapMemory>,
    pub vk_unmap_memory: Option<vk::PFN_vkUnmapMemory>,
    pub vk_flush_mapped_memory_ranges: Option<vk::PFN_vkFlushMappedMemoryRanges>,
    pub vk_invalidate_mapped_memory_ranges: Option<vk::PFN_vkInvalidateMappedMemoryRanges>,
    pub vk_bind_buffer_memory: Option<vk::PFN_vkBindBufferMemory>,
    pub vk_bind_image_memory: Option<vk::PFN_vkBindImageMemory>,
    pub vk_get_buffer_memory_requirements: Option<vk::PFN_vkGetBufferMemoryRequirements>,
    pub vk_get_image_memory_requirements: Option<vk::PFN_vkGetImageMemoryRequirements>,
    pub vk_create_buffer: Option<vk::PFN_vkCreateBuffer>,
    pub vk_destroy_buffer: Option<vk::PFN_vkDestroyBuffer>,
    pub vk_create_image: Option<vk::PFN_vkCreateImage>,
    pub vk_destroy_image: Option<vk::PFN_vkDestroyImage>,
    pub vk_cmd_copy_buffer: Option<vk::PFN_vkCmdCopyBuffer>,
    #[doc = "Fetch \"vkGetBufferMemoryRequirements2\" on Vulkan >= 1.1, fetch \"vkGetBufferMemoryRequirements2KHR\" when using VK_KHR_dedicated_allocation extension."]
    pub vk_get_buffer_memory_requirements_2_khr: Option<vk::PFN_vkGetBufferMemoryRequirements2>,
    #[doc = "Fetch \"vkGetImageMemoryRequirements2\" on Vulkan >= 1.1, fetch \"vkGetImageMemoryRequirements2KHR\" when using VK_KHR_dedicated_allocation extension."]
    pub vk_get_image_memory_requirements_2_khr: Option<vk::PFN_vkGetImageMemoryRequirements2>,
    #[doc = "Fetch \"vkBindBufferMemory2\" on Vulkan >= 1.1, fetch \"vkBindBufferMemory2KHR\" when using VK_KHR_bind_memory2 extension."]
    pub vk_bind_buffer_memory_2_khr: Option<vk::PFN_vkBindBufferMemory2>,
    #[doc = "Fetch \"vkBindImageMemory2\" on Vulkan >= 1.1, fetch \"vkBindImageMemory2KHR\" when using VK_KHR_bind_memory2 extension."]
    pub vk_bind_image_memory_2_khr: Option<vk::PFN_vkBindImageMemory2>,
    #[doc = "Fetch from \"vkGetPhysicalDeviceMemoryProperties2\" on Vulkan >= 1.1, but you can also fetch it from \"vkGetPhysicalDeviceMemoryProperties2KHR\" if you enabled extension VK_KHR_get_physical_device_properties2."]
    pub vk_get_physical_device_memory_properties_2_khr:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties2>,
    #[doc = "Fetch from \"vkGetDeviceBufferMemoryRequirements\" on Vulkan >= 1.3, but you can also fetch it from \"vkGetDeviceBufferMemoryRequirementsKHR\" if you enabled extension VK_KHR_maintenance4."]
    pub vk_get_device_buffer_memory_requirements:
        Option<vk::PFN_vkGetDeviceBufferMemoryRequirements>,
    #[doc = "Fetch from \"vkGetDeviceImageMemoryRequirements\" on Vulkan >= 1.3, but you can also fetch it from \"vkGetDeviceImageMemoryRequirementsKHR\" if you enabled extension VK_KHR_maintenance4."]
    pub vk_get_device_image_memory_requirements: Option<vk::PFN_vkGetDeviceImageMemoryRequirements>,
}
impl Default for VulkanFunctions {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl VulkanFunctions {
    pub fn get_instance_proc_addr(
        mut self,
        vk_get_instance_proc_addr: Option<vk::PFN_vkGetInstanceProcAddr>,
    ) -> Self {
        self.vk_get_instance_proc_addr = vk_get_instance_proc_addr;
        self
    }
    pub fn get_device_proc_addr(
        mut self,
        vk_get_device_proc_addr: Option<vk::PFN_vkGetDeviceProcAddr>,
    ) -> Self {
        self.vk_get_device_proc_addr = vk_get_device_proc_addr;
        self
    }
    pub fn get_physical_device_properties(
        mut self,
        vk_get_physical_device_properties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    ) -> Self {
        self.vk_get_physical_device_properties = vk_get_physical_device_properties;
        self
    }
    pub fn get_physical_device_memory_properties(
        mut self,
        vk_get_physical_device_memory_properties: Option<
            vk::PFN_vkGetPhysicalDeviceMemoryProperties,
        >,
    ) -> Self {
        self.vk_get_physical_device_memory_properties = vk_get_physical_device_memory_properties;
        self
    }
    pub fn allocate_memory(mut self, vk_allocate_memory: Option<vk::PFN_vkAllocateMemory>) -> Self {
        self.vk_allocate_memory = vk_allocate_memory;
        self
    }
    pub fn free_memory(mut self, vk_free_memory: Option<vk::PFN_vkFreeMemory>) -> Self {
        self.vk_free_memory = vk_free_memory;
        self
    }
    pub fn map_memory(mut self, vk_map_memory: Option<vk::PFN_vkMapMemory>) -> Self {
        self.vk_map_memory = vk_map_memory;
        self
    }
    pub fn unmap_memory(mut self, vk_unmap_memory: Option<vk::PFN_vkUnmapMemory>) -> Self {
        self.vk_unmap_memory = vk_unmap_memory;
        self
    }
    pub fn flush_mapped_memory_ranges(
        mut self,
        vk_flush_mapped_memory_ranges: Option<vk::PFN_vkFlushMappedMemoryRanges>,
    ) -> Self {
        self.vk_flush_mapped_memory_ranges = vk_flush_mapped_memory_ranges;
        self
    }
    pub fn invalidate_mapped_memory_ranges(
        mut self,
        vk_invalidate_mapped_memory_ranges: Option<vk::PFN_vkInvalidateMappedMemoryRanges>,
    ) -> Self {
        self.vk_invalidate_mapped_memory_ranges = vk_invalidate_mapped_memory_ranges;
        self
    }
    pub fn bind_buffer_memory(
        mut self,
        vk_bind_buffer_memory: Option<vk::PFN_vkBindBufferMemory>,
    ) -> Self {
        self.vk_bind_buffer_memory = vk_bind_buffer_memory;
        self
    }
    pub fn bind_image_memory(
        mut self,
        vk_bind_image_memory: Option<vk::PFN_vkBindImageMemory>,
    ) -> Self {
        self.vk_bind_image_memory = vk_bind_image_memory;
        self
    }
    pub fn get_buffer_memory_requirements(
        mut self,
        vk_get_buffer_memory_requirements: Option<vk::PFN_vkGetBufferMemoryRequirements>,
    ) -> Self {
        self.vk_get_buffer_memory_requirements = vk_get_buffer_memory_requirements;
        self
    }
    pub fn get_image_memory_requirements(
        mut self,
        vk_get_image_memory_requirements: Option<vk::PFN_vkGetImageMemoryRequirements>,
    ) -> Self {
        self.vk_get_image_memory_requirements = vk_get_image_memory_requirements;
        self
    }
    pub fn create_buffer(mut self, vk_create_buffer: Option<vk::PFN_vkCreateBuffer>) -> Self {
        self.vk_create_buffer = vk_create_buffer;
        self
    }
    pub fn destroy_buffer(mut self, vk_destroy_buffer: Option<vk::PFN_vkDestroyBuffer>) -> Self {
        self.vk_destroy_buffer = vk_destroy_buffer;
        self
    }
    pub fn create_image(mut self, vk_create_image: Option<vk::PFN_vkCreateImage>) -> Self {
        self.vk_create_image = vk_create_image;
        self
    }
    pub fn destroy_image(mut self, vk_destroy_image: Option<vk::PFN_vkDestroyImage>) -> Self {
        self.vk_destroy_image = vk_destroy_image;
        self
    }
    pub fn cmd_copy_buffer(mut self, vk_cmd_copy_buffer: Option<vk::PFN_vkCmdCopyBuffer>) -> Self {
        self.vk_cmd_copy_buffer = vk_cmd_copy_buffer;
        self
    }
    pub fn get_buffer_memory_requirements_2_khr(
        mut self,
        vk_get_buffer_memory_requirements_2_khr: Option<vk::PFN_vkGetBufferMemoryRequirements2>,
    ) -> Self {
        self.vk_get_buffer_memory_requirements_2_khr = vk_get_buffer_memory_requirements_2_khr;
        self
    }
    pub fn get_image_memory_requirements_2_khr(
        mut self,
        vk_get_image_memory_requirements_2_khr: Option<vk::PFN_vkGetImageMemoryRequirements2>,
    ) -> Self {
        self.vk_get_image_memory_requirements_2_khr = vk_get_image_memory_requirements_2_khr;
        self
    }
    pub fn bind_buffer_memory_2_khr(
        mut self,
        vk_bind_buffer_memory_2_khr: Option<vk::PFN_vkBindBufferMemory2>,
    ) -> Self {
        self.vk_bind_buffer_memory_2_khr = vk_bind_buffer_memory_2_khr;
        self
    }
    pub fn bind_image_memory_2_khr(
        mut self,
        vk_bind_image_memory_2_khr: Option<vk::PFN_vkBindImageMemory2>,
    ) -> Self {
        self.vk_bind_image_memory_2_khr = vk_bind_image_memory_2_khr;
        self
    }
    pub fn get_physical_device_memory_properties_2_khr(
        mut self,
        vk_get_physical_device_memory_properties_2_khr: Option<
            vk::PFN_vkGetPhysicalDeviceMemoryProperties2,
        >,
    ) -> Self {
        self.vk_get_physical_device_memory_properties_2_khr =
            vk_get_physical_device_memory_properties_2_khr;
        self
    }
    pub fn get_device_buffer_memory_requirements(
        mut self,
        vk_get_device_buffer_memory_requirements: Option<
            vk::PFN_vkGetDeviceBufferMemoryRequirements,
        >,
    ) -> Self {
        self.vk_get_device_buffer_memory_requirements = vk_get_device_buffer_memory_requirements;
        self
    }
    pub fn get_device_image_memory_requirements(
        mut self,
        vk_get_device_image_memory_requirements: Option<vk::PFN_vkGetDeviceImageMemoryRequirements>,
    ) -> Self {
        self.vk_get_device_image_memory_requirements = vk_get_device_image_memory_requirements;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Description of a Allocator to be created."]
pub struct AllocatorCreateInfo<'a> {
    #[doc = "Flags for created allocator. Use #VmaAllocatorCreateFlagBits enum."]
    pub flags: crate::vma::AllocatorCreateFlags,
    #[doc = "Vulkan physical device.\n /** It must be valid throughout whole lifetime of created allocator."]
    pub physical_device: vk::PhysicalDevice,
    #[doc = "Vulkan device.\n /** It must be valid throughout whole lifetime of created allocator."]
    pub device: vk::Device,
    #[doc = "Preferred size of a single `VkDeviceMemory` block to be allocated from large heaps > 1 GiB. Optional.\n /** Set to 0 to use default, which is currently 256 MiB."]
    pub preferred_large_heap_block_size: vk::DeviceSize,
    #[doc = "Custom CPU memory allocation callbacks. Optional.\n /** Optional, can be null. When specified, will also be used for all CPU-side memory allocations."]
    pub p_allocation_callbacks: *const vk::AllocationCallbacks<'a>,
    #[doc = "Informative callbacks for `vkAllocateMemory`, `vkFreeMemory`. Optional.\n /** Optional, can be null."]
    pub p_device_memory_callbacks: *const crate::vma::DeviceMemoryCallbacks,
    #[doc = "\\brief Either null or a pointer to an array of limits on maximum number of bytes that can be allocated out of particular Vulkan memory heap.\n\n If not NULL, it must be a pointer to an array of\n `VkPhysicalDeviceMemoryProperties::memoryHeapCount` elements, defining limit on\n maximum number of bytes that can be allocated out of particular Vulkan memory\n heap.\n\n Any of the elements may be equal to `VK_WHOLE_SIZE`, which means no limit on that\n heap. This is also the default in case of `pHeapSizeLimit` = NULL.\n\n If there is a limit defined for a heap:\n\n - If user tries to allocate more memory from that heap using this allocator,\n   the allocation fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY`.\n - If the limit is smaller than heap size reported in `VkMemoryHeap::size`, the\n   value of this limit will be reported instead when using vmaGetMemoryProperties().\n\n Warning! Using this feature may not be equivalent to installing a GPU with\n smaller amount of memory, because graphics driver doesn't necessary fail new\n allocations with `VK_ERROR_OUT_OF_DEVICE_MEMORY` result when memory capacity is\n exceeded. It may return success and just silently migrate some device memory\n blocks to system RAM. This driver behavior can also be controlled using\n VK_AMD_memory_overallocation_behavior extension."]
    pub p_heap_size_limit: *const vk::DeviceSize,
    #[doc = "\\brief Pointers to Vulkan functions. Can be null.\n\n For details see [Pointers to Vulkan functions](@ref config_Vulkan_functions)."]
    pub p_vulkan_functions: *const crate::vma::VulkanFunctions,
    #[doc = "\\brief Handle to Vulkan instance object.\n\n Starting from version 3.0.0 this member is no longer optional, it must be set!"]
    pub instance: vk::Instance,
    #[doc = "\\brief Optional. Vulkan version that the application uses.\n\n It must be a value in the format as created by macro `VK_MAKE_VERSION` or a constant like: `VK_API_VERSION_1_1`, `VK_API_VERSION_1_0`.\n The patch version number specified is ignored. Only the major and minor versions are considered.\n Only versions 1.0, 1.1, 1.2, 1.3 are supported by the current implementation.\n Leaving it initialized to zero is equivalent to `VK_API_VERSION_1_0`.\n It must match the Vulkan version used by the application and supported on the selected physical device,\n so it must be no higher than `VkApplicationInfo::apiVersion` passed to `vkCreateInstance`\n and no higher than `VkPhysicalDeviceProperties::apiVersion` found on the physical device used."]
    pub vulkan_api_version: u32,
    #[doc = "\\brief Either null or a pointer to an array of external memory handle types for each Vulkan memory type.\n\n If not NULL, it must be a pointer to an array of `VkPhysicalDeviceMemoryProperties::memoryTypeCount`\n elements, defining external memory handle types of particular Vulkan memory type,\n to be passed using `VkExportMemoryAllocateInfoKHR`.\n\n Any of the elements may be equal to 0, which means not to use `VkExportMemoryAllocateInfoKHR` on this memory type.\n This is also the default in case of `pTypeExternalMemoryHandleTypes` = NULL."]
    pub p_type_external_memory_handle_types: *const vk::ExternalMemoryHandleTypeFlagsKHR,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> Default for AllocatorCreateInfo<'a> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<'a> AllocatorCreateInfo<'a> {
    pub fn flags(mut self, flags: crate::vma::AllocatorCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn physical_device(mut self, physical_device: vk::PhysicalDevice) -> Self {
        self.physical_device = physical_device;
        self
    }
    pub fn device(mut self, device: vk::Device) -> Self {
        self.device = device;
        self
    }
    pub fn preferred_large_heap_block_size(
        mut self,
        preferred_large_heap_block_size: vk::DeviceSize,
    ) -> Self {
        self.preferred_large_heap_block_size = preferred_large_heap_block_size;
        self
    }
    pub fn allocation_callbacks(
        mut self,
        p_allocation_callbacks: &'a vk::AllocationCallbacks<'a>,
    ) -> Self {
        self.p_allocation_callbacks = p_allocation_callbacks;
        self
    }
    pub fn device_memory_callbacks(
        mut self,
        p_device_memory_callbacks: &'a crate::vma::DeviceMemoryCallbacks,
    ) -> Self {
        self.p_device_memory_callbacks = p_device_memory_callbacks;
        self
    }
    pub fn heap_size_limit(mut self, p_heap_size_limit: &'a [vk::DeviceSize]) -> Self {
        self.p_heap_size_limit = p_heap_size_limit.as_ptr();
        self
    }
    pub fn vulkan_functions(mut self, p_vulkan_functions: &'a crate::vma::VulkanFunctions) -> Self {
        self.p_vulkan_functions = p_vulkan_functions;
        self
    }
    pub fn instance(mut self, instance: vk::Instance) -> Self {
        self.instance = instance;
        self
    }
    pub fn vulkan_api_version(mut self, vulkan_api_version: u32) -> Self {
        self.vulkan_api_version = vulkan_api_version;
        self
    }
    pub fn type_external_memory_handle_types(
        mut self,
        p_type_external_memory_handle_types: &'a [vk::ExternalMemoryHandleTypeFlagsKHR],
    ) -> Self {
        self.p_type_external_memory_handle_types = p_type_external_memory_handle_types.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Information about existing #VmaAllocator object."]
pub struct AllocatorInfo {
    #[doc = "\\brief Handle to Vulkan instance object.\n\n This is the same value as has been passed through VmaAllocatorCreateInfo::instance."]
    pub instance: vk::Instance,
    #[doc = "\\brief Handle to Vulkan physical device object.\n\n This is the same value as has been passed through VmaAllocatorCreateInfo::physicalDevice."]
    pub physical_device: vk::PhysicalDevice,
    #[doc = "\\brief Handle to Vulkan device object.\n\n This is the same value as has been passed through VmaAllocatorCreateInfo::device."]
    pub device: vk::Device,
}
impl Default for AllocatorInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AllocatorInfo {
    pub fn instance(mut self, instance: vk::Instance) -> Self {
        self.instance = instance;
        self
    }
    pub fn physical_device(mut self, physical_device: vk::PhysicalDevice) -> Self {
        self.physical_device = physical_device;
        self
    }
    pub fn device(mut self, device: vk::Device) -> Self {
        self.device = device;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Calculated statistics of memory usage e.g. in a specific memory type, heap, custom pool, or total.\n\nThese are fast to calculate.\nSee functions: vmaGetHeapBudgets(), vmaGetPoolStatistics()."]
pub struct Statistics {
    #[doc = "\\brief Number of `VkDeviceMemory` objects - Vulkan memory blocks allocated."]
    pub block_count: u32,
    #[doc = "\\brief Number of #VmaAllocation objects allocated.\n\n Dedicated allocations have their own blocks, so each one adds 1 to `allocationCount` as well as `blockCount`."]
    pub allocation_count: u32,
    #[doc = "\\brief Number of bytes allocated in `VkDeviceMemory` blocks.\n\n \\note To avoid confusion, please be aware that what Vulkan calls an \"allocation\" - a whole `VkDeviceMemory` object\n (e.g. as in `VkPhysicalDeviceLimits::maxMemoryAllocationCount`) is called a \"block\" in VMA, while VMA calls\n \"allocation\" a #VmaAllocation object that represents a memory region sub-allocated from such block, usually for a single buffer or image."]
    pub block_bytes: vk::DeviceSize,
    #[doc = "\\brief Total number of bytes occupied by all #VmaAllocation objects.\n\n Always less or equal than `blockBytes`.\n Difference `(blockBytes - allocationBytes)` is the amount of memory allocated from Vulkan\n but unused by any #VmaAllocation."]
    pub allocation_bytes: vk::DeviceSize,
}
impl Default for Statistics {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Statistics {
    pub fn block_count(mut self, block_count: u32) -> Self {
        self.block_count = block_count;
        self
    }
    pub fn allocation_count(mut self, allocation_count: u32) -> Self {
        self.allocation_count = allocation_count;
        self
    }
    pub fn block_bytes(mut self, block_bytes: vk::DeviceSize) -> Self {
        self.block_bytes = block_bytes;
        self
    }
    pub fn allocation_bytes(mut self, allocation_bytes: vk::DeviceSize) -> Self {
        self.allocation_bytes = allocation_bytes;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief More detailed statistics than #VmaStatistics.\n\nThese are slower to calculate. Use for debugging purposes.\nSee functions: vmaCalculateStatistics(), vmaCalculatePoolStatistics().\n\nPrevious version of the statistics API provided averages, but they have been removed\nbecause they can be easily calculated as:\n\n\\code\nVkDeviceSize allocationSizeAvg = detailedStats.statistics.allocationBytes / detailedStats.statistics.allocationCount;\nVkDeviceSize unusedBytes = detailedStats.statistics.blockBytes - detailedStats.statistics.allocationBytes;\nVkDeviceSize unusedRangeSizeAvg = unusedBytes / detailedStats.unusedRangeCount;\n\\endcode"]
pub struct DetailedStatistics {
    #[doc = "Basic statistics."]
    pub statistics: crate::vma::Statistics,
    #[doc = "Number of free ranges of memory between allocations."]
    pub unused_range_count: u32,
    #[doc = "Smallest allocation size. `VK_WHOLE_SIZE` if there are 0 allocations."]
    pub allocation_size_min: vk::DeviceSize,
    #[doc = "Largest allocation size. 0 if there are 0 allocations."]
    pub allocation_size_max: vk::DeviceSize,
    #[doc = "Smallest empty range size. `VK_WHOLE_SIZE` if there are 0 empty ranges."]
    pub unused_range_size_min: vk::DeviceSize,
    #[doc = "Largest empty range size. 0 if there are 0 empty ranges."]
    pub unused_range_size_max: vk::DeviceSize,
}
impl Default for DetailedStatistics {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DetailedStatistics {
    pub fn statistics(mut self, statistics: crate::vma::Statistics) -> Self {
        self.statistics = statistics;
        self
    }
    pub fn unused_range_count(mut self, unused_range_count: u32) -> Self {
        self.unused_range_count = unused_range_count;
        self
    }
    pub fn allocation_size_min(mut self, allocation_size_min: vk::DeviceSize) -> Self {
        self.allocation_size_min = allocation_size_min;
        self
    }
    pub fn allocation_size_max(mut self, allocation_size_max: vk::DeviceSize) -> Self {
        self.allocation_size_max = allocation_size_max;
        self
    }
    pub fn unused_range_size_min(mut self, unused_range_size_min: vk::DeviceSize) -> Self {
        self.unused_range_size_min = unused_range_size_min;
        self
    }
    pub fn unused_range_size_max(mut self, unused_range_size_max: vk::DeviceSize) -> Self {
        self.unused_range_size_max = unused_range_size_max;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief  General statistics from current state of the Allocator -\ntotal memory usage across all memory heaps and types.\n\nThese are slower to calculate. Use for debugging purposes.\nSee function vmaCalculateStatistics()."]
pub struct TotalStatistics {
    pub memory_type: [crate::vma::DetailedStatistics; 32usize],
    pub memory_heap: [crate::vma::DetailedStatistics; 16usize],
    pub total: crate::vma::DetailedStatistics,
}
impl Default for TotalStatistics {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl TotalStatistics {
    pub fn memory_type(mut self, memory_type: [crate::vma::DetailedStatistics; 32usize]) -> Self {
        self.memory_type = memory_type;
        self
    }
    pub fn memory_heap(mut self, memory_heap: [crate::vma::DetailedStatistics; 16usize]) -> Self {
        self.memory_heap = memory_heap;
        self
    }
    pub fn total(mut self, total: crate::vma::DetailedStatistics) -> Self {
        self.total = total;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Statistics of current memory usage and available budget for a specific memory heap.\n\nThese are fast to calculate.\nSee function vmaGetHeapBudgets()."]
pub struct Budget {
    #[doc = "\\brief Statistics fetched from the library."]
    pub statistics: crate::vma::Statistics,
    #[doc = "\\brief Estimated current memory usage of the program, in bytes.\n\n Fetched from system using VK_EXT_memory_budget extension if enabled.\n\n It might be different than `statistics.blockBytes` (usually higher) due to additional implicit objects\n also occupying the memory, like swapchain, pipelines, descriptor heaps, command buffers, or\n `VkDeviceMemory` blocks allocated outside of this library, if any."]
    pub usage: vk::DeviceSize,
    #[doc = "\\brief Estimated amount of memory available to the program, in bytes.\n\n Fetched from system using VK_EXT_memory_budget extension if enabled.\n\n It might be different (most probably smaller) than `VkMemoryHeap::size[heapIndex]` due to factors\n external to the program, decided by the operating system.\n Difference `budget - usage` is the amount of additional memory that can probably\n be allocated without problems. Exceeding the budget may result in various problems."]
    pub budget: vk::DeviceSize,
}
impl Default for Budget {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Budget {
    pub fn statistics(mut self, statistics: crate::vma::Statistics) -> Self {
        self.statistics = statistics;
        self
    }
    pub fn usage(mut self, usage: vk::DeviceSize) -> Self {
        self.usage = usage;
        self
    }
    pub fn budget(mut self, budget: vk::DeviceSize) -> Self {
        self.budget = budget;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Parameters of new #VmaAllocation.\n\nTo be used with functions like vmaCreateBuffer(), vmaCreateImage(), and many others."]
pub struct AllocationCreateInfo {
    #[doc = "Use #VmaAllocationCreateFlagBits enum."]
    pub flags: crate::vma::AllocationCreateFlags,
    #[doc = "\\brief Intended usage of memory.\n\n You can leave #VMA_MEMORY_USAGE_UNKNOWN if you specify memory requirements in other way. \\n\n If `pool` is not null, this member is ignored."]
    pub usage: crate::vma::MemoryUsage,
    #[doc = "\\brief Flags that must be set in a Memory Type chosen for an allocation.\n\n Leave 0 if you specify memory requirements in other way. \\n\n If `pool` is not null, this member is ignored."]
    pub required_flags: vk::MemoryPropertyFlags,
    #[doc = "\\brief Flags that preferably should be set in a memory type chosen for an allocation.\n\n Set to 0 if no additional flags are preferred. \\n\n If `pool` is not null, this member is ignored."]
    pub preferred_flags: vk::MemoryPropertyFlags,
    #[doc = "\\brief Bitmask containing one bit set for every memory type acceptable for this allocation.\n\n Value 0 is equivalent to `UINT32_MAX` - it means any memory type is accepted if\n it meets other requirements specified by this structure, with no further\n restrictions on memory type index. \\n\n If `pool` is not null, this member is ignored."]
    pub memory_type_bits: u32,
    #[doc = "\\brief Pool that this allocation should be created in.\n\n Leave `VK_NULL_HANDLE` to allocate from default pool. If not null, members:\n `usage`, `requiredFlags`, `preferredFlags`, `memoryTypeBits` are ignored."]
    pub pool: crate::vma::Pool,
    #[doc = "\\brief Custom general-purpose pointer that will be stored in #VmaAllocation, can be read as VmaAllocationInfo::pUserData and changed using vmaSetAllocationUserData().\n\n If #VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT is used, it must be either\n null or pointer to a null-terminated string. The string will be then copied to\n internal buffer, so it doesn't need to be valid after allocation call."]
    pub p_user_data: *mut ::std::ffi::c_void,
    #[doc = "\\brief A floating-point value between 0 and 1, indicating the priority of the allocation relative to other memory allocations.\n\n It is used only when #VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT flag was used during creation of the #VmaAllocator object\n and this allocation ends up as dedicated or is explicitly forced as dedicated using #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.\n Otherwise, it has the priority of a memory block where it is placed and this variable is ignored."]
    pub priority: f32,
}
impl Default for AllocationCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AllocationCreateInfo {
    pub fn flags(mut self, flags: crate::vma::AllocationCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn usage(mut self, usage: crate::vma::MemoryUsage) -> Self {
        self.usage = usage;
        self
    }
    pub fn required_flags(mut self, required_flags: vk::MemoryPropertyFlags) -> Self {
        self.required_flags = required_flags;
        self
    }
    pub fn preferred_flags(mut self, preferred_flags: vk::MemoryPropertyFlags) -> Self {
        self.preferred_flags = preferred_flags;
        self
    }
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.memory_type_bits = memory_type_bits;
        self
    }
    pub fn pool(mut self, pool: crate::vma::Pool) -> Self {
        self.pool = pool;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.p_user_data = p_user_data;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.priority = priority;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Describes parameter of created #VmaPool."]
pub struct PoolCreateInfo<'a> {
    #[doc = "\\brief Vulkan memory type index to allocate this pool from."]
    pub memory_type_index: u32,
    #[doc = "\\brief Use combination of #VmaPoolCreateFlagBits."]
    pub flags: crate::vma::PoolCreateFlags,
    #[doc = "\\brief Size of a single `VkDeviceMemory` block to be allocated as part of this pool, in bytes. Optional.\n\n Specify nonzero to set explicit, constant size of memory blocks used by this\n pool.\n\n Leave 0 to use default and let the library manage block sizes automatically.\n Sizes of particular blocks may vary.\n In this case, the pool will also support dedicated allocations."]
    pub block_size: vk::DeviceSize,
    #[doc = "\\brief Minimum number of blocks to be always allocated in this pool, even if they stay empty.\n\n Set to 0 to have no preallocated blocks and allow the pool be completely empty."]
    pub min_block_count: usize,
    #[doc = "\\brief Maximum number of blocks that can be allocated in this pool. Optional.\n\n Set to 0 to use default, which is `SIZE_MAX`, which means no limit.\n\n Set to same value as VmaPoolCreateInfo::minBlockCount to have fixed amount of memory allocated\n throughout whole lifetime of this pool."]
    pub max_block_count: usize,
    #[doc = "\\brief A floating-point value between 0 and 1, indicating the priority of the allocations in this pool relative to other memory allocations.\n\n It is used only when #VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT flag was used during creation of the #VmaAllocator object.\n Otherwise, this variable is ignored."]
    pub priority: f32,
    #[doc = "\\brief Additional minimum alignment to be used for all allocations created from this pool. Can be 0.\n\n Leave 0 (default) not to impose any additional alignment. If not 0, it must be a power of two.\n It can be useful in cases where alignment returned by Vulkan by functions like `vkGetBufferMemoryRequirements` is not enough,\n e.g. when doing interop with OpenGL."]
    pub min_allocation_alignment: vk::DeviceSize,
    #[doc = "\\brief Additional `pNext` chain to be attached to `VkMemoryAllocateInfo` used for every allocation made by this pool. Optional.\n\n Optional, can be null. If not null, it must point to a `pNext` chain of structures that can be attached to `VkMemoryAllocateInfo`.\n It can be useful for special needs such as adding `VkExportMemoryAllocateInfoKHR`.\n Structures pointed by this member must remain alive and unchanged for the whole lifetime of the custom pool.\n\n Please note that some structures, e.g. `VkMemoryPriorityAllocateInfoEXT`, `VkMemoryDedicatedAllocateInfoKHR`,\n can be attached automatically by this library when using other, more convenient of its features."]
    pub p_memory_allocate_next: *mut ::std::ffi::c_void,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> Default for PoolCreateInfo<'a> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<'a> PoolCreateInfo<'a> {
    pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
        self.memory_type_index = memory_type_index;
        self
    }
    pub fn flags(mut self, flags: crate::vma::PoolCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn block_size(mut self, block_size: vk::DeviceSize) -> Self {
        self.block_size = block_size;
        self
    }
    pub fn min_block_count(mut self, min_block_count: usize) -> Self {
        self.min_block_count = min_block_count;
        self
    }
    pub fn max_block_count(mut self, max_block_count: usize) -> Self {
        self.max_block_count = max_block_count;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.priority = priority;
        self
    }
    pub fn min_allocation_alignment(mut self, min_allocation_alignment: vk::DeviceSize) -> Self {
        self.min_allocation_alignment = min_allocation_alignment;
        self
    }
    pub fn memory_allocate_next(
        mut self,
        p_memory_allocate_next: Option<&'a mut impl vk::ExtendsMemoryAllocateInfo>,
    ) -> Self {
        self.p_memory_allocate_next =
            p_memory_allocate_next.map_or(::std::ptr::null_mut(), |p| p as *mut _ as *mut _);
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of #VmaAllocation objects, that can be retrieved using function vmaGetAllocationInfo().\n\nThere is also an extended version of this structure that carries additional parameters: #VmaAllocationInfo2."]
pub struct AllocationInfo<'a> {
    #[doc = "\\brief Memory type index that this allocation was allocated from.\n\n It never changes."]
    pub memory_type: u32,
    #[doc = "\\brief Handle to Vulkan memory object.\n\n Same memory object can be shared by multiple allocations.\n\n It can change after the allocation is moved during \\ref defragmentation."]
    pub device_memory: vk::DeviceMemory,
    #[doc = "\\brief Offset in `VkDeviceMemory` object to the beginning of this allocation, in bytes. `(deviceMemory, offset)` pair is unique to this allocation.\n\n You usually don't need to use this offset. If you create a buffer or an image together with the allocation using e.g. function\n vmaCreateBuffer(), vmaCreateImage(), functions that operate on these resources refer to the beginning of the buffer or image,\n not entire device memory block. Functions like vmaMapMemory(), vmaBindBufferMemory() also refer to the beginning of the allocation\n and apply this offset automatically.\n\n It can change after the allocation is moved during \\ref defragmentation."]
    pub offset: vk::DeviceSize,
    #[doc = "\\brief Size of this allocation, in bytes.\n\n It never changes.\n\n \\note Allocation size returned in this variable may be greater than the size\n requested for the resource e.g. as `VkBufferCreateInfo::size`. Whole size of the\n allocation is accessible for operations on memory e.g. using a pointer after\n mapping with vmaMapMemory(), but operations on the resource e.g. using\n `vkCmdCopyBuffer` must be limited to the size of the resource."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Pointer to the beginning of this allocation as mapped data.\n\n If the allocation hasn't been mapped using vmaMapMemory() and hasn't been\n created with #VMA_ALLOCATION_CREATE_MAPPED_BIT flag, this value is null.\n\n It can change after call to vmaMapMemory(), vmaUnmapMemory().\n It can also change after the allocation is moved during \\ref defragmentation."]
    pub p_mapped_data: *mut ::std::ffi::c_void,
    #[doc = "\\brief Custom general-purpose pointer that was passed as VmaAllocationCreateInfo::pUserData or set using vmaSetAllocationUserData().\n\n It can change after call to vmaSetAllocationUserData() for this allocation."]
    pub p_user_data: *mut ::std::ffi::c_void,
    #[doc = "\\brief Custom allocation name that was set with vmaSetAllocationName().\n\n It can change after call to vmaSetAllocationName() for this allocation.\n\n Another way to set custom name is to pass it in VmaAllocationCreateInfo::pUserData with\n additional flag #VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT set [DEPRECATED]."]
    pub p_name: *const ::std::ffi::c_char,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> Default for AllocationInfo<'a> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<'a> AllocationInfo<'a> {
    pub fn get_name(&self) -> Option<&::std::ffi::CStr> {
        if !self.p_name.is_null() {
            Some(unsafe { ::std::ffi::CStr::from_ptr(self.p_name) })
        } else {
            None
        }
    }
}
impl<'a> AllocationInfo<'a> {
    pub fn memory_type(mut self, memory_type: u32) -> Self {
        self.memory_type = memory_type;
        self
    }
    pub fn device_memory(mut self, device_memory: vk::DeviceMemory) -> Self {
        self.device_memory = device_memory;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.size = size;
        self
    }
    pub fn mapped_data(mut self, p_mapped_data: *mut ::std::ffi::c_void) -> Self {
        self.p_mapped_data = p_mapped_data;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.p_user_data = p_user_data;
        self
    }
    pub fn name(mut self, p_name: Option<&'a ::std::ffi::CStr>) -> Self {
        self.p_name = p_name.map_or(::std::ptr::null(), |s| s.as_ptr());
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Extended parameters of a #VmaAllocation object that can be retrieved using function vmaGetAllocationInfo2()."]
pub struct AllocationInfo2<'a> {
    #[doc = "\\brief Basic parameters of the allocation.\n \n If you need only these, you can use function vmaGetAllocationInfo() and structure #VmaAllocationInfo instead."]
    pub allocation_info: crate::vma::AllocationInfo<'a>,
    #[doc = "\\brief Size of the `VkDeviceMemory` block that the allocation belongs to.\n \n In case of an allocation with dedicated memory, it will be equal to `allocationInfo.size`."]
    pub block_size: vk::DeviceSize,
    #[doc = "\\brief `VK_TRUE` if the allocation has dedicated memory, `VK_FALSE` if it was placed as part of a larger memory block.\n \n When `VK_TRUE`, it also means `VkMemoryDedicatedAllocateInfo` was used when creating the allocation\n (if VK_KHR_dedicated_allocation extension or Vulkan version >= 1.1 is enabled)."]
    pub dedicated_memory: vk::Bool32,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> Default for AllocationInfo2<'a> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<'a> AllocationInfo2<'a> {
    pub fn allocation_info(mut self, allocation_info: crate::vma::AllocationInfo<'a>) -> Self {
        self.allocation_info = allocation_info;
        self
    }
    pub fn block_size(mut self, block_size: vk::DeviceSize) -> Self {
        self.block_size = block_size;
        self
    }
    pub fn dedicated_memory(mut self, dedicated_memory: vk::Bool32) -> Self {
        self.dedicated_memory = dedicated_memory;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Parameters for defragmentation.\n\nTo be used with function vmaBeginDefragmentation()."]
pub struct DefragmentationInfo {
    #[doc = "\\brief Use combination of #VmaDefragmentationFlagBits."]
    pub flags: crate::vma::DefragmentationFlags,
    #[doc = "\\brief Custom pool to be defragmented.\n\n If null then default pools will undergo defragmentation process."]
    pub pool: crate::vma::Pool,
    #[doc = "\\brief Maximum numbers of bytes that can be copied during single pass, while moving allocations to different places.\n\n `0` means no limit."]
    pub max_bytes_per_pass: vk::DeviceSize,
    #[doc = "\\brief Maximum number of allocations that can be moved during single pass to a different place.\n\n `0` means no limit."]
    pub max_allocations_per_pass: u32,
    #[doc = "\\brief Optional custom callback for stopping vmaBeginDefragmentation().\n\n Have to return true for breaking current defragmentation pass."]
    pub pfn_break_callback: PFN_vmaCheckDefragmentationBreakFunction,
    #[doc = "\\brief Optional data to pass to custom callback for stopping pass of defragmentation."]
    pub p_break_callback_user_data: *mut ::std::ffi::c_void,
}
impl Default for DefragmentationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DefragmentationInfo {
    pub fn flags(mut self, flags: crate::vma::DefragmentationFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn pool(mut self, pool: crate::vma::Pool) -> Self {
        self.pool = pool;
        self
    }
    pub fn max_bytes_per_pass(mut self, max_bytes_per_pass: vk::DeviceSize) -> Self {
        self.max_bytes_per_pass = max_bytes_per_pass;
        self
    }
    pub fn max_allocations_per_pass(mut self, max_allocations_per_pass: u32) -> Self {
        self.max_allocations_per_pass = max_allocations_per_pass;
        self
    }
    pub fn break_callback(
        mut self,
        pfn_break_callback: PFN_vmaCheckDefragmentationBreakFunction,
    ) -> Self {
        self.pfn_break_callback = pfn_break_callback;
        self
    }
    pub fn break_callback_user_data(
        mut self,
        p_break_callback_user_data: *mut ::std::ffi::c_void,
    ) -> Self {
        self.p_break_callback_user_data = p_break_callback_user_data;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Single move of an allocation to be done for defragmentation."]
pub struct DefragmentationMove {
    #[doc = "Operation to be performed on the allocation by vmaEndDefragmentationPass(). Default value is #VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY. You can modify it."]
    pub operation: crate::vma::DefragmentationMoveOperation,
    #[doc = "Allocation that should be moved."]
    pub src_allocation: crate::vma::Allocation,
    #[doc = "\\brief Temporary allocation pointing to destination memory that will replace `srcAllocation`.\n\n \\warning Do not store this allocation in your data structures! It exists only temporarily, for the duration of the defragmentation pass,\n to be used for binding new buffer/image to the destination memory using e.g. vmaBindBufferMemory().\n vmaEndDefragmentationPass() will destroy it and make `srcAllocation` point to this memory."]
    pub dst_tmp_allocation: crate::vma::Allocation,
}
impl Default for DefragmentationMove {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DefragmentationMove {
    pub fn operation(mut self, operation: crate::vma::DefragmentationMoveOperation) -> Self {
        self.operation = operation;
        self
    }
    pub fn src_allocation(mut self, src_allocation: crate::vma::Allocation) -> Self {
        self.src_allocation = src_allocation;
        self
    }
    pub fn dst_tmp_allocation(mut self, dst_tmp_allocation: crate::vma::Allocation) -> Self {
        self.dst_tmp_allocation = dst_tmp_allocation;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Parameters for incremental defragmentation steps.\n\nTo be used with function vmaBeginDefragmentationPass()."]
pub struct DefragmentationPassMoveInfo<'a> {
    #[doc = "Number of elements in the `pMoves` array."]
    pub move_count: u32,
    #[doc = "\\brief Array of moves to be performed by the user in the current defragmentation pass.\n\n Pointer to an array of `moveCount` elements, owned by VMA, created in vmaBeginDefragmentationPass(), destroyed in vmaEndDefragmentationPass().\n\n For each element, you should:\n\n 1. Create a new buffer/image in the place pointed by VmaDefragmentationMove::dstMemory + VmaDefragmentationMove::dstOffset.\n 2. Copy data from the VmaDefragmentationMove::srcAllocation e.g. using `vkCmdCopyBuffer`, `vkCmdCopyImage`.\n 3. Make sure these commands finished executing on the GPU.\n 4. Destroy the old buffer/image.\n\n Only then you can finish defragmentation pass by calling vmaEndDefragmentationPass().\n After this call, the allocation will point to the new place in memory.\n\n Alternatively, if you cannot move specific allocation, you can set VmaDefragmentationMove::operation to #VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE.\n\n Alternatively, if you decide you want to completely remove the allocation:\n\n 1. Destroy its buffer/image.\n 2. Set VmaDefragmentationMove::operation to #VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY.\n\n Then, after vmaEndDefragmentationPass() the allocation will be freed."]
    pub p_moves: *mut crate::vma::DefragmentationMove,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> Default for DefragmentationPassMoveInfo<'a> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<'a> DefragmentationPassMoveInfo<'a> {
    pub fn get_moves(&self) -> &[crate::vma::DefragmentationMove] {
        unsafe { ::std::slice::from_raw_parts(self.p_moves, self.move_count as _) }
    }
    pub fn get_moves_mut(&mut self) -> &mut [crate::vma::DefragmentationMove] {
        unsafe { ::std::slice::from_raw_parts_mut(self.p_moves, self.move_count as _) }
    }
}
impl<'a> DefragmentationPassMoveInfo<'a> {
    pub fn moves(mut self, p_moves: &'a mut [crate::vma::DefragmentationMove]) -> Self {
        self.p_moves = p_moves.as_mut_ptr();
        self.move_count = p_moves.len() as _;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Statistics returned for defragmentation process in function vmaEndDefragmentation()."]
pub struct DefragmentationStats {
    #[doc = "Total number of bytes that have been copied while moving allocations to different places."]
    pub bytes_moved: vk::DeviceSize,
    #[doc = "Total number of bytes that have been released to the system by freeing empty `VkDeviceMemory` objects."]
    pub bytes_freed: vk::DeviceSize,
    #[doc = "Number of allocations that have been moved to different places."]
    pub allocations_moved: u32,
    #[doc = "Number of empty `VkDeviceMemory` objects that have been released to the system."]
    pub device_memory_blocks_freed: u32,
}
impl Default for DefragmentationStats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DefragmentationStats {
    pub fn bytes_moved(mut self, bytes_moved: vk::DeviceSize) -> Self {
        self.bytes_moved = bytes_moved;
        self
    }
    pub fn bytes_freed(mut self, bytes_freed: vk::DeviceSize) -> Self {
        self.bytes_freed = bytes_freed;
        self
    }
    pub fn allocations_moved(mut self, allocations_moved: u32) -> Self {
        self.allocations_moved = allocations_moved;
        self
    }
    pub fn device_memory_blocks_freed(mut self, device_memory_blocks_freed: u32) -> Self {
        self.device_memory_blocks_freed = device_memory_blocks_freed;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of created #VmaVirtualBlock object to be passed to vmaCreateVirtualBlock()."]
pub struct VirtualBlockCreateInfo<'a> {
    #[doc = "\\brief Total size of the virtual block.\n\n Sizes can be expressed in bytes or any units you want as long as you are consistent in using them.\n For example, if you allocate from some array of structures, 1 can mean single instance of entire structure."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Use combination of #VmaVirtualBlockCreateFlagBits."]
    pub flags: crate::vma::VirtualBlockCreateFlags,
    #[doc = "\\brief Custom CPU memory allocation callbacks. Optional.\n\n Optional, can be null. When specified, they will be used for all CPU-side memory allocations."]
    pub p_allocation_callbacks: *const vk::AllocationCallbacks<'a>,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> Default for VirtualBlockCreateInfo<'a> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<'a> VirtualBlockCreateInfo<'a> {
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.size = size;
        self
    }
    pub fn flags(mut self, flags: crate::vma::VirtualBlockCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn allocation_callbacks(
        mut self,
        p_allocation_callbacks: &'a vk::AllocationCallbacks<'a>,
    ) -> Self {
        self.p_allocation_callbacks = p_allocation_callbacks;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of created virtual allocation to be passed to vmaVirtualAllocate()."]
pub struct VirtualAllocationCreateInfo {
    #[doc = "\\brief Size of the allocation.\n\n Cannot be zero."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Required alignment of the allocation. Optional.\n\n Must be power of two. Special value 0 has the same meaning as 1 - means no special alignment is required, so allocation can start at any offset."]
    pub alignment: vk::DeviceSize,
    #[doc = "\\brief Use combination of #VmaVirtualAllocationCreateFlagBits."]
    pub flags: crate::vma::VirtualAllocationCreateFlags,
    #[doc = "\\brief Custom pointer to be associated with the allocation. Optional.\n\n It can be any value and can be used for user-defined purposes. It can be fetched or changed later."]
    pub p_user_data: *mut ::std::ffi::c_void,
}
impl Default for VirtualAllocationCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl VirtualAllocationCreateInfo {
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.size = size;
        self
    }
    pub fn alignment(mut self, alignment: vk::DeviceSize) -> Self {
        self.alignment = alignment;
        self
    }
    pub fn flags(mut self, flags: crate::vma::VirtualAllocationCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.p_user_data = p_user_data;
        self
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of an existing virtual allocation, returned by vmaGetVirtualAllocationInfo()."]
pub struct VirtualAllocationInfo {
    #[doc = "\\brief Offset of the allocation.\n\n Offset at which the allocation was made."]
    pub offset: vk::DeviceSize,
    #[doc = "\\brief Size of the allocation.\n\n Same value as passed in VmaVirtualAllocationCreateInfo::size."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Custom pointer associated with the allocation.\n\n Same value as passed in VmaVirtualAllocationCreateInfo::pUserData or to vmaSetVirtualAllocationUserData()."]
    pub p_user_data: *mut ::std::ffi::c_void,
}
impl Default for VirtualAllocationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl VirtualAllocationInfo {
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.size = size;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.p_user_data = p_user_data;
        self
    }
}
