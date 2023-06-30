use crate::function_ptrs::*;
use crate::handles::*;
use ash::vk;
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags for created #VmaAllocator."]
pub struct AllocatorCreateFlags(u32);
impl AllocatorCreateFlags {
    #[doc = "\\brief Allocator and all objects created from it will not be synchronized internally, so you must guarantee they are used from only one thread at a time or synchronized externally by you.\n\n    Using this flag may increase performance because internal mutexes are not used."]
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1);
    #[doc = "\\brief Enables usage of VK_KHR_dedicated_allocation extension.\n\n    The flag works only if VmaAllocatorCreateInfo::vulkanApiVersion `== VK_API_VERSION_1_0`.\n    When it is `VK_API_VERSION_1_1`, the flag is ignored because the extension has been promoted to Vulkan 1.1.\n\n    Using this extension will automatically allocate dedicated blocks of memory for\n    some buffers and images instead of suballocating place for them out of bigger\n    memory blocks (as if you explicitly used #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT\n    flag) when it is recommended by the driver. It may improve performance on some\n    GPUs.\n\n    You may set this flag only if you found out that following device extensions are\n    supported, you enabled them while creating Vulkan device passed as\n    VmaAllocatorCreateInfo::device, and you want them to be used internally by this\n    library:\n\n    - VK_KHR_get_memory_requirements2 (device extension)\n    - VK_KHR_dedicated_allocation (device extension)\n\n    When this flag is set, you can experience following warnings reported by Vulkan\n    validation layer. You can ignore them.\n\n    > vkBindBufferMemory(): Binding memory to buffer 0x2d but vkGetBufferMemoryRequirements() has not been called on that buffer."]
    pub const KHR_DEDICATED_ALLOCATION: Self = Self(2);
    #[doc = "Enables usage of VK_KHR_bind_memory2 extension.\n\n    The flag works only if VmaAllocatorCreateInfo::vulkanApiVersion `== VK_API_VERSION_1_0`.\n    When it is `VK_API_VERSION_1_1`, the flag is ignored because the extension has been promoted to Vulkan 1.1.\n\n    You may set this flag only if you found out that this device extension is supported,\n    you enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,\n    and you want it to be used internally by this library.\n\n    The extension provides functions `vkBindBufferMemory2KHR` and `vkBindImageMemory2KHR`,\n    which allow to pass a chain of `pNext` structures while binding.\n    This flag is required if you use `pNext` parameter in vmaBindBufferMemory2() or vmaBindImageMemory2()."]
    pub const KHR_BIND_MEMORY2: Self = Self(4);
    #[doc = "Enables usage of VK_EXT_memory_budget extension.\n\n    You may set this flag only if you found out that this device extension is supported,\n    you enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,\n    and you want it to be used internally by this library, along with another instance extension\n    VK_KHR_get_physical_device_properties2, which is required by it (or Vulkan 1.1, where this extension is promoted).\n\n    The extension provides query for current memory usage and budget, which will probably\n    be more accurate than an estimation used by the library otherwise."]
    pub const EXT_MEMORY_BUDGET: Self = Self(8);
    #[doc = "Enables usage of VK_AMD_device_coherent_memory extension.\n\n    You may set this flag only if you:\n\n    - found out that this device extension is supported and enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,\n    - checked that `VkPhysicalDeviceCoherentMemoryFeaturesAMD::deviceCoherentMemory` is true and set it while creating the Vulkan device,\n    - want it to be used internally by this library.\n\n    The extension and accompanying device feature provide access to memory types with\n    `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` and `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD` flags.\n    They are useful mostly for writing breadcrumb markers - a common method for debugging GPU crash/hang/TDR.\n\n    When the extension is not enabled, such memory types are still enumerated, but their usage is illegal.\n    To protect from this error, if you don't create the allocator with this flag, it will refuse to allocate any memory or create a custom pool in such memory type,\n    returning `VK_ERROR_FEATURE_NOT_PRESENT`."]
    pub const AMD_DEVICE_COHERENT_MEMORY: Self = Self(16);
    #[doc = "Enables usage of \"buffer device address\" feature, which allows you to use function\n    `vkGetBufferDeviceAddress*` to get raw GPU pointer to a buffer and pass it for usage inside a shader.\n\n    You may set this flag only if you:\n\n    1. (For Vulkan version < 1.2) Found as available and enabled device extension\n    VK_KHR_buffer_device_address.\n    This extension is promoted to core Vulkan 1.2.\n    2. Found as available and enabled device feature `VkPhysicalDeviceBufferDeviceAddressFeatures::bufferDeviceAddress`.\n\n    When this flag is set, you can create buffers with `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` using VMA.\n    The library automatically adds `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` to\n    allocated memory blocks wherever it might be needed.\n\n    For more information, see documentation chapter \\ref enabling_buffer_device_address."]
    pub const BUFFER_DEVICE_ADDRESS: Self = Self(32);
    #[doc = "Enables usage of VK_EXT_memory_priority extension in the library.\n\n    You may set this flag only if you found available and enabled this device extension,\n    along with `VkPhysicalDeviceMemoryPriorityFeaturesEXT::memoryPriority == VK_TRUE`,\n    while creating Vulkan device passed as VmaAllocatorCreateInfo::device.\n\n    When this flag is used, VmaAllocationCreateInfo::priority and VmaPoolCreateInfo::priority\n    are used to set priorities of allocated Vulkan memory. Without it, these variables are ignored.\n\n    A priority must be a floating-point value between 0 and 1, indicating the priority of the allocation relative to other memory allocations.\n    Larger values are higher priority. The granularity of the priorities is implementation-dependent.\n    It is automatically passed to every call to `vkAllocateMemory` done by the library using structure `VkMemoryPriorityAllocateInfoEXT`.\n    The value to be used for default priority is 0.5.\n    For more details, see the documentation of the VK_EXT_memory_priority extension."]
    pub const EXT_MEMORY_PRIORITY: Self = Self(64);
}
impl ::std::ops::BitOr<AllocatorCreateFlags> for AllocatorCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: AllocatorCreateFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign<AllocatorCreateFlags> for AllocatorCreateFlags {
    fn bitor_assign(&mut self, rhs: AllocatorCreateFlags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<AllocatorCreateFlags> for AllocatorCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: AllocatorCreateFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign<AllocatorCreateFlags> for AllocatorCreateFlags {
    fn bitand_assign(&mut self, rhs: AllocatorCreateFlags) {
        self.0 &= rhs.0;
    }
}
impl ::std::ops::BitXor<AllocatorCreateFlags> for AllocatorCreateFlags {
    type Output = Self;
    fn bitxor(self, rhs: AllocatorCreateFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign<AllocatorCreateFlags> for AllocatorCreateFlags {
    fn bitxor_assign(&mut self, rhs: AllocatorCreateFlags) {
        self.0 ^= rhs.0;
    }
}
impl ::std::ops::Not for AllocatorCreateFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "\\brief Intended usage of the allocated memory."]
pub struct MemoryUsage(u32);
impl MemoryUsage {
    #[doc = "No intended memory usage specified.\n    Use other members of VmaAllocationCreateInfo to specify your requirements."]
    pub const UNKNOWN: Self = Self(0);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n    Prefers `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`."]
    pub const GPU_ONLY: Self = Self(1);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n    Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` and `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`."]
    pub const CPU_ONLY: Self = Self(2);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n    Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`, prefers `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`."]
    pub const CPU_TO_GPU: Self = Self(3);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n    Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`, prefers `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`."]
    pub const GPU_TO_CPU: Self = Self(4);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n    Prefers not `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`."]
    pub const CPU_COPY: Self = Self(5);
    #[doc = "Lazily allocated GPU memory having `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`.\n    Exists mostly on mobile platforms. Using it on desktop PC or other GPUs with no such memory type present will fail the allocation.\n\n    Usage: Memory for transient attachment images (color attachments, depth attachments etc.), created with `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`.\n\n    Allocations with this usage are always created as dedicated - it implies #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT."]
    pub const GPU_LAZILY_ALLOCATED: Self = Self(6);
    #[doc = "Selects best memory type automatically.\n    This flag is recommended for most common use cases.\n\n    When using this flag, if you want to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT),\n    you must pass one of the flags: #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT\n    in VmaAllocationCreateInfo::flags.\n\n    It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.\n    vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()\n    and not with generic memory allocation functions."]
    pub const AUTO: Self = Self(7);
    #[doc = "Selects best memory type automatically with preference for GPU (device) memory.\n\n    When using this flag, if you want to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT),\n    you must pass one of the flags: #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT\n    in VmaAllocationCreateInfo::flags.\n\n    It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.\n    vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()\n    and not with generic memory allocation functions."]
    pub const AUTO_PREFER_DEVICE: Self = Self(8);
    #[doc = "Selects best memory type automatically with preference for CPU (host) memory.\n\n    When using this flag, if you want to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT),\n    you must pass one of the flags: #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT\n    in VmaAllocationCreateInfo::flags.\n\n    It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.\n    vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()\n    and not with generic memory allocation functions."]
    pub const AUTO_PREFER_HOST: Self = Self(9);
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaAllocationCreateInfo::flags."]
pub struct AllocationCreateFlags(u32);
impl AllocationCreateFlags {
    #[doc = "\\brief Set this flag if the allocation should have its own memory block.\n\n    Use it for special, big resources, like fullscreen images used as attachments."]
    pub const DEDICATED_MEMORY: Self = Self(1);
    #[doc = "\\brief Set this flag to only try to allocate from existing `VkDeviceMemory` blocks and never create new such block.\n\n    If new allocation cannot be placed in any of the existing blocks, allocation\n    fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY` error.\n\n    You should not use #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT and\n    #VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT at the same time. It makes no sense."]
    pub const NEVER_ALLOCATE: Self = Self(2);
    #[doc = "\\brief Set this flag to use a memory that will be persistently mapped and retrieve pointer to it.\n\n    Pointer to mapped memory will be returned through VmaAllocationInfo::pMappedData.\n\n    It is valid to use this flag for allocation made from memory type that is not\n    `HOST_VISIBLE`. This flag is then ignored and memory is not mapped. This is\n    useful if you need an allocation that is efficient to use on GPU\n    (`DEVICE_LOCAL`) and still want to map it directly if possible on platforms that\n    support it (e.g. Intel GPU)."]
    pub const MAPPED: Self = Self(4);
    #[doc = "\\deprecated Preserved for backward compatibility. Consider using vmaSetAllocationName() instead.\n\n    Set this flag to treat VmaAllocationCreateInfo::pUserData as pointer to a\n    null-terminated string. Instead of copying pointer value, a local copy of the\n    string is made and stored in allocation's `pName`. The string is automatically\n    freed together with the allocation. It is also used in vmaBuildStatsString()."]
    pub const USER_DATA_COPY_STRING: Self = Self(32);
    #[doc = "Allocation will be created from upper stack in a double stack pool.\n\n    This flag is only allowed for custom pools created with #VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT flag."]
    pub const UPPER_ADDRESS: Self = Self(64);
    #[doc = "Create both buffer/image and allocation, but don't bind them together.\n    It is useful when you want to bind yourself to do some more advanced binding, e.g. using some extensions.\n    The flag is meaningful only with functions that bind by default: vmaCreateBuffer(), vmaCreateImage().\n    Otherwise it is ignored.\n\n    If you want to make sure the new buffer/image is not tied to the new memory allocation\n    through `VkMemoryDedicatedAllocateInfoKHR` structure in case the allocation ends up in its own memory block,\n    use also flag #VMA_ALLOCATION_CREATE_CAN_ALIAS_BIT."]
    pub const DONT_BIND: Self = Self(128);
    #[doc = "Create allocation only if additional device memory required for it, if any, won't exceed\n    memory budget. Otherwise return `VK_ERROR_OUT_OF_DEVICE_MEMORY`."]
    pub const WITHIN_BUDGET: Self = Self(256);
    #[doc = "\\brief Set this flag if the allocated memory will have aliasing resources.\n\n    Usage of this flag prevents supplying `VkMemoryDedicatedAllocateInfoKHR` when #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT is specified.\n    Otherwise created dedicated memory will not be suitable for aliasing resources, resulting in Vulkan Validation Layer errors."]
    pub const CAN_ALIAS: Self = Self(512);
    #[doc = "Requests possibility to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT).\n\n    - If you use #VMA_MEMORY_USAGE_AUTO or other `VMA_MEMORY_USAGE_AUTO*` value,\n      you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.\n    - If you use other value of #VmaMemoryUsage, this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.\n      This includes allocations created in \\ref custom_memory_pools.\n\n    Declares that mapped memory will only be written sequentially, e.g. using `memcpy()` or a loop writing number-by-number,\n    never read or accessed randomly, so a memory type can be selected that is uncached and write-combined.\n\n    \\warning Violating this declaration may work correctly, but will likely be very slow.\n    Watch out for implicit reads introduced by doing e.g. `pMappedData[i] += x;`\n    Better prepare your data in a local variable and `memcpy()` it to the mapped pointer all at once."]
    pub const HOST_ACCESS_SEQUENTIAL_WRITE: Self = Self(1024);
    #[doc = "Requests possibility to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT).\n\n    - If you use #VMA_MEMORY_USAGE_AUTO or other `VMA_MEMORY_USAGE_AUTO*` value,\n      you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.\n    - If you use other value of #VmaMemoryUsage, this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.\n      This includes allocations created in \\ref custom_memory_pools.\n\n    Declares that mapped memory can be read, written, and accessed in random order,\n    so a `HOST_CACHED` memory type is required."]
    pub const HOST_ACCESS_RANDOM: Self = Self(2048);
    #[doc = "Together with #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT,\n    it says that despite request for host access, a not-`HOST_VISIBLE` memory type can be selected\n    if it may improve performance.\n\n    By using this flag, you declare that you will check if the allocation ended up in a `HOST_VISIBLE` memory type\n    (e.g. using vmaGetAllocationMemoryProperties()) and if not, you will create some \"staging\" buffer and\n    issue an explicit transfer to write/read your data.\n    To prepare for this possibility, don't forget to add appropriate flags like\n    `VK_BUFFER_USAGE_TRANSFER_DST_BIT`, `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` to the parameters of created buffer or image."]
    pub const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD: Self = Self(4096);
    #[doc = "Allocation strategy that chooses smallest possible free range for the allocation\n    to minimize memory usage and fragmentation, possibly at the expense of allocation time."]
    pub const STRATEGY_MIN_MEMORY: Self = Self(65536);
    #[doc = "Allocation strategy that chooses first suitable free range for the allocation -\n    not necessarily in terms of the smallest offset but the one that is easiest and fastest to find\n    to minimize allocation time, possibly at the expense of allocation quality."]
    pub const STRATEGY_MIN_TIME: Self = Self(131072);
    #[doc = "Allocation strategy that chooses always the lowest offset in available space.\n    This is not the most efficient strategy but achieves highly packed data.\n    Used internally by defragmentation, not recommended in typical usage."]
    pub const STRATEGY_MIN_OFFSET: Self = Self(262144);
    #[doc = "Alias to #VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT."]
    pub const STRATEGY_BEST_FIT: Self = Self(65536);
    #[doc = "Alias to #VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT."]
    pub const STRATEGY_FIRST_FIT: Self = Self(131072);
    #[doc = "A bit mask to extract only `STRATEGY` bits from entire set of flags."]
    pub const STRATEGY_MASK: Self = Self(458752);
}
impl ::std::ops::BitOr<AllocationCreateFlags> for AllocationCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: AllocationCreateFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign<AllocationCreateFlags> for AllocationCreateFlags {
    fn bitor_assign(&mut self, rhs: AllocationCreateFlags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<AllocationCreateFlags> for AllocationCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: AllocationCreateFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign<AllocationCreateFlags> for AllocationCreateFlags {
    fn bitand_assign(&mut self, rhs: AllocationCreateFlags) {
        self.0 &= rhs.0;
    }
}
impl ::std::ops::BitXor<AllocationCreateFlags> for AllocationCreateFlags {
    type Output = Self;
    fn bitxor(self, rhs: AllocationCreateFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign<AllocationCreateFlags> for AllocationCreateFlags {
    fn bitxor_assign(&mut self, rhs: AllocationCreateFlags) {
        self.0 ^= rhs.0;
    }
}
impl ::std::ops::Not for AllocationCreateFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaPoolCreateInfo::flags."]
pub struct PoolCreateFlags(u32);
impl PoolCreateFlags {
    #[doc = "\\brief Use this flag if you always allocate only buffers and linear images or only optimal images out of this pool and so Buffer-Image Granularity can be ignored.\n\n    This is an optional optimization flag.\n\n    If you always allocate using vmaCreateBuffer(), vmaCreateImage(),\n    vmaAllocateMemoryForBuffer(), then you don't need to use it because allocator\n    knows exact type of your allocations so it can handle Buffer-Image Granularity\n    in the optimal way.\n\n    If you also allocate using vmaAllocateMemoryForImage() or vmaAllocateMemory(),\n    exact type of such allocations is not known, so allocator must be conservative\n    in handling Buffer-Image Granularity, which can lead to suboptimal allocation\n    (wasted memory). In that case, if you can make sure you always allocate only\n    buffers and linear images or only optimal images out of this pool, use this flag\n    to make allocator disregard Buffer-Image Granularity and so make allocations\n    faster and more optimal."]
    pub const IGNORE_BUFFER_IMAGE_GRANULARITY: Self = Self(2);
    #[doc = "\\brief Enables alternative, linear allocation algorithm in this pool.\n\n    Specify this flag to enable linear allocation algorithm, which always creates\n    new allocations after last one and doesn't reuse space from allocations freed in\n    between. It trades memory consumption for simplified algorithm and data\n    structure, which has better performance and uses less memory for metadata.\n\n    By using this flag, you can achieve behavior of free-at-once, stack,\n    ring buffer, and double stack.\n    For details, see documentation chapter \\ref linear_algorithm."]
    pub const LINEAR_ALGORITHM: Self = Self(4);
    #[doc = "Bit mask to extract only `ALGORITHM` bits from entire set of flags."]
    pub const ALGORITHM_MASK: Self = Self(4);
}
impl ::std::ops::BitOr<PoolCreateFlags> for PoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: PoolCreateFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign<PoolCreateFlags> for PoolCreateFlags {
    fn bitor_assign(&mut self, rhs: PoolCreateFlags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<PoolCreateFlags> for PoolCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: PoolCreateFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign<PoolCreateFlags> for PoolCreateFlags {
    fn bitand_assign(&mut self, rhs: PoolCreateFlags) {
        self.0 &= rhs.0;
    }
}
impl ::std::ops::BitXor<PoolCreateFlags> for PoolCreateFlags {
    type Output = Self;
    fn bitxor(self, rhs: PoolCreateFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign<PoolCreateFlags> for PoolCreateFlags {
    fn bitxor_assign(&mut self, rhs: PoolCreateFlags) {
        self.0 ^= rhs.0;
    }
}
impl ::std::ops::Not for PoolCreateFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaDefragmentationInfo::flags."]
pub struct DefragmentationFlags(u32);
impl DefragmentationFlags {
    pub const FLAG_ALGORITHM_FAST: Self = Self(1);
    pub const FLAG_ALGORITHM_BALANCED: Self = Self(2);
    pub const FLAG_ALGORITHM_FULL: Self = Self(4);
    #[doc = "\\brief Use the most roboust algorithm at the cost of time to compute and number of copies to make.\n    Only available when bufferImageGranularity is greater than 1, since it aims to reduce\n    alignment issues between different types of resources.\n    Otherwise falls back to same behavior as #VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FULL_BIT."]
    pub const FLAG_ALGORITHM_EXTENSIVE: Self = Self(8);
    #[doc = "A bit mask to extract only `ALGORITHM` bits from entire set of flags."]
    pub const FLAG_ALGORITHM_MASK: Self = Self(15);
}
impl ::std::ops::BitOr<DefragmentationFlags> for DefragmentationFlags {
    type Output = Self;
    fn bitor(self, rhs: DefragmentationFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign<DefragmentationFlags> for DefragmentationFlags {
    fn bitor_assign(&mut self, rhs: DefragmentationFlags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<DefragmentationFlags> for DefragmentationFlags {
    type Output = Self;
    fn bitand(self, rhs: DefragmentationFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign<DefragmentationFlags> for DefragmentationFlags {
    fn bitand_assign(&mut self, rhs: DefragmentationFlags) {
        self.0 &= rhs.0;
    }
}
impl ::std::ops::BitXor<DefragmentationFlags> for DefragmentationFlags {
    type Output = Self;
    fn bitxor(self, rhs: DefragmentationFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign<DefragmentationFlags> for DefragmentationFlags {
    fn bitxor_assign(&mut self, rhs: DefragmentationFlags) {
        self.0 ^= rhs.0;
    }
}
impl ::std::ops::Not for DefragmentationFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Operation performed on single defragmentation move. See structure #VmaDefragmentationMove."]
pub struct DefragmentationMoveOperation(u32);
impl DefragmentationMoveOperation {
    #[doc = "Buffer/image has been recreated at `dstTmpAllocation`, data has been copied, old buffer/image has been destroyed. `srcAllocation` should be changed to point to the new place. This is the default value set by vmaBeginDefragmentationPass()."]
    pub const COPY: Self = Self(0);
    #[doc = "Set this value if you cannot move the allocation. New place reserved at `dstTmpAllocation` will be freed. `srcAllocation` will remain unchanged."]
    pub const IGNORE: Self = Self(1);
    #[doc = "Set this value if you decide to abandon the allocation and you destroyed the buffer/image. New place reserved at `dstTmpAllocation` will be freed, along with `srcAllocation`, which will be destroyed."]
    pub const DESTROY: Self = Self(2);
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaVirtualBlockCreateInfo::flags."]
pub struct VirtualBlockCreateFlags(u32);
impl VirtualBlockCreateFlags {
    #[doc = "\\brief Enables alternative, linear allocation algorithm in this virtual block.\n\n    Specify this flag to enable linear allocation algorithm, which always creates\n    new allocations after last one and doesn't reuse space from allocations freed in\n    between. It trades memory consumption for simplified algorithm and data\n    structure, which has better performance and uses less memory for metadata.\n\n    By using this flag, you can achieve behavior of free-at-once, stack,\n    ring buffer, and double stack.\n    For details, see documentation chapter \\ref linear_algorithm."]
    pub const LINEAR_ALGORITHM: Self = Self(1);
    #[doc = "\\brief Bit mask to extract only `ALGORITHM` bits from entire set of flags."]
    pub const ALGORITHM_MASK: Self = Self(1);
}
impl ::std::ops::BitOr<VirtualBlockCreateFlags> for VirtualBlockCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: VirtualBlockCreateFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign<VirtualBlockCreateFlags> for VirtualBlockCreateFlags {
    fn bitor_assign(&mut self, rhs: VirtualBlockCreateFlags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<VirtualBlockCreateFlags> for VirtualBlockCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: VirtualBlockCreateFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign<VirtualBlockCreateFlags> for VirtualBlockCreateFlags {
    fn bitand_assign(&mut self, rhs: VirtualBlockCreateFlags) {
        self.0 &= rhs.0;
    }
}
impl ::std::ops::BitXor<VirtualBlockCreateFlags> for VirtualBlockCreateFlags {
    type Output = Self;
    fn bitxor(self, rhs: VirtualBlockCreateFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign<VirtualBlockCreateFlags> for VirtualBlockCreateFlags {
    fn bitxor_assign(&mut self, rhs: VirtualBlockCreateFlags) {
        self.0 ^= rhs.0;
    }
}
impl ::std::ops::Not for VirtualBlockCreateFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaVirtualAllocationCreateInfo::flags."]
pub struct VirtualAllocationCreateFlags(u32);
impl VirtualAllocationCreateFlags {
    #[doc = "\\brief Allocation will be created from upper stack in a double stack pool.\n\n    This flag is only allowed for virtual blocks created with #VMA_VIRTUAL_BLOCK_CREATE_LINEAR_ALGORITHM_BIT flag."]
    pub const UPPER_ADDRESS: Self = Self(64);
    #[doc = "\\brief Allocation strategy that tries to minimize memory usage."]
    pub const STRATEGY_MIN_MEMORY: Self = Self(65536);
    #[doc = "\\brief Allocation strategy that tries to minimize allocation time."]
    pub const STRATEGY_MIN_TIME: Self = Self(131072);
    #[doc = "Allocation strategy that chooses always the lowest offset in available space.\n    This is not the most efficient strategy but achieves highly packed data."]
    pub const STRATEGY_MIN_OFFSET: Self = Self(262144);
    #[doc = "\\brief A bit mask to extract only `STRATEGY` bits from entire set of flags.\n\n    These strategy flags are binary compatible with equivalent flags in #VmaAllocationCreateFlagBits."]
    pub const STRATEGY_MASK: Self = Self(458752);
}
impl ::std::ops::BitOr<VirtualAllocationCreateFlags> for VirtualAllocationCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: VirtualAllocationCreateFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign<VirtualAllocationCreateFlags> for VirtualAllocationCreateFlags {
    fn bitor_assign(&mut self, rhs: VirtualAllocationCreateFlags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<VirtualAllocationCreateFlags> for VirtualAllocationCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: VirtualAllocationCreateFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign<VirtualAllocationCreateFlags> for VirtualAllocationCreateFlags {
    fn bitand_assign(&mut self, rhs: VirtualAllocationCreateFlags) {
        self.0 &= rhs.0;
    }
}
impl ::std::ops::BitXor<VirtualAllocationCreateFlags> for VirtualAllocationCreateFlags {
    type Output = Self;
    fn bitxor(self, rhs: VirtualAllocationCreateFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign<VirtualAllocationCreateFlags> for VirtualAllocationCreateFlags {
    fn bitxor_assign(&mut self, rhs: VirtualAllocationCreateFlags) {
        self.0 ^= rhs.0;
    }
}
impl ::std::ops::Not for VirtualAllocationCreateFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
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
    pub fn builder<'a>() -> DeviceMemoryCallbacksBuilder<'a> {
        DeviceMemoryCallbacksBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DeviceMemoryCallbacksBuilder<'a> {
    inner: DeviceMemoryCallbacks,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DeviceMemoryCallbacksBuilder<'a> {
    pub fn allocate(mut self, pfn_allocate: PFN_vmaAllocateDeviceMemoryFunction) -> Self {
        self.inner.pfn_allocate = pfn_allocate;
        self
    }
    pub fn free(mut self, pfn_free: PFN_vmaFreeDeviceMemoryFunction) -> Self {
        self.inner.pfn_free = pfn_free;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> ::std::ops::Deref for DeviceMemoryCallbacksBuilder<'a> {
    type Target = DeviceMemoryCallbacks;
    fn deref(&self) -> &DeviceMemoryCallbacks {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for DeviceMemoryCallbacksBuilder<'a> {
    fn deref_mut(&mut self) -> &mut DeviceMemoryCallbacks {
        &mut self.inner
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
    pub fn builder<'a>() -> VulkanFunctionsBuilder<'a> {
        VulkanFunctionsBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VulkanFunctionsBuilder<'a> {
    inner: VulkanFunctions,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> VulkanFunctionsBuilder<'a> {
    pub fn get_instance_proc_addr(
        mut self,
        vk_get_instance_proc_addr: Option<vk::PFN_vkGetInstanceProcAddr>,
    ) -> Self {
        self.inner.vk_get_instance_proc_addr = vk_get_instance_proc_addr;
        self
    }
    pub fn get_device_proc_addr(
        mut self,
        vk_get_device_proc_addr: Option<vk::PFN_vkGetDeviceProcAddr>,
    ) -> Self {
        self.inner.vk_get_device_proc_addr = vk_get_device_proc_addr;
        self
    }
    pub fn get_physical_device_properties(
        mut self,
        vk_get_physical_device_properties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    ) -> Self {
        self.inner.vk_get_physical_device_properties = vk_get_physical_device_properties;
        self
    }
    pub fn get_physical_device_memory_properties(
        mut self,
        vk_get_physical_device_memory_properties: Option<
            vk::PFN_vkGetPhysicalDeviceMemoryProperties,
        >,
    ) -> Self {
        self.inner.vk_get_physical_device_memory_properties =
            vk_get_physical_device_memory_properties;
        self
    }
    pub fn allocate_memory(mut self, vk_allocate_memory: Option<vk::PFN_vkAllocateMemory>) -> Self {
        self.inner.vk_allocate_memory = vk_allocate_memory;
        self
    }
    pub fn free_memory(mut self, vk_free_memory: Option<vk::PFN_vkFreeMemory>) -> Self {
        self.inner.vk_free_memory = vk_free_memory;
        self
    }
    pub fn map_memory(mut self, vk_map_memory: Option<vk::PFN_vkMapMemory>) -> Self {
        self.inner.vk_map_memory = vk_map_memory;
        self
    }
    pub fn unmap_memory(mut self, vk_unmap_memory: Option<vk::PFN_vkUnmapMemory>) -> Self {
        self.inner.vk_unmap_memory = vk_unmap_memory;
        self
    }
    pub fn flush_mapped_memory_ranges(
        mut self,
        vk_flush_mapped_memory_ranges: Option<vk::PFN_vkFlushMappedMemoryRanges>,
    ) -> Self {
        self.inner.vk_flush_mapped_memory_ranges = vk_flush_mapped_memory_ranges;
        self
    }
    pub fn invalidate_mapped_memory_ranges(
        mut self,
        vk_invalidate_mapped_memory_ranges: Option<vk::PFN_vkInvalidateMappedMemoryRanges>,
    ) -> Self {
        self.inner.vk_invalidate_mapped_memory_ranges = vk_invalidate_mapped_memory_ranges;
        self
    }
    pub fn bind_buffer_memory(
        mut self,
        vk_bind_buffer_memory: Option<vk::PFN_vkBindBufferMemory>,
    ) -> Self {
        self.inner.vk_bind_buffer_memory = vk_bind_buffer_memory;
        self
    }
    pub fn bind_image_memory(
        mut self,
        vk_bind_image_memory: Option<vk::PFN_vkBindImageMemory>,
    ) -> Self {
        self.inner.vk_bind_image_memory = vk_bind_image_memory;
        self
    }
    pub fn get_buffer_memory_requirements(
        mut self,
        vk_get_buffer_memory_requirements: Option<vk::PFN_vkGetBufferMemoryRequirements>,
    ) -> Self {
        self.inner.vk_get_buffer_memory_requirements = vk_get_buffer_memory_requirements;
        self
    }
    pub fn get_image_memory_requirements(
        mut self,
        vk_get_image_memory_requirements: Option<vk::PFN_vkGetImageMemoryRequirements>,
    ) -> Self {
        self.inner.vk_get_image_memory_requirements = vk_get_image_memory_requirements;
        self
    }
    pub fn create_buffer(mut self, vk_create_buffer: Option<vk::PFN_vkCreateBuffer>) -> Self {
        self.inner.vk_create_buffer = vk_create_buffer;
        self
    }
    pub fn destroy_buffer(mut self, vk_destroy_buffer: Option<vk::PFN_vkDestroyBuffer>) -> Self {
        self.inner.vk_destroy_buffer = vk_destroy_buffer;
        self
    }
    pub fn create_image(mut self, vk_create_image: Option<vk::PFN_vkCreateImage>) -> Self {
        self.inner.vk_create_image = vk_create_image;
        self
    }
    pub fn destroy_image(mut self, vk_destroy_image: Option<vk::PFN_vkDestroyImage>) -> Self {
        self.inner.vk_destroy_image = vk_destroy_image;
        self
    }
    pub fn cmd_copy_buffer(mut self, vk_cmd_copy_buffer: Option<vk::PFN_vkCmdCopyBuffer>) -> Self {
        self.inner.vk_cmd_copy_buffer = vk_cmd_copy_buffer;
        self
    }
    pub fn get_buffer_memory_requirements_2_khr(
        mut self,
        vk_get_buffer_memory_requirements_2_khr: Option<vk::PFN_vkGetBufferMemoryRequirements2>,
    ) -> Self {
        self.inner.vk_get_buffer_memory_requirements_2_khr =
            vk_get_buffer_memory_requirements_2_khr;
        self
    }
    pub fn get_image_memory_requirements_2_khr(
        mut self,
        vk_get_image_memory_requirements_2_khr: Option<vk::PFN_vkGetImageMemoryRequirements2>,
    ) -> Self {
        self.inner.vk_get_image_memory_requirements_2_khr = vk_get_image_memory_requirements_2_khr;
        self
    }
    pub fn bind_buffer_memory_2_khr(
        mut self,
        vk_bind_buffer_memory_2_khr: Option<vk::PFN_vkBindBufferMemory2>,
    ) -> Self {
        self.inner.vk_bind_buffer_memory_2_khr = vk_bind_buffer_memory_2_khr;
        self
    }
    pub fn bind_image_memory_2_khr(
        mut self,
        vk_bind_image_memory_2_khr: Option<vk::PFN_vkBindImageMemory2>,
    ) -> Self {
        self.inner.vk_bind_image_memory_2_khr = vk_bind_image_memory_2_khr;
        self
    }
    pub fn get_physical_device_memory_properties_2_khr(
        mut self,
        vk_get_physical_device_memory_properties_2_khr: Option<
            vk::PFN_vkGetPhysicalDeviceMemoryProperties2,
        >,
    ) -> Self {
        self.inner.vk_get_physical_device_memory_properties_2_khr =
            vk_get_physical_device_memory_properties_2_khr;
        self
    }
    pub fn get_device_buffer_memory_requirements(
        mut self,
        vk_get_device_buffer_memory_requirements: Option<
            vk::PFN_vkGetDeviceBufferMemoryRequirements,
        >,
    ) -> Self {
        self.inner.vk_get_device_buffer_memory_requirements =
            vk_get_device_buffer_memory_requirements;
        self
    }
    pub fn get_device_image_memory_requirements(
        mut self,
        vk_get_device_image_memory_requirements: Option<vk::PFN_vkGetDeviceImageMemoryRequirements>,
    ) -> Self {
        self.inner.vk_get_device_image_memory_requirements =
            vk_get_device_image_memory_requirements;
        self
    }
}
impl<'a> ::std::ops::Deref for VulkanFunctionsBuilder<'a> {
    type Target = VulkanFunctions;
    fn deref(&self) -> &VulkanFunctions {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for VulkanFunctionsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut VulkanFunctions {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Description of a Allocator to be created."]
pub struct AllocatorCreateInfo {
    #[doc = "Flags for created allocator. Use #VmaAllocatorCreateFlagBits enum."]
    pub flags: AllocatorCreateFlags,
    #[doc = "Vulkan physical device.\n    /** It must be valid throughout whole lifetime of created allocator."]
    pub physical_device: vk::PhysicalDevice,
    #[doc = "Vulkan device.\n    /** It must be valid throughout whole lifetime of created allocator."]
    pub device: vk::Device,
    #[doc = "Preferred size of a single `VkDeviceMemory` block to be allocated from large heaps > 1 GiB. Optional.\n    /** Set to 0 to use default, which is currently 256 MiB."]
    pub preferred_large_heap_block_size: vk::DeviceSize,
    #[doc = "Custom CPU memory allocation callbacks. Optional.\n    /** Optional, can be null. When specified, will also be used for all CPU-side memory allocations."]
    pub p_allocation_callbacks: *const vk::AllocationCallbacks,
    #[doc = "Informative callbacks for `vkAllocateMemory`, `vkFreeMemory`. Optional.\n    /** Optional, can be null."]
    pub p_device_memory_callbacks: *const DeviceMemoryCallbacks,
    #[doc = "\\brief Either null or a pointer to an array of limits on maximum number of bytes that can be allocated out of particular Vulkan memory heap.\n\n    If not NULL, it must be a pointer to an array of\n    `VkPhysicalDeviceMemoryProperties::memoryHeapCount` elements, defining limit on\n    maximum number of bytes that can be allocated out of particular Vulkan memory\n    heap.\n\n    Any of the elements may be equal to `VK_WHOLE_SIZE`, which means no limit on that\n    heap. This is also the default in case of `pHeapSizeLimit` = NULL.\n\n    If there is a limit defined for a heap:\n\n    - If user tries to allocate more memory from that heap using this allocator,\n      the allocation fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY`.\n    - If the limit is smaller than heap size reported in `VkMemoryHeap::size`, the\n      value of this limit will be reported instead when using vmaGetMemoryProperties().\n\n    Warning! Using this feature may not be equivalent to installing a GPU with\n    smaller amount of memory, because graphics driver doesn't necessary fail new\n    allocations with `VK_ERROR_OUT_OF_DEVICE_MEMORY` result when memory capacity is\n    exceeded. It may return success and just silently migrate some device memory\n    blocks to system RAM. This driver behavior can also be controlled using\n    VK_AMD_memory_overallocation_behavior extension."]
    pub p_heap_size_limit: *const vk::DeviceSize,
    #[doc = "\\brief Pointers to Vulkan functions. Can be null.\n\n    For details see [Pointers to Vulkan functions](@ref config_Vulkan_functions)."]
    pub p_vulkan_functions: *const VulkanFunctions,
    #[doc = "\\brief Handle to Vulkan instance object.\n\n    Starting from version 3.0.0 this member is no longer optional, it must be set!"]
    pub instance: vk::Instance,
    #[doc = "\\brief Optional. The highest version of Vulkan that the application is designed to use.\n\n    It must be a value in the format as created by macro `VK_MAKE_VERSION` or a constant like: `VK_API_VERSION_1_1`, `VK_API_VERSION_1_0`.\n    The patch version number specified is ignored. Only the major and minor versions are considered.\n    It must be less or equal (preferably equal) to value as passed to `vkCreateInstance` as `VkApplicationInfo::apiVersion`.\n    Only versions 1.0, 1.1, 1.2, 1.3 are supported by the current implementation.\n    Leaving it initialized to zero is equivalent to `VK_API_VERSION_1_0`."]
    pub vulkan_api_version: u32,
    #[doc = "\\brief Either null or a pointer to an array of external memory handle types for each Vulkan memory type.\n\n    If not NULL, it must be a pointer to an array of `VkPhysicalDeviceMemoryProperties::memoryTypeCount`\n    elements, defining external memory handle types of particular Vulkan memory type,\n    to be passed using `VkExportMemoryAllocateInfoKHR`.\n\n    Any of the elements may be equal to 0, which means not to use `VkExportMemoryAllocateInfoKHR` on this memory type.\n    This is also the default in case of `pTypeExternalMemoryHandleTypes` = NULL."]
    pub p_type_external_memory_handle_types: *const vk::ExternalMemoryHandleTypeFlagsKHR,
}
impl Default for AllocatorCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AllocatorCreateInfo {
    pub fn builder<'a>() -> AllocatorCreateInfoBuilder<'a> {
        AllocatorCreateInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AllocatorCreateInfoBuilder<'a> {
    inner: AllocatorCreateInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> AllocatorCreateInfoBuilder<'a> {
    pub fn flags(mut self, flags: AllocatorCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn physical_device(mut self, physical_device: vk::PhysicalDevice) -> Self {
        self.inner.physical_device = physical_device;
        self
    }
    pub fn device(mut self, device: vk::Device) -> Self {
        self.inner.device = device;
        self
    }
    pub fn preferred_large_heap_block_size(
        mut self,
        preferred_large_heap_block_size: vk::DeviceSize,
    ) -> Self {
        self.inner.preferred_large_heap_block_size = preferred_large_heap_block_size;
        self
    }
    pub fn allocation_callbacks(
        mut self,
        p_allocation_callbacks: &'a vk::AllocationCallbacks,
    ) -> Self {
        self.inner.p_allocation_callbacks = p_allocation_callbacks;
        self
    }
    pub fn device_memory_callbacks(
        mut self,
        p_device_memory_callbacks: &'a DeviceMemoryCallbacks,
    ) -> Self {
        self.inner.p_device_memory_callbacks = p_device_memory_callbacks;
        self
    }
    pub fn heap_size_limit(mut self, p_heap_size_limit: &'a [vk::DeviceSize]) -> Self {
        self.inner.p_heap_size_limit = p_heap_size_limit.as_ptr();
        self
    }
    pub fn vulkan_functions(mut self, p_vulkan_functions: &'a VulkanFunctions) -> Self {
        self.inner.p_vulkan_functions = p_vulkan_functions;
        self
    }
    pub fn instance(mut self, instance: vk::Instance) -> Self {
        self.inner.instance = instance;
        self
    }
    pub fn vulkan_api_version(mut self, vulkan_api_version: u32) -> Self {
        self.inner.vulkan_api_version = vulkan_api_version;
        self
    }
    pub fn type_external_memory_handle_types(
        mut self,
        p_type_external_memory_handle_types: &'a [vk::ExternalMemoryHandleTypeFlagsKHR],
    ) -> Self {
        self.inner.p_type_external_memory_handle_types =
            p_type_external_memory_handle_types.as_ptr();
        self
    }
}
impl<'a> ::std::ops::Deref for AllocatorCreateInfoBuilder<'a> {
    type Target = AllocatorCreateInfo;
    fn deref(&self) -> &AllocatorCreateInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for AllocatorCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut AllocatorCreateInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Information about existing #VmaAllocator object."]
pub struct AllocatorInfo {
    #[doc = "\\brief Handle to Vulkan instance object.\n\n    This is the same value as has been passed through VmaAllocatorCreateInfo::instance."]
    pub instance: vk::Instance,
    #[doc = "\\brief Handle to Vulkan physical device object.\n\n    This is the same value as has been passed through VmaAllocatorCreateInfo::physicalDevice."]
    pub physical_device: vk::PhysicalDevice,
    #[doc = "\\brief Handle to Vulkan device object.\n\n    This is the same value as has been passed through VmaAllocatorCreateInfo::device."]
    pub device: vk::Device,
}
impl Default for AllocatorInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AllocatorInfo {
    pub fn builder<'a>() -> AllocatorInfoBuilder<'a> {
        AllocatorInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AllocatorInfoBuilder<'a> {
    inner: AllocatorInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> AllocatorInfoBuilder<'a> {
    pub fn instance(mut self, instance: vk::Instance) -> Self {
        self.inner.instance = instance;
        self
    }
    pub fn physical_device(mut self, physical_device: vk::PhysicalDevice) -> Self {
        self.inner.physical_device = physical_device;
        self
    }
    pub fn device(mut self, device: vk::Device) -> Self {
        self.inner.device = device;
        self
    }
}
impl<'a> ::std::ops::Deref for AllocatorInfoBuilder<'a> {
    type Target = AllocatorInfo;
    fn deref(&self) -> &AllocatorInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for AllocatorInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut AllocatorInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Calculated statistics of memory usage e.g. in a specific memory type, heap, custom pool, or total.\n\nThese are fast to calculate.\nSee functions: vmaGetHeapBudgets(), vmaGetPoolStatistics()."]
pub struct Statistics {
    #[doc = "\\brief Number of `VkDeviceMemory` objects - Vulkan memory blocks allocated."]
    pub block_count: u32,
    #[doc = "\\brief Number of #VmaAllocation objects allocated.\n\n    Dedicated allocations have their own blocks, so each one adds 1 to `allocationCount` as well as `blockCount`."]
    pub allocation_count: u32,
    #[doc = "\\brief Number of bytes allocated in `VkDeviceMemory` blocks.\n\n    \\note To avoid confusion, please be aware that what Vulkan calls an \"allocation\" - a whole `VkDeviceMemory` object\n    (e.g. as in `VkPhysicalDeviceLimits::maxMemoryAllocationCount`) is called a \"block\" in VMA, while VMA calls\n    \"allocation\" a #VmaAllocation object that represents a memory region sub-allocated from such block, usually for a single buffer or image."]
    pub block_bytes: vk::DeviceSize,
    #[doc = "\\brief Total number of bytes occupied by all #VmaAllocation objects.\n\n    Always less or equal than `blockBytes`.\n    Difference `(blockBytes - allocationBytes)` is the amount of memory allocated from Vulkan\n    but unused by any #VmaAllocation."]
    pub allocation_bytes: vk::DeviceSize,
}
impl Default for Statistics {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Statistics {
    pub fn builder<'a>() -> StatisticsBuilder<'a> {
        StatisticsBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StatisticsBuilder<'a> {
    inner: Statistics,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> StatisticsBuilder<'a> {
    pub fn block_count(mut self, block_count: u32) -> Self {
        self.inner.block_count = block_count;
        self
    }
    pub fn allocation_count(mut self, allocation_count: u32) -> Self {
        self.inner.allocation_count = allocation_count;
        self
    }
    pub fn block_bytes(mut self, block_bytes: vk::DeviceSize) -> Self {
        self.inner.block_bytes = block_bytes;
        self
    }
    pub fn allocation_bytes(mut self, allocation_bytes: vk::DeviceSize) -> Self {
        self.inner.allocation_bytes = allocation_bytes;
        self
    }
}
impl<'a> ::std::ops::Deref for StatisticsBuilder<'a> {
    type Target = Statistics;
    fn deref(&self) -> &Statistics {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for StatisticsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Statistics {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief More detailed statistics than #VmaStatistics.\n\nThese are slower to calculate. Use for debugging purposes.\nSee functions: vmaCalculateStatistics(), vmaCalculatePoolStatistics().\n\nPrevious version of the statistics API provided averages, but they have been removed\nbecause they can be easily calculated as:\n\n\\code\nVkDeviceSize allocationSizeAvg = detailedStats.statistics.allocationBytes / detailedStats.statistics.allocationCount;\nVkDeviceSize unusedBytes = detailedStats.statistics.blockBytes - detailedStats.statistics.allocationBytes;\nVkDeviceSize unusedRangeSizeAvg = unusedBytes / detailedStats.unusedRangeCount;\n\\endcode"]
pub struct DetailedStatistics {
    #[doc = "Basic statistics."]
    pub statistics: Statistics,
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
    pub fn builder<'a>() -> DetailedStatisticsBuilder<'a> {
        DetailedStatisticsBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DetailedStatisticsBuilder<'a> {
    inner: DetailedStatistics,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DetailedStatisticsBuilder<'a> {
    pub fn statistics(mut self, statistics: Statistics) -> Self {
        self.inner.statistics = statistics;
        self
    }
    pub fn unused_range_count(mut self, unused_range_count: u32) -> Self {
        self.inner.unused_range_count = unused_range_count;
        self
    }
    pub fn allocation_size_min(mut self, allocation_size_min: vk::DeviceSize) -> Self {
        self.inner.allocation_size_min = allocation_size_min;
        self
    }
    pub fn allocation_size_max(mut self, allocation_size_max: vk::DeviceSize) -> Self {
        self.inner.allocation_size_max = allocation_size_max;
        self
    }
    pub fn unused_range_size_min(mut self, unused_range_size_min: vk::DeviceSize) -> Self {
        self.inner.unused_range_size_min = unused_range_size_min;
        self
    }
    pub fn unused_range_size_max(mut self, unused_range_size_max: vk::DeviceSize) -> Self {
        self.inner.unused_range_size_max = unused_range_size_max;
        self
    }
}
impl<'a> ::std::ops::Deref for DetailedStatisticsBuilder<'a> {
    type Target = DetailedStatistics;
    fn deref(&self) -> &DetailedStatistics {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for DetailedStatisticsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut DetailedStatistics {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief  General statistics from current state of the Allocator -\ntotal memory usage across all memory heaps and types.\n\nThese are slower to calculate. Use for debugging purposes.\nSee function vmaCalculateStatistics()."]
pub struct TotalStatistics {
    pub memory_type: [DetailedStatistics; 32usize],
    pub memory_heap: [DetailedStatistics; 16usize],
    pub total: DetailedStatistics,
}
impl Default for TotalStatistics {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl TotalStatistics {
    pub fn builder<'a>() -> TotalStatisticsBuilder<'a> {
        TotalStatisticsBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TotalStatisticsBuilder<'a> {
    inner: TotalStatistics,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> TotalStatisticsBuilder<'a> {
    pub fn memory_type(mut self, memory_type: [DetailedStatistics; 32usize]) -> Self {
        self.inner.memory_type = memory_type;
        self
    }
    pub fn memory_heap(mut self, memory_heap: [DetailedStatistics; 16usize]) -> Self {
        self.inner.memory_heap = memory_heap;
        self
    }
    pub fn total(mut self, total: DetailedStatistics) -> Self {
        self.inner.total = total;
        self
    }
}
impl<'a> ::std::ops::Deref for TotalStatisticsBuilder<'a> {
    type Target = TotalStatistics;
    fn deref(&self) -> &TotalStatistics {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for TotalStatisticsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut TotalStatistics {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Statistics of current memory usage and available budget for a specific memory heap.\n\nThese are fast to calculate.\nSee function vmaGetHeapBudgets()."]
pub struct Budget {
    #[doc = "\\brief Statistics fetched from the library."]
    pub statistics: Statistics,
    #[doc = "\\brief Estimated current memory usage of the program, in bytes.\n\n    Fetched from system using VK_EXT_memory_budget extension if enabled.\n\n    It might be different than `statistics.blockBytes` (usually higher) due to additional implicit objects\n    also occupying the memory, like swapchain, pipelines, descriptor heaps, command buffers, or\n    `VkDeviceMemory` blocks allocated outside of this library, if any."]
    pub usage: vk::DeviceSize,
    #[doc = "\\brief Estimated amount of memory available to the program, in bytes.\n\n    Fetched from system using VK_EXT_memory_budget extension if enabled.\n\n    It might be different (most probably smaller) than `VkMemoryHeap::size[heapIndex]` due to factors\n    external to the program, decided by the operating system.\n    Difference `budget - usage` is the amount of additional memory that can probably\n    be allocated without problems. Exceeding the budget may result in various problems."]
    pub budget: vk::DeviceSize,
}
impl Default for Budget {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Budget {
    pub fn builder<'a>() -> BudgetBuilder<'a> {
        BudgetBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BudgetBuilder<'a> {
    inner: Budget,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> BudgetBuilder<'a> {
    pub fn statistics(mut self, statistics: Statistics) -> Self {
        self.inner.statistics = statistics;
        self
    }
    pub fn usage(mut self, usage: vk::DeviceSize) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn budget(mut self, budget: vk::DeviceSize) -> Self {
        self.inner.budget = budget;
        self
    }
}
impl<'a> ::std::ops::Deref for BudgetBuilder<'a> {
    type Target = Budget;
    fn deref(&self) -> &Budget {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for BudgetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Budget {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Parameters of new #VmaAllocation.\n\nTo be used with functions like vmaCreateBuffer(), vmaCreateImage(), and many others."]
pub struct AllocationCreateInfo {
    #[doc = "Use #VmaAllocationCreateFlagBits enum."]
    pub flags: AllocationCreateFlags,
    #[doc = "\\brief Intended usage of memory.\n\n    You can leave #VMA_MEMORY_USAGE_UNKNOWN if you specify memory requirements in other way. \\n\n    If `pool` is not null, this member is ignored."]
    pub usage: MemoryUsage,
    #[doc = "\\brief Flags that must be set in a Memory Type chosen for an allocation.\n\n    Leave 0 if you specify memory requirements in other way. \\n\n    If `pool` is not null, this member is ignored."]
    pub required_flags: vk::MemoryPropertyFlags,
    #[doc = "\\brief Flags that preferably should be set in a memory type chosen for an allocation.\n\n    Set to 0 if no additional flags are preferred. \\n\n    If `pool` is not null, this member is ignored."]
    pub preferred_flags: vk::MemoryPropertyFlags,
    #[doc = "\\brief Bitmask containing one bit set for every memory type acceptable for this allocation.\n\n    Value 0 is equivalent to `UINT32_MAX` - it means any memory type is accepted if\n    it meets other requirements specified by this structure, with no further\n    restrictions on memory type index. \\n\n    If `pool` is not null, this member is ignored."]
    pub memory_type_bits: u32,
    #[doc = "\\brief Pool that this allocation should be created in.\n\n    Leave `VK_NULL_HANDLE` to allocate from default pool. If not null, members:\n    `usage`, `requiredFlags`, `preferredFlags`, `memoryTypeBits` are ignored."]
    pub pool: Pool,
    #[doc = "\\brief Custom general-purpose pointer that will be stored in #VmaAllocation, can be read as VmaAllocationInfo::pUserData and changed using vmaSetAllocationUserData().\n\n    If #VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT is used, it must be either\n    null or pointer to a null-terminated string. The string will be then copied to\n    internal buffer, so it doesn't need to be valid after allocation call."]
    pub p_user_data: *mut ::std::ffi::c_void,
    #[doc = "\\brief A floating-point value between 0 and 1, indicating the priority of the allocation relative to other memory allocations.\n\n    It is used only when #VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT flag was used during creation of the #VmaAllocator object\n    and this allocation ends up as dedicated or is explicitly forced as dedicated using #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.\n    Otherwise, it has the priority of a memory block where it is placed and this variable is ignored."]
    pub priority: f32,
}
impl Default for AllocationCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AllocationCreateInfo {
    pub fn builder<'a>() -> AllocationCreateInfoBuilder<'a> {
        AllocationCreateInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AllocationCreateInfoBuilder<'a> {
    inner: AllocationCreateInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> AllocationCreateInfoBuilder<'a> {
    pub fn flags(mut self, flags: AllocationCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn usage(mut self, usage: MemoryUsage) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn required_flags(mut self, required_flags: vk::MemoryPropertyFlags) -> Self {
        self.inner.required_flags = required_flags;
        self
    }
    pub fn preferred_flags(mut self, preferred_flags: vk::MemoryPropertyFlags) -> Self {
        self.inner.preferred_flags = preferred_flags;
        self
    }
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.inner.memory_type_bits = memory_type_bits;
        self
    }
    pub fn pool(mut self, pool: Pool) -> Self {
        self.inner.pool = pool;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.inner.priority = priority;
        self
    }
}
impl<'a> ::std::ops::Deref for AllocationCreateInfoBuilder<'a> {
    type Target = AllocationCreateInfo;
    fn deref(&self) -> &AllocationCreateInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for AllocationCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut AllocationCreateInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Describes parameter of created #VmaPool."]
pub struct PoolCreateInfo {
    #[doc = "\\brief Vulkan memory type index to allocate this pool from."]
    pub memory_type_index: u32,
    #[doc = "\\brief Use combination of #VmaPoolCreateFlagBits."]
    pub flags: PoolCreateFlags,
    #[doc = "\\brief Size of a single `VkDeviceMemory` block to be allocated as part of this pool, in bytes. Optional.\n\n    Specify nonzero to set explicit, constant size of memory blocks used by this\n    pool.\n\n    Leave 0 to use default and let the library manage block sizes automatically.\n    Sizes of particular blocks may vary.\n    In this case, the pool will also support dedicated allocations."]
    pub block_size: vk::DeviceSize,
    #[doc = "\\brief Minimum number of blocks to be always allocated in this pool, even if they stay empty.\n\n    Set to 0 to have no preallocated blocks and allow the pool be completely empty."]
    pub min_block_count: usize,
    #[doc = "\\brief Maximum number of blocks that can be allocated in this pool. Optional.\n\n    Set to 0 to use default, which is `SIZE_MAX`, which means no limit.\n\n    Set to same value as VmaPoolCreateInfo::minBlockCount to have fixed amount of memory allocated\n    throughout whole lifetime of this pool."]
    pub max_block_count: usize,
    #[doc = "\\brief A floating-point value between 0 and 1, indicating the priority of the allocations in this pool relative to other memory allocations.\n\n    It is used only when #VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT flag was used during creation of the #VmaAllocator object.\n    Otherwise, this variable is ignored."]
    pub priority: f32,
    #[doc = "\\brief Additional minimum alignment to be used for all allocations created from this pool. Can be 0.\n\n    Leave 0 (default) not to impose any additional alignment. If not 0, it must be a power of two.\n    It can be useful in cases where alignment returned by Vulkan by functions like `vkGetBufferMemoryRequirements` is not enough,\n    e.g. when doing interop with OpenGL."]
    pub min_allocation_alignment: vk::DeviceSize,
    #[doc = "\\brief Additional `pNext` chain to be attached to `VkMemoryAllocateInfo` used for every allocation made by this pool. Optional.\n\n    Optional, can be null. If not null, it must point to a `pNext` chain of structures that can be attached to `VkMemoryAllocateInfo`.\n    It can be useful for special needs such as adding `VkExportMemoryAllocateInfoKHR`.\n    Structures pointed by this member must remain alive and unchanged for the whole lifetime of the custom pool.\n\n    Please note that some structures, e.g. `VkMemoryPriorityAllocateInfoEXT`, `VkMemoryDedicatedAllocateInfoKHR`,\n    can be attached automatically by this library when using other, more convenient of its features."]
    pub p_memory_allocate_next: *mut ::std::ffi::c_void,
}
impl Default for PoolCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl PoolCreateInfo {
    pub fn builder<'a>() -> PoolCreateInfoBuilder<'a> {
        PoolCreateInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PoolCreateInfoBuilder<'a> {
    inner: PoolCreateInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> PoolCreateInfoBuilder<'a> {
    pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
        self.inner.memory_type_index = memory_type_index;
        self
    }
    pub fn flags(mut self, flags: PoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn block_size(mut self, block_size: vk::DeviceSize) -> Self {
        self.inner.block_size = block_size;
        self
    }
    pub fn min_block_count(mut self, min_block_count: usize) -> Self {
        self.inner.min_block_count = min_block_count;
        self
    }
    pub fn max_block_count(mut self, max_block_count: usize) -> Self {
        self.inner.max_block_count = max_block_count;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.inner.priority = priority;
        self
    }
    pub fn min_allocation_alignment(mut self, min_allocation_alignment: vk::DeviceSize) -> Self {
        self.inner.min_allocation_alignment = min_allocation_alignment;
        self
    }
    pub fn memory_allocate_next(mut self, p_memory_allocate_next: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_memory_allocate_next = p_memory_allocate_next;
        self
    }
}
impl<'a> ::std::ops::Deref for PoolCreateInfoBuilder<'a> {
    type Target = PoolCreateInfo;
    fn deref(&self) -> &PoolCreateInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for PoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut PoolCreateInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of #VmaAllocation objects, that can be retrieved using function vmaGetAllocationInfo()."]
pub struct AllocationInfo {
    #[doc = "\\brief Memory type index that this allocation was allocated from.\n\n    It never changes."]
    pub memory_type: u32,
    #[doc = "\\brief Handle to Vulkan memory object.\n\n    Same memory object can be shared by multiple allocations.\n\n    It can change after the allocation is moved during \\ref defragmentation."]
    pub device_memory: vk::DeviceMemory,
    #[doc = "\\brief Offset in `VkDeviceMemory` object to the beginning of this allocation, in bytes. `(deviceMemory, offset)` pair is unique to this allocation.\n\n    You usually don't need to use this offset. If you create a buffer or an image together with the allocation using e.g. function\n    vmaCreateBuffer(), vmaCreateImage(), functions that operate on these resources refer to the beginning of the buffer or image,\n    not entire device memory block. Functions like vmaMapMemory(), vmaBindBufferMemory() also refer to the beginning of the allocation\n    and apply this offset automatically.\n\n    It can change after the allocation is moved during \\ref defragmentation."]
    pub offset: vk::DeviceSize,
    #[doc = "\\brief Size of this allocation, in bytes.\n\n    It never changes.\n\n    \\note Allocation size returned in this variable may be greater than the size\n    requested for the resource e.g. as `VkBufferCreateInfo::size`. Whole size of the\n    allocation is accessible for operations on memory e.g. using a pointer after\n    mapping with vmaMapMemory(), but operations on the resource e.g. using\n    `vkCmdCopyBuffer` must be limited to the size of the resource."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Pointer to the beginning of this allocation as mapped data.\n\n    If the allocation hasn't been mapped using vmaMapMemory() and hasn't been\n    created with #VMA_ALLOCATION_CREATE_MAPPED_BIT flag, this value is null.\n\n    It can change after call to vmaMapMemory(), vmaUnmapMemory().\n    It can also change after the allocation is moved during \\ref defragmentation."]
    pub p_mapped_data: *mut ::std::ffi::c_void,
    #[doc = "\\brief Custom general-purpose pointer that was passed as VmaAllocationCreateInfo::pUserData or set using vmaSetAllocationUserData().\n\n    It can change after call to vmaSetAllocationUserData() for this allocation."]
    pub p_user_data: *mut ::std::ffi::c_void,
    #[doc = "\\brief Custom allocation name that was set with vmaSetAllocationName().\n\n    It can change after call to vmaSetAllocationName() for this allocation.\n\n    Another way to set custom name is to pass it in VmaAllocationCreateInfo::pUserData with\n    additional flag #VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT set [DEPRECATED]."]
    pub p_name: *const ::std::ffi::c_char,
}
impl Default for AllocationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl AllocationInfo {
    pub fn builder<'a>() -> AllocationInfoBuilder<'a> {
        AllocationInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AllocationInfoBuilder<'a> {
    inner: AllocationInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> AllocationInfoBuilder<'a> {
    pub fn memory_type(mut self, memory_type: u32) -> Self {
        self.inner.memory_type = memory_type;
        self
    }
    pub fn device_memory(mut self, device_memory: vk::DeviceMemory) -> Self {
        self.inner.device_memory = device_memory;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn mapped_data(mut self, p_mapped_data: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_mapped_data = p_mapped_data;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
    pub fn name(mut self, p_name: *const ::std::ffi::c_char) -> Self {
        self.inner.p_name = p_name;
        self
    }
}
impl<'a> ::std::ops::Deref for AllocationInfoBuilder<'a> {
    type Target = AllocationInfo;
    fn deref(&self) -> &AllocationInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for AllocationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut AllocationInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Parameters for defragmentation.\n\nTo be used with function vmaBeginDefragmentation()."]
pub struct DefragmentationInfo {
    #[doc = "\\brief Use combination of #VmaDefragmentationFlagBits."]
    pub flags: DefragmentationFlags,
    #[doc = "\\brief Custom pool to be defragmented.\n\n    If null then default pools will undergo defragmentation process."]
    pub pool: Pool,
    #[doc = "\\brief Maximum numbers of bytes that can be copied during single pass, while moving allocations to different places.\n\n    `0` means no limit."]
    pub max_bytes_per_pass: vk::DeviceSize,
    #[doc = "\\brief Maximum number of allocations that can be moved during single pass to a different place.\n\n    `0` means no limit."]
    pub max_allocations_per_pass: u32,
}
impl Default for DefragmentationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DefragmentationInfo {
    pub fn builder<'a>() -> DefragmentationInfoBuilder<'a> {
        DefragmentationInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DefragmentationInfoBuilder<'a> {
    inner: DefragmentationInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DefragmentationInfoBuilder<'a> {
    pub fn flags(mut self, flags: DefragmentationFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pool(mut self, pool: Pool) -> Self {
        self.inner.pool = pool;
        self
    }
    pub fn max_bytes_per_pass(mut self, max_bytes_per_pass: vk::DeviceSize) -> Self {
        self.inner.max_bytes_per_pass = max_bytes_per_pass;
        self
    }
    pub fn max_allocations_per_pass(mut self, max_allocations_per_pass: u32) -> Self {
        self.inner.max_allocations_per_pass = max_allocations_per_pass;
        self
    }
}
impl<'a> ::std::ops::Deref for DefragmentationInfoBuilder<'a> {
    type Target = DefragmentationInfo;
    fn deref(&self) -> &DefragmentationInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for DefragmentationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut DefragmentationInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Single move of an allocation to be done for defragmentation."]
pub struct DefragmentationMove {
    #[doc = "Operation to be performed on the allocation by vmaEndDefragmentationPass(). Default value is #VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY. You can modify it."]
    pub operation: DefragmentationMoveOperation,
    #[doc = "Allocation that should be moved."]
    pub src_allocation: Allocation,
    #[doc = "\\brief Temporary allocation pointing to destination memory that will replace `srcAllocation`.\n\n    \\warning Do not store this allocation in your data structures! It exists only temporarily, for the duration of the defragmentation pass,\n    to be used for binding new buffer/image to the destination memory using e.g. vmaBindBufferMemory().\n    vmaEndDefragmentationPass() will destroy it and make `srcAllocation` point to this memory."]
    pub dst_tmp_allocation: Allocation,
}
impl Default for DefragmentationMove {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DefragmentationMove {
    pub fn builder<'a>() -> DefragmentationMoveBuilder<'a> {
        DefragmentationMoveBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DefragmentationMoveBuilder<'a> {
    inner: DefragmentationMove,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DefragmentationMoveBuilder<'a> {
    pub fn operation(mut self, operation: DefragmentationMoveOperation) -> Self {
        self.inner.operation = operation;
        self
    }
    pub fn src_allocation(mut self, src_allocation: Allocation) -> Self {
        self.inner.src_allocation = src_allocation;
        self
    }
    pub fn dst_tmp_allocation(mut self, dst_tmp_allocation: Allocation) -> Self {
        self.inner.dst_tmp_allocation = dst_tmp_allocation;
        self
    }
}
impl<'a> ::std::ops::Deref for DefragmentationMoveBuilder<'a> {
    type Target = DefragmentationMove;
    fn deref(&self) -> &DefragmentationMove {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for DefragmentationMoveBuilder<'a> {
    fn deref_mut(&mut self) -> &mut DefragmentationMove {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "\\brief Parameters for incremental defragmentation steps.\n\nTo be used with function vmaBeginDefragmentationPass()."]
pub struct DefragmentationPassMoveInfo {
    #[doc = "Number of elements in the `pMoves` array."]
    pub move_count: u32,
    #[doc = "\\brief Array of moves to be performed by the user in the current defragmentation pass.\n\n    Pointer to an array of `moveCount` elements, owned by VMA, created in vmaBeginDefragmentationPass(), destroyed in vmaEndDefragmentationPass().\n\n    For each element, you should:\n\n    1. Create a new buffer/image in the place pointed by VmaDefragmentationMove::dstMemory + VmaDefragmentationMove::dstOffset.\n    2. Copy data from the VmaDefragmentationMove::srcAllocation e.g. using `vkCmdCopyBuffer`, `vkCmdCopyImage`.\n    3. Make sure these commands finished executing on the GPU.\n    4. Destroy the old buffer/image.\n\n    Only then you can finish defragmentation pass by calling vmaEndDefragmentationPass().\n    After this call, the allocation will point to the new place in memory.\n\n    Alternatively, if you cannot move specific allocation, you can set VmaDefragmentationMove::operation to #VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE.\n\n    Alternatively, if you decide you want to completely remove the allocation:\n\n    1. Destroy its buffer/image.\n    2. Set VmaDefragmentationMove::operation to #VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY.\n\n    Then, after vmaEndDefragmentationPass() the allocation will be freed."]
    pub p_moves: *mut DefragmentationMove,
}
impl Default for DefragmentationPassMoveInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl DefragmentationPassMoveInfo {
    pub fn builder<'a>() -> DefragmentationPassMoveInfoBuilder<'a> {
        DefragmentationPassMoveInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DefragmentationPassMoveInfoBuilder<'a> {
    inner: DefragmentationPassMoveInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DefragmentationPassMoveInfoBuilder<'a> {
    pub fn moves(mut self, p_moves: &'a mut [DefragmentationMove]) -> Self {
        self.p_moves = p_moves.as_mut_ptr();
        self.inner.move_count = p_moves.len() as _;
        self
    }
}
impl<'a> ::std::ops::Deref for DefragmentationPassMoveInfoBuilder<'a> {
    type Target = DefragmentationPassMoveInfo;
    fn deref(&self) -> &DefragmentationPassMoveInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for DefragmentationPassMoveInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut DefragmentationPassMoveInfo {
        &mut self.inner
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
    pub fn builder<'a>() -> DefragmentationStatsBuilder<'a> {
        DefragmentationStatsBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DefragmentationStatsBuilder<'a> {
    inner: DefragmentationStats,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DefragmentationStatsBuilder<'a> {
    pub fn bytes_moved(mut self, bytes_moved: vk::DeviceSize) -> Self {
        self.inner.bytes_moved = bytes_moved;
        self
    }
    pub fn bytes_freed(mut self, bytes_freed: vk::DeviceSize) -> Self {
        self.inner.bytes_freed = bytes_freed;
        self
    }
    pub fn allocations_moved(mut self, allocations_moved: u32) -> Self {
        self.inner.allocations_moved = allocations_moved;
        self
    }
    pub fn device_memory_blocks_freed(mut self, device_memory_blocks_freed: u32) -> Self {
        self.inner.device_memory_blocks_freed = device_memory_blocks_freed;
        self
    }
}
impl<'a> ::std::ops::Deref for DefragmentationStatsBuilder<'a> {
    type Target = DefragmentationStats;
    fn deref(&self) -> &DefragmentationStats {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for DefragmentationStatsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut DefragmentationStats {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of created #VmaVirtualBlock object to be passed to vmaCreateVirtualBlock()."]
pub struct VirtualBlockCreateInfo {
    #[doc = "\\brief Total size of the virtual block.\n\n    Sizes can be expressed in bytes or any units you want as long as you are consistent in using them.\n    For example, if you allocate from some array of structures, 1 can mean single instance of entire structure."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Use combination of #VmaVirtualBlockCreateFlagBits."]
    pub flags: VirtualBlockCreateFlags,
    #[doc = "\\brief Custom CPU memory allocation callbacks. Optional.\n\n    Optional, can be null. When specified, they will be used for all CPU-side memory allocations."]
    pub p_allocation_callbacks: *const vk::AllocationCallbacks,
}
impl Default for VirtualBlockCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl VirtualBlockCreateInfo {
    pub fn builder<'a>() -> VirtualBlockCreateInfoBuilder<'a> {
        VirtualBlockCreateInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualBlockCreateInfoBuilder<'a> {
    inner: VirtualBlockCreateInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> VirtualBlockCreateInfoBuilder<'a> {
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn flags(mut self, flags: VirtualBlockCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn allocation_callbacks(
        mut self,
        p_allocation_callbacks: &'a vk::AllocationCallbacks,
    ) -> Self {
        self.inner.p_allocation_callbacks = p_allocation_callbacks;
        self
    }
}
impl<'a> ::std::ops::Deref for VirtualBlockCreateInfoBuilder<'a> {
    type Target = VirtualBlockCreateInfo;
    fn deref(&self) -> &VirtualBlockCreateInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for VirtualBlockCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut VirtualBlockCreateInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of created virtual allocation to be passed to vmaVirtualAllocate()."]
pub struct VirtualAllocationCreateInfo {
    #[doc = "\\brief Size of the allocation.\n\n    Cannot be zero."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Required alignment of the allocation. Optional.\n\n    Must be power of two. Special value 0 has the same meaning as 1 - means no special alignment is required, so allocation can start at any offset."]
    pub alignment: vk::DeviceSize,
    #[doc = "\\brief Use combination of #VmaVirtualAllocationCreateFlagBits."]
    pub flags: VirtualAllocationCreateFlags,
    #[doc = "\\brief Custom pointer to be associated with the allocation. Optional.\n\n    It can be any value and can be used for user-defined purposes. It can be fetched or changed later."]
    pub p_user_data: *mut ::std::ffi::c_void,
}
impl Default for VirtualAllocationCreateInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl VirtualAllocationCreateInfo {
    pub fn builder<'a>() -> VirtualAllocationCreateInfoBuilder<'a> {
        VirtualAllocationCreateInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualAllocationCreateInfoBuilder<'a> {
    inner: VirtualAllocationCreateInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> VirtualAllocationCreateInfoBuilder<'a> {
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn alignment(mut self, alignment: vk::DeviceSize) -> Self {
        self.inner.alignment = alignment;
        self
    }
    pub fn flags(mut self, flags: VirtualAllocationCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> ::std::ops::Deref for VirtualAllocationCreateInfoBuilder<'a> {
    type Target = VirtualAllocationCreateInfo;
    fn deref(&self) -> &VirtualAllocationCreateInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for VirtualAllocationCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut VirtualAllocationCreateInfo {
        &mut self.inner
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Parameters of an existing virtual allocation, returned by vmaGetVirtualAllocationInfo()."]
pub struct VirtualAllocationInfo {
    #[doc = "\\brief Offset of the allocation.\n\n    Offset at which the allocation was made."]
    pub offset: vk::DeviceSize,
    #[doc = "\\brief Size of the allocation.\n\n    Same value as passed in VmaVirtualAllocationCreateInfo::size."]
    pub size: vk::DeviceSize,
    #[doc = "\\brief Custom pointer associated with the allocation.\n\n    Same value as passed in VmaVirtualAllocationCreateInfo::pUserData or to vmaSetVirtualAllocationUserData()."]
    pub p_user_data: *mut ::std::ffi::c_void,
}
impl Default for VirtualAllocationInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl VirtualAllocationInfo {
    pub fn builder<'a>() -> VirtualAllocationInfoBuilder<'a> {
        VirtualAllocationInfoBuilder::default()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualAllocationInfoBuilder<'a> {
    inner: VirtualAllocationInfo,
    _p: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> VirtualAllocationInfoBuilder<'a> {
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn user_data(mut self, p_user_data: *mut ::std::ffi::c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> ::std::ops::Deref for VirtualAllocationInfoBuilder<'a> {
    type Target = VirtualAllocationInfo;
    fn deref(&self) -> &VirtualAllocationInfo {
        &self.inner
    }
}
impl<'a> ::std::ops::DerefMut for VirtualAllocationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut VirtualAllocationInfo {
        &mut self.inner
    }
}
#[doc = "Creates #VmaAllocator object."]
pub unsafe fn create_allocator(
    p_create_info: &AllocatorCreateInfo,
) -> Result<Allocator, vk::Result> {
    extern "C" {
        fn vmaCreateAllocator(
            p_create_info: *const AllocatorCreateInfo,
            p_allocator: *mut Allocator,
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
pub unsafe fn destroy_allocator(allocator: Allocator) {
    extern "C" {
        fn vmaDestroyAllocator(allocator: Allocator);
    }
    vmaDestroyAllocator(allocator);
}
#[doc = "\\brief Returns information about existing #VmaAllocator object - handle to Vulkan device etc.\n\nIt might be useful if you want to keep just the #VmaAllocator handle and fetch other required handles to\n`VkPhysicalDevice`, `VkDevice` etc. every time using this function."]
pub unsafe fn get_allocator_info(allocator: Allocator) -> AllocatorInfo {
    extern "C" {
        fn vmaGetAllocatorInfo(allocator: Allocator, p_allocator_info: *mut AllocatorInfo);
    }
    let mut p_allocator_info = ::std::mem::zeroed();
    vmaGetAllocatorInfo(allocator, &mut p_allocator_info);
    p_allocator_info
}
#[doc = "PhysicalDeviceProperties are fetched from physicalDevice by the allocator.\nYou can access it here, without fetching it again on your own."]
pub unsafe fn get_physical_device_properties(
    allocator: Allocator,
) -> *const vk::PhysicalDeviceProperties {
    extern "C" {
        fn vmaGetPhysicalDeviceProperties(
            allocator: Allocator,
            pp_physical_device_properties: *mut *const vk::PhysicalDeviceProperties,
        );
    }
    let mut pp_physical_device_properties = ::std::mem::zeroed();
    vmaGetPhysicalDeviceProperties(allocator, &mut pp_physical_device_properties);
    pp_physical_device_properties
}
#[doc = "PhysicalDeviceMemoryProperties are fetched from physicalDevice by the allocator.\nYou can access it here, without fetching it again on your own."]
pub unsafe fn get_memory_properties(
    allocator: Allocator,
) -> *const vk::PhysicalDeviceMemoryProperties {
    extern "C" {
        fn vmaGetMemoryProperties(
            allocator: Allocator,
            pp_physical_device_memory_properties: *mut *const vk::PhysicalDeviceMemoryProperties,
        );
    }
    let mut pp_physical_device_memory_properties = ::std::mem::zeroed();
    vmaGetMemoryProperties(allocator, &mut pp_physical_device_memory_properties);
    pp_physical_device_memory_properties
}
#[doc = "\\brief Given Memory Type Index, returns Property Flags of this memory type.\n\nThis is just a convenience function. Same information can be obtained using\nvmaGetMemoryProperties()."]
pub unsafe fn get_memory_type_properties(
    allocator: Allocator,
    memory_type_index: u32,
) -> vk::MemoryPropertyFlags {
    extern "C" {
        fn vmaGetMemoryTypeProperties(
            allocator: Allocator,
            memory_type_index: u32,
            p_flags: *mut vk::MemoryPropertyFlags,
        );
    }
    let mut p_flags = ::std::mem::zeroed();
    vmaGetMemoryTypeProperties(allocator, memory_type_index, &mut p_flags);
    p_flags
}
#[doc = "\\brief Sets index of the current frame."]
pub unsafe fn set_current_frame_index(allocator: Allocator, frame_index: u32) {
    extern "C" {
        fn vmaSetCurrentFrameIndex(allocator: Allocator, frame_index: u32);
    }
    vmaSetCurrentFrameIndex(allocator, frame_index);
}
#[doc = "\\brief Retrieves statistics from current state of the Allocator.\n\nThis function is called \"calculate\" not \"get\" because it has to traverse all\ninternal data structures, so it may be quite slow. Use it for debugging purposes.\nFor faster but more brief statistics suitable to be called every frame or every allocation,\nuse vmaGetHeapBudgets().\n\nNote that when using allocator from multiple threads, returned information may immediately\nbecome outdated."]
pub unsafe fn calculate_statistics(allocator: Allocator) -> TotalStatistics {
    extern "C" {
        fn vmaCalculateStatistics(allocator: Allocator, p_stats: *mut TotalStatistics);
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaCalculateStatistics(allocator, &mut p_stats);
    p_stats
}
#[doc = "\\brief Retrieves information about current memory usage and budget for all memory heaps.\n\n\\param allocator\n\\param[out] pBudgets Must point to array with number of elements at least equal to number of memory heaps in physical device used.\n\nThis function is called \"get\" not \"calculate\" because it is very fast, suitable to be called\nevery frame or every allocation. For more detailed statistics use vmaCalculateStatistics().\n\nNote that when using allocator from multiple threads, returned information may immediately\nbecome outdated."]
pub unsafe fn get_heap_budgets(allocator: Allocator) -> Vec<Budget> {
    extern "C" {
        fn vmaGetHeapBudgets(allocator: Allocator, p_budgets: *mut Budget);
    }
    let p_budgets_len = (*get_memory_properties(allocator)).memory_heap_count;
    let mut p_budgets = vec![::std::mem::zeroed(); p_budgets_len as _];
    vmaGetHeapBudgets(allocator, p_budgets.as_mut_ptr());
    p_budgets
}
#[doc = "\\brief Helps to find memoryTypeIndex, given memoryTypeBits and VmaAllocationCreateInfo.\n\nThis algorithm tries to find a memory type that:\n\n- Is allowed by memoryTypeBits.\n- Contains all the flags from pAllocationCreateInfo->requiredFlags.\n- Matches intended usage.\n- Has as many flags from pAllocationCreateInfo->preferredFlags as possible.\n\n\\return Returns VK_ERROR_FEATURE_NOT_PRESENT if not found. Receiving such result\nfrom this function or any other allocating function probably means that your\ndevice doesn't support any memory type with requested features for the specific\ntype of resource you want to use it for. Please check parameters of your\nresource, like image layout (OPTIMAL versus LINEAR) or mip level count."]
pub unsafe fn find_memory_type_index(
    allocator: Allocator,
    memory_type_bits: u32,
    p_allocation_create_info: &AllocationCreateInfo,
) -> Result<u32, vk::Result> {
    extern "C" {
        fn vmaFindMemoryTypeIndex(
            allocator: Allocator,
            memory_type_bits: u32,
            p_allocation_create_info: *const AllocationCreateInfo,
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
    allocator: Allocator,
    p_buffer_create_info: &vk::BufferCreateInfo,
    p_allocation_create_info: &AllocationCreateInfo,
) -> Result<u32, vk::Result> {
    extern "C" {
        fn vmaFindMemoryTypeIndexForBufferInfo(
            allocator: Allocator,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_allocation_create_info: *const AllocationCreateInfo,
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
    allocator: Allocator,
    p_image_create_info: &vk::ImageCreateInfo,
    p_allocation_create_info: &AllocationCreateInfo,
) -> Result<u32, vk::Result> {
    extern "C" {
        fn vmaFindMemoryTypeIndexForImageInfo(
            allocator: Allocator,
            p_image_create_info: *const vk::ImageCreateInfo,
            p_allocation_create_info: *const AllocationCreateInfo,
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
    allocator: Allocator,
    p_create_info: &PoolCreateInfo,
) -> Result<Pool, vk::Result> {
    extern "C" {
        fn vmaCreatePool(
            allocator: Allocator,
            p_create_info: *const PoolCreateInfo,
            p_pool: *mut Pool,
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
pub unsafe fn destroy_pool(allocator: Allocator, pool: Pool) {
    extern "C" {
        fn vmaDestroyPool(allocator: Allocator, pool: Pool);
    }
    vmaDestroyPool(allocator, pool);
}
#[doc = "\\brief Retrieves statistics of existing #VmaPool object.\n\n\\param allocator Allocator object.\n\\param pool Pool object.\n\\param[out] pPoolStats Statistics of specified pool."]
pub unsafe fn get_pool_statistics(allocator: Allocator, pool: Pool) -> Statistics {
    extern "C" {
        fn vmaGetPoolStatistics(allocator: Allocator, pool: Pool, p_pool_stats: *mut Statistics);
    }
    let mut p_pool_stats = ::std::mem::zeroed();
    vmaGetPoolStatistics(allocator, pool, &mut p_pool_stats);
    p_pool_stats
}
#[doc = "\\brief Retrieves detailed statistics of existing #VmaPool object.\n\n\\param allocator Allocator object.\n\\param pool Pool object.\n\\param[out] pPoolStats Statistics of specified pool."]
pub unsafe fn calculate_pool_statistics(allocator: Allocator, pool: Pool) -> DetailedStatistics {
    extern "C" {
        fn vmaCalculatePoolStatistics(
            allocator: Allocator,
            pool: Pool,
            p_pool_stats: *mut DetailedStatistics,
        );
    }
    let mut p_pool_stats = ::std::mem::zeroed();
    vmaCalculatePoolStatistics(allocator, pool, &mut p_pool_stats);
    p_pool_stats
}
#[doc = "\\brief Checks magic number in margins around all allocations in given memory pool in search for corruptions.\n\nCorruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to nonzero,\n`VMA_DEBUG_MARGIN` is defined to nonzero and the pool is created in memory type that is\n`HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref debugging_memory_usage_corruption_detection).\n\nPossible return values:\n\n- `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for specified pool.\n- `VK_SUCCESS` - corruption detection has been performed and succeeded.\n- `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions around one of the allocations.\n  `VMA_ASSERT` is also fired in that case.\n- Other value: Error returned by Vulkan, e.g. memory mapping failure."]
pub unsafe fn check_pool_corruption(allocator: Allocator, pool: Pool) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaCheckPoolCorruption(allocator: Allocator, pool: Pool) -> vk::Result;
    }
    let result = vmaCheckPoolCorruption(allocator, pool);
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Retrieves name of a custom pool.\n\nAfter the call `ppName` is either null or points to an internally-owned null-terminated string\ncontaining name of the pool that was previously set. The pointer becomes invalid when the pool is\ndestroyed or its name is changed using vmaSetPoolName()."]
pub unsafe fn get_pool_name(allocator: Allocator, pool: Pool) -> *const ::std::ffi::c_char {
    extern "C" {
        fn vmaGetPoolName(
            allocator: Allocator,
            pool: Pool,
            pp_name: *mut *const ::std::ffi::c_char,
        );
    }
    let mut pp_name = ::std::mem::zeroed();
    vmaGetPoolName(allocator, pool, &mut pp_name);
    pp_name
}
#[doc = "\\brief Sets name of a custom pool.\n\n`pName` can be either null or pointer to a null-terminated string with new name for the pool.\nFunction makes internal copy of the string, so it can be changed or freed immediately after this call."]
pub unsafe fn set_pool_name(allocator: Allocator, pool: Pool, p_name: *const ::std::ffi::c_char) {
    extern "C" {
        fn vmaSetPoolName(allocator: Allocator, pool: Pool, p_name: *const ::std::ffi::c_char);
    }
    vmaSetPoolName(allocator, pool, p_name);
}
#[doc = "\\brief General purpose memory allocation.\n\n\\param allocator\n\\param pVkMemoryRequirements\n\\param pCreateInfo\n\\param[out] pAllocation Handle to allocated memory.\n\\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().\n\nYou should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().\n\nIt is recommended to use vmaAllocateMemoryForBuffer(), vmaAllocateMemoryForImage(),\nvmaCreateBuffer(), vmaCreateImage() instead whenever possible."]
pub unsafe fn allocate_memory(
    allocator: Allocator,
    p_vk_memory_requirements: &vk::MemoryRequirements,
    p_create_info: &AllocationCreateInfo,
) -> Result<(Allocation, AllocationInfo), vk::Result> {
    extern "C" {
        fn vmaAllocateMemory(
            allocator: Allocator,
            p_vk_memory_requirements: *const vk::MemoryRequirements,
            p_create_info: *const AllocationCreateInfo,
            p_allocation: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
pub unsafe fn allocate_memory_pages(
    allocator: Allocator,
    p_vk_memory_requirements: &[vk::MemoryRequirements],
    p_create_info: &[AllocationCreateInfo],
) -> Result<(Vec<Allocation>, Vec<AllocationInfo>), vk::Result> {
    extern "C" {
        fn vmaAllocateMemoryPages(
            allocator: Allocator,
            p_vk_memory_requirements: *const vk::MemoryRequirements,
            p_create_info: *const AllocationCreateInfo,
            allocation_count: usize,
            p_allocations: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
pub unsafe fn allocate_memory_for_buffer(
    allocator: Allocator,
    buffer: vk::Buffer,
    p_create_info: &AllocationCreateInfo,
) -> Result<(Allocation, AllocationInfo), vk::Result> {
    extern "C" {
        fn vmaAllocateMemoryForBuffer(
            allocator: Allocator,
            buffer: vk::Buffer,
            p_create_info: *const AllocationCreateInfo,
            p_allocation: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
pub unsafe fn allocate_memory_for_image(
    allocator: Allocator,
    image: vk::Image,
    p_create_info: &AllocationCreateInfo,
) -> Result<(Allocation, AllocationInfo), vk::Result> {
    extern "C" {
        fn vmaAllocateMemoryForImage(
            allocator: Allocator,
            image: vk::Image,
            p_create_info: *const AllocationCreateInfo,
            p_allocation: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
pub unsafe fn free_memory(allocator: Allocator, allocation: Allocation) {
    extern "C" {
        fn vmaFreeMemory(allocator: Allocator, allocation: Allocation);
    }
    vmaFreeMemory(allocator, allocation);
}
#[doc = "\\brief Frees memory and destroys multiple allocations.\n\nWord \"pages\" is just a suggestion to use this function to free pieces of memory used for sparse binding.\nIt is just a general purpose function to free memory and destroy allocations made using e.g. vmaAllocateMemory(),\nvmaAllocateMemoryPages() and other functions.\nIt may be internally optimized to be more efficient than calling vmaFreeMemory() `allocationCount` times.\n\nAllocations in `pAllocations` array can come from any memory pools and types.\nPassing `VK_NULL_HANDLE` as elements of `pAllocations` array is valid. Such entries are just skipped."]
pub unsafe fn free_memory_pages(allocator: Allocator, p_allocations: &[Allocation]) {
    extern "C" {
        fn vmaFreeMemoryPages(
            allocator: Allocator,
            allocation_count: usize,
            p_allocations: *const Allocation,
        );
    }
    vmaFreeMemoryPages(allocator, p_allocations.len() as _, p_allocations.as_ptr());
}
#[doc = "\\brief Returns current information about specified allocation.\n\nCurrent parameters of given allocation are returned in `pAllocationInfo`.\n\nAlthough this function doesn't lock any mutex, so it should be quite efficient,\nyou should avoid calling it too often.\nYou can retrieve same VmaAllocationInfo structure while creating your resource, from function\nvmaCreateBuffer(), vmaCreateImage(). You can remember it if you are sure parameters don't change\n(e.g. due to defragmentation)."]
pub unsafe fn get_allocation_info(allocator: Allocator, allocation: Allocation) -> AllocationInfo {
    extern "C" {
        fn vmaGetAllocationInfo(
            allocator: Allocator,
            allocation: Allocation,
            p_allocation_info: *mut AllocationInfo,
        );
    }
    let mut p_allocation_info = ::std::mem::zeroed();
    vmaGetAllocationInfo(allocator, allocation, &mut p_allocation_info);
    p_allocation_info
}
#[doc = "\\brief Sets pUserData in given allocation to new value.\n\nThe value of pointer `pUserData` is copied to allocation's `pUserData`.\nIt is opaque, so you can use it however you want - e.g.\nas a pointer, ordinal number or some handle to you own data."]
pub unsafe fn set_allocation_user_data(
    allocator: Allocator,
    allocation: Allocation,
    p_user_data: *mut ::std::ffi::c_void,
) {
    extern "C" {
        fn vmaSetAllocationUserData(
            allocator: Allocator,
            allocation: Allocation,
            p_user_data: *mut ::std::ffi::c_void,
        );
    }
    vmaSetAllocationUserData(allocator, allocation, p_user_data);
}
#[doc = "\\brief Sets pName in given allocation to new value.\n\n`pName` must be either null, or pointer to a null-terminated string. The function\nmakes local copy of the string and sets it as allocation's `pName`. String\npassed as pName doesn't need to be valid for whole lifetime of the allocation -\nyou can free it after this call. String previously pointed by allocation's\n`pName` is freed from memory."]
pub unsafe fn set_allocation_name(
    allocator: Allocator,
    allocation: Allocation,
    p_name: *const ::std::ffi::c_char,
) {
    extern "C" {
        fn vmaSetAllocationName(
            allocator: Allocator,
            allocation: Allocation,
            p_name: *const ::std::ffi::c_char,
        );
    }
    vmaSetAllocationName(allocator, allocation, p_name);
}
#[doc = "\\brief Given an allocation, returns Property Flags of its memory type.\n\nThis is just a convenience function. Same information can be obtained using\nvmaGetAllocationInfo() + vmaGetMemoryProperties()."]
pub unsafe fn get_allocation_memory_properties(
    allocator: Allocator,
    allocation: Allocation,
) -> vk::MemoryPropertyFlags {
    extern "C" {
        fn vmaGetAllocationMemoryProperties(
            allocator: Allocator,
            allocation: Allocation,
            p_flags: *mut vk::MemoryPropertyFlags,
        );
    }
    let mut p_flags = ::std::mem::zeroed();
    vmaGetAllocationMemoryProperties(allocator, allocation, &mut p_flags);
    p_flags
}
#[doc = "\\brief Maps memory represented by given allocation and returns pointer to it.\n\nMaps memory represented by given allocation to make it accessible to CPU code.\nWhen succeeded, `*ppData` contains pointer to first byte of this memory.\n\n\\warning\nIf the allocation is part of a bigger `VkDeviceMemory` block, returned pointer is\ncorrectly offsetted to the beginning of region assigned to this particular allocation.\nUnlike the result of `vkMapMemory`, it points to the allocation, not to the beginning of the whole block.\nYou should not add VmaAllocationInfo::offset to it!\n\nMapping is internally reference-counted and synchronized, so despite raw Vulkan\nfunction `vkMapMemory()` cannot be used to map same block of `VkDeviceMemory`\nmultiple times simultaneously, it is safe to call this function on allocations\nassigned to the same memory block. Actual Vulkan memory will be mapped on first\nmapping and unmapped on last unmapping.\n\nIf the function succeeded, you must call vmaUnmapMemory() to unmap the\nallocation when mapping is no longer needed or before freeing the allocation, at\nthe latest.\n\nIt also safe to call this function multiple times on the same allocation. You\nmust call vmaUnmapMemory() same number of times as you called vmaMapMemory().\n\nIt is also safe to call this function on allocation created with\n#VMA_ALLOCATION_CREATE_MAPPED_BIT flag. Its memory stays mapped all the time.\nYou must still call vmaUnmapMemory() same number of times as you called\nvmaMapMemory(). You must not call vmaUnmapMemory() additional time to free the\n\"0-th\" mapping made automatically due to #VMA_ALLOCATION_CREATE_MAPPED_BIT flag.\n\nThis function fails when used on allocation made in memory type that is not\n`HOST_VISIBLE`.\n\nThis function doesn't automatically flush or invalidate caches.\nIf the allocation is made from a memory types that is not `HOST_COHERENT`,\nyou also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification."]
pub unsafe fn map_memory(
    allocator: Allocator,
    allocation: Allocation,
) -> Result<*mut ::std::ffi::c_void, vk::Result> {
    extern "C" {
        fn vmaMapMemory(
            allocator: Allocator,
            allocation: Allocation,
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
pub unsafe fn unmap_memory(allocator: Allocator, allocation: Allocation) {
    extern "C" {
        fn vmaUnmapMemory(allocator: Allocator, allocation: Allocation);
    }
    vmaUnmapMemory(allocator, allocation);
}
#[doc = "\\brief Flushes memory of given allocation.\n\nCalls `vkFlushMappedMemoryRanges()` for memory associated with given range of given allocation.\nIt needs to be called after writing to a mapped memory for memory types that are not `HOST_COHERENT`.\nUnmap operation doesn't do that automatically.\n\n- `offset` must be relative to the beginning of allocation.\n- `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given allocation.\n- `offset` and `size` don't have to be aligned.\n  They are internally rounded down/up to multiply of `nonCoherentAtomSize`.\n- If `size` is 0, this call is ignored.\n- If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is `HOST_COHERENT`,\n  this call is ignored.\n\nWarning! `offset` and `size` are relative to the contents of given `allocation`.\nIf you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.\nDo not pass allocation's offset as `offset`!!!\n\nThis function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is\ncalled, otherwise `VK_SUCCESS`."]
pub unsafe fn flush_allocation(
    allocator: Allocator,
    allocation: Allocation,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaFlushAllocation(
            allocator: Allocator,
            allocation: Allocation,
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
    allocator: Allocator,
    allocation: Allocation,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaInvalidateAllocation(
            allocator: Allocator,
            allocation: Allocation,
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
#[doc = "\\brief Flushes memory of given set of allocations.\n\nCalls `vkFlushMappedMemoryRanges()` for memory associated with given ranges of given allocations.\nFor more information, see documentation of vmaFlushAllocation().\n\n\\param allocator\n\\param allocationCount\n\\param allocations\n\\param offsets If not null, it must point to an array of offsets of regions to flush, relative to the beginning of respective allocations. Null means all ofsets are zero.\n\\param sizes If not null, it must point to an array of sizes of regions to flush in respective allocations. Null means `VK_WHOLE_SIZE` for all allocations.\n\nThis function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is\ncalled, otherwise `VK_SUCCESS`."]
pub unsafe fn flush_allocations(
    allocator: Allocator,
    allocations: &[Allocation],
    offsets: &[vk::DeviceSize],
    sizes: &[vk::DeviceSize],
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaFlushAllocations(
            allocator: Allocator,
            allocation_count: u32,
            allocations: *const Allocation,
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
#[doc = "\\brief Invalidates memory of given set of allocations.\n\nCalls `vkInvalidateMappedMemoryRanges()` for memory associated with given ranges of given allocations.\nFor more information, see documentation of vmaInvalidateAllocation().\n\n\\param allocator\n\\param allocationCount\n\\param allocations\n\\param offsets If not null, it must point to an array of offsets of regions to flush, relative to the beginning of respective allocations. Null means all ofsets are zero.\n\\param sizes If not null, it must point to an array of sizes of regions to flush in respective allocations. Null means `VK_WHOLE_SIZE` for all allocations.\n\nThis function returns the `VkResult` from `vkInvalidateMappedMemoryRanges` if it is\ncalled, otherwise `VK_SUCCESS`."]
pub unsafe fn invalidate_allocations(
    allocator: Allocator,
    allocations: &[Allocation],
    offsets: &[vk::DeviceSize],
    sizes: &[vk::DeviceSize],
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaInvalidateAllocations(
            allocator: Allocator,
            allocation_count: u32,
            allocations: *const Allocation,
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
#[doc = "\\brief Checks magic number in margins around all allocations in given memory types (in both default and custom pools) in search for corruptions.\n\n\\param allocator\n\\param memoryTypeBits Bit mask, where each bit set means that a memory type with that index should be checked.\n\nCorruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to nonzero,\n`VMA_DEBUG_MARGIN` is defined to nonzero and only for memory types that are\n`HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref debugging_memory_usage_corruption_detection).\n\nPossible return values:\n\n- `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for any of specified memory types.\n- `VK_SUCCESS` - corruption detection has been performed and succeeded.\n- `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions around one of the allocations.\n  `VMA_ASSERT` is also fired in that case.\n- Other value: Error returned by Vulkan, e.g. memory mapping failure."]
pub unsafe fn check_corruption(
    allocator: Allocator,
    memory_type_bits: u32,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaCheckCorruption(allocator: Allocator, memory_type_bits: u32) -> vk::Result;
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
    allocator: Allocator,
    p_info: &DefragmentationInfo,
) -> Result<DefragmentationContext, vk::Result> {
    extern "C" {
        fn vmaBeginDefragmentation(
            allocator: Allocator,
            p_info: *const DefragmentationInfo,
            p_context: *mut DefragmentationContext,
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
    allocator: Allocator,
    context: DefragmentationContext,
) -> DefragmentationStats {
    extern "C" {
        fn vmaEndDefragmentation(
            allocator: Allocator,
            context: DefragmentationContext,
            p_stats: *mut DefragmentationStats,
        );
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaEndDefragmentation(allocator, context, &mut p_stats);
    p_stats
}
#[doc = "\\brief Starts single defragmentation pass.\n\n\\param allocator Allocator object.\n\\param context Context object that has been created by vmaBeginDefragmentation().\n\\param[out] pPassInfo Computed information for current pass.\n\\returns\n- `VK_SUCCESS` if no more moves are possible. Then you can omit call to vmaEndDefragmentationPass() and simply end whole defragmentation.\n- `VK_INCOMPLETE` if there are pending moves returned in `pPassInfo`. You need to perform them, call vmaEndDefragmentationPass(),\n  and then preferably try another pass with vmaBeginDefragmentationPass()."]
pub unsafe fn begin_defragmentation_pass(
    allocator: Allocator,
    context: DefragmentationContext,
) -> Result<DefragmentationPassMoveInfo, vk::Result> {
    extern "C" {
        fn vmaBeginDefragmentationPass(
            allocator: Allocator,
            context: DefragmentationContext,
            p_pass_info: *mut DefragmentationPassMoveInfo,
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
pub unsafe fn end_defragmentation_pass(
    allocator: Allocator,
    context: DefragmentationContext,
) -> Result<DefragmentationPassMoveInfo, vk::Result> {
    extern "C" {
        fn vmaEndDefragmentationPass(
            allocator: Allocator,
            context: DefragmentationContext,
            p_pass_info: *mut DefragmentationPassMoveInfo,
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
    allocator: Allocator,
    allocation: Allocation,
    buffer: vk::Buffer,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindBufferMemory(
            allocator: Allocator,
            allocation: Allocation,
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
    allocator: Allocator,
    allocation: Allocation,
    allocation_local_offset: vk::DeviceSize,
    buffer: vk::Buffer,
    p_next: &::std::ffi::c_void,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindBufferMemory2(
            allocator: Allocator,
            allocation: Allocation,
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
        p_next,
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Binds image to allocation.\n\nBinds specified image to region of memory represented by specified allocation.\nGets `VkDeviceMemory` handle and offset from the allocation.\nIf you want to create an image, allocate memory for it and bind them together separately,\nyou should use this function for binding instead of standard `vkBindImageMemory()`,\nbecause it ensures proper synchronization so that when a `VkDeviceMemory` object is used by multiple\nallocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple threads simultaneously\n(which is illegal in Vulkan).\n\nIt is recommended to use function vmaCreateImage() instead of this one."]
pub unsafe fn bind_image_memory(
    allocator: Allocator,
    allocation: Allocation,
    image: vk::Image,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindImageMemory(
            allocator: Allocator,
            allocation: Allocation,
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
    allocator: Allocator,
    allocation: Allocation,
    allocation_local_offset: vk::DeviceSize,
    image: vk::Image,
    p_next: &::std::ffi::c_void,
) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaBindImageMemory2(
            allocator: Allocator,
            allocation: Allocation,
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
        p_next,
    );
    if result == vk::Result::SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}
#[doc = "\\brief Creates a new `VkBuffer`, allocates and binds memory for it.\n\n\\param allocator\n\\param pBufferCreateInfo\n\\param pAllocationCreateInfo\n\\param[out] pBuffer Buffer that was created.\n\\param[out] pAllocation Allocation that was created.\n\\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().\n\nThis function automatically:\n\n-# Creates buffer.\n-# Allocates appropriate memory for it.\n-# Binds the buffer with the memory.\n\nIf any of these operations fail, buffer and allocation are not created,\nreturned value is negative error code, `*pBuffer` and `*pAllocation` are null.\n\nIf the function succeeded, you must destroy both buffer and allocation when you\nno longer need them using either convenience function vmaDestroyBuffer() or\nseparately, using `vkDestroyBuffer()` and vmaFreeMemory().\n\nIf #VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT flag was used,\nVK_KHR_dedicated_allocation extension is used internally to query driver whether\nit requires or prefers the new buffer to have dedicated allocation. If yes,\nand if dedicated allocation is possible\n(#VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT is not used), it creates dedicated\nallocation for this buffer, just like when using\n#VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.\n\n\\note This function creates a new `VkBuffer`. Sub-allocation of parts of one large buffer,\nalthough recommended as a good practice, is out of scope of this library and could be implemented\nby the user as a higher-level logic on top of VMA."]
pub unsafe fn create_buffer(
    allocator: Allocator,
    p_buffer_create_info: &vk::BufferCreateInfo,
    p_allocation_create_info: &AllocationCreateInfo,
) -> Result<(vk::Buffer, Allocation, AllocationInfo), vk::Result> {
    extern "C" {
        fn vmaCreateBuffer(
            allocator: Allocator,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_allocation_create_info: *const AllocationCreateInfo,
            p_buffer: *mut vk::Buffer,
            p_allocation: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
pub unsafe fn create_buffer_with_alignment(
    allocator: Allocator,
    p_buffer_create_info: &vk::BufferCreateInfo,
    p_allocation_create_info: &AllocationCreateInfo,
    min_alignment: vk::DeviceSize,
) -> Result<(vk::Buffer, Allocation, AllocationInfo), vk::Result> {
    extern "C" {
        fn vmaCreateBufferWithAlignment(
            allocator: Allocator,
            p_buffer_create_info: *const vk::BufferCreateInfo,
            p_allocation_create_info: *const AllocationCreateInfo,
            min_alignment: vk::DeviceSize,
            p_buffer: *mut vk::Buffer,
            p_allocation: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
    allocator: Allocator,
    allocation: Allocation,
    p_buffer_create_info: &vk::BufferCreateInfo,
) -> Result<vk::Buffer, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingBuffer(
            allocator: Allocator,
            allocation: Allocation,
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
    allocator: Allocator,
    allocation: Allocation,
    allocation_local_offset: vk::DeviceSize,
    p_buffer_create_info: &vk::BufferCreateInfo,
) -> Result<vk::Buffer, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingBuffer2(
            allocator: Allocator,
            allocation: Allocation,
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
pub unsafe fn destroy_buffer(allocator: Allocator, buffer: vk::Buffer, allocation: Allocation) {
    extern "C" {
        fn vmaDestroyBuffer(allocator: Allocator, buffer: vk::Buffer, allocation: Allocation);
    }
    vmaDestroyBuffer(allocator, buffer, allocation);
}
#[doc = "Function similar to vmaCreateBuffer()."]
pub unsafe fn create_image(
    allocator: Allocator,
    p_image_create_info: &vk::ImageCreateInfo,
    p_allocation_create_info: &AllocationCreateInfo,
) -> Result<(vk::Image, Allocation, AllocationInfo), vk::Result> {
    extern "C" {
        fn vmaCreateImage(
            allocator: Allocator,
            p_image_create_info: *const vk::ImageCreateInfo,
            p_allocation_create_info: *const AllocationCreateInfo,
            p_image: *mut vk::Image,
            p_allocation: *mut Allocation,
            p_allocation_info: *mut AllocationInfo,
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
    allocator: Allocator,
    allocation: Allocation,
    p_image_create_info: &vk::ImageCreateInfo,
) -> Result<vk::Image, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingImage(
            allocator: Allocator,
            allocation: Allocation,
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
    allocator: Allocator,
    allocation: Allocation,
    allocation_local_offset: vk::DeviceSize,
    p_image_create_info: &vk::ImageCreateInfo,
) -> Result<vk::Image, vk::Result> {
    extern "C" {
        fn vmaCreateAliasingImage2(
            allocator: Allocator,
            allocation: Allocation,
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
pub unsafe fn destroy_image(allocator: Allocator, image: vk::Image, allocation: Allocation) {
    extern "C" {
        fn vmaDestroyImage(allocator: Allocator, image: vk::Image, allocation: Allocation);
    }
    vmaDestroyImage(allocator, image, allocation);
}
#[doc = "\\brief Creates new #VmaVirtualBlock object.\n\n\\param pCreateInfo Parameters for creation.\n\\param[out] pVirtualBlock Returned virtual block object or `VMA_NULL` if creation failed."]
pub unsafe fn create_virtual_block(
    p_create_info: &VirtualBlockCreateInfo,
) -> Result<VirtualBlock, vk::Result> {
    extern "C" {
        fn vmaCreateVirtualBlock(
            p_create_info: *const VirtualBlockCreateInfo,
            p_virtual_block: *mut VirtualBlock,
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
pub unsafe fn destroy_virtual_block(virtual_block: VirtualBlock) {
    extern "C" {
        fn vmaDestroyVirtualBlock(virtual_block: VirtualBlock);
    }
    vmaDestroyVirtualBlock(virtual_block);
}
#[doc = "\\brief Returns true of the #VmaVirtualBlock is empty - contains 0 virtual allocations and has all its space available for new allocations."]
pub unsafe fn is_virtual_block_empty(virtual_block: VirtualBlock) -> Result<(), vk::Result> {
    extern "C" {
        fn vmaIsVirtualBlockEmpty(virtual_block: VirtualBlock) -> vk::Result;
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
    virtual_block: VirtualBlock,
    allocation: VirtualAllocation,
) -> VirtualAllocationInfo {
    extern "C" {
        fn vmaGetVirtualAllocationInfo(
            virtual_block: VirtualBlock,
            allocation: VirtualAllocation,
            p_virtual_alloc_info: *mut VirtualAllocationInfo,
        );
    }
    let mut p_virtual_alloc_info = ::std::mem::zeroed();
    vmaGetVirtualAllocationInfo(virtual_block, allocation, &mut p_virtual_alloc_info);
    p_virtual_alloc_info
}
#[doc = "\\brief Allocates new virtual allocation inside given #VmaVirtualBlock.\n\nIf the allocation fails due to not enough free space available, `VK_ERROR_OUT_OF_DEVICE_MEMORY` is returned\n(despite the function doesn't ever allocate actual GPU memory).\n`pAllocation` is then set to `VK_NULL_HANDLE` and `pOffset`, if not null, it set to `UINT64_MAX`.\n\n\\param virtualBlock Virtual block\n\\param pCreateInfo Parameters for the allocation\n\\param[out] pAllocation Returned handle of the new allocation\n\\param[out] pOffset Returned offset of the new allocation. Optional, can be null."]
pub unsafe fn virtual_allocate(
    virtual_block: VirtualBlock,
    p_create_info: &VirtualAllocationCreateInfo,
) -> Result<(VirtualAllocation, vk::DeviceSize), vk::Result> {
    extern "C" {
        fn vmaVirtualAllocate(
            virtual_block: VirtualBlock,
            p_create_info: *const VirtualAllocationCreateInfo,
            p_allocation: *mut VirtualAllocation,
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
pub unsafe fn virtual_free(virtual_block: VirtualBlock, allocation: VirtualAllocation) {
    extern "C" {
        fn vmaVirtualFree(virtual_block: VirtualBlock, allocation: VirtualAllocation);
    }
    vmaVirtualFree(virtual_block, allocation);
}
#[doc = "\\brief Frees all virtual allocations inside given #VmaVirtualBlock.\n\nYou must either call this function or free each virtual allocation individually with vmaVirtualFree()\nbefore destroying a virtual block. Otherwise, an assert is called.\n\nIf you keep pointer to some additional metadata associated with your virtual allocation in its `pUserData`,\ndon't forget to free it as well."]
pub unsafe fn clear_virtual_block(virtual_block: VirtualBlock) {
    extern "C" {
        fn vmaClearVirtualBlock(virtual_block: VirtualBlock);
    }
    vmaClearVirtualBlock(virtual_block);
}
#[doc = "\\brief Changes custom pointer associated with given virtual allocation."]
pub unsafe fn set_virtual_allocation_user_data(
    virtual_block: VirtualBlock,
    allocation: VirtualAllocation,
    p_user_data: *mut ::std::ffi::c_void,
) {
    extern "C" {
        fn vmaSetVirtualAllocationUserData(
            virtual_block: VirtualBlock,
            allocation: VirtualAllocation,
            p_user_data: *mut ::std::ffi::c_void,
        );
    }
    vmaSetVirtualAllocationUserData(virtual_block, allocation, p_user_data);
}
#[doc = "\\brief Calculates and returns statistics about virtual allocations and memory usage in given #VmaVirtualBlock.\n\nThis function is fast to call. For more detailed statistics, see vmaCalculateVirtualBlockStatistics()."]
pub unsafe fn get_virtual_block_statistics(virtual_block: VirtualBlock) -> Statistics {
    extern "C" {
        fn vmaGetVirtualBlockStatistics(virtual_block: VirtualBlock, p_stats: *mut Statistics);
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaGetVirtualBlockStatistics(virtual_block, &mut p_stats);
    p_stats
}
#[doc = "\\brief Calculates and returns detailed statistics about virtual allocations and memory usage in given #VmaVirtualBlock.\n\nThis function is slow to call. Use for debugging purposes.\nFor less detailed statistics, see vmaGetVirtualBlockStatistics()."]
pub unsafe fn calculate_virtual_block_statistics(
    virtual_block: VirtualBlock,
) -> DetailedStatistics {
    extern "C" {
        fn vmaCalculateVirtualBlockStatistics(
            virtual_block: VirtualBlock,
            p_stats: *mut DetailedStatistics,
        );
    }
    let mut p_stats = ::std::mem::zeroed();
    vmaCalculateVirtualBlockStatistics(virtual_block, &mut p_stats);
    p_stats
}
#[doc = "\\brief Builds and returns a null-terminated string in JSON format with information about given #VmaVirtualBlock.\n\\param virtualBlock Virtual block.\n\\param[out] ppStatsString Returned string.\n\\param detailedMap Pass `VK_FALSE` to only obtain statistics as returned by vmaCalculateVirtualBlockStatistics(). Pass `VK_TRUE` to also obtain full list of allocations and free spaces.\n\nReturned string must be freed using vmaFreeVirtualBlockStatsString()."]
pub unsafe fn build_virtual_block_stats_string(
    virtual_block: VirtualBlock,
    detailed_map: vk::Bool32,
) -> *mut ::std::ffi::c_char {
    extern "C" {
        fn vmaBuildVirtualBlockStatsString(
            virtual_block: VirtualBlock,
            pp_stats_string: *mut *mut ::std::ffi::c_char,
            detailed_map: vk::Bool32,
        );
    }
    let mut pp_stats_string = ::std::mem::zeroed();
    vmaBuildVirtualBlockStatsString(virtual_block, &mut pp_stats_string, detailed_map);
    pp_stats_string
}
#[doc = "Frees a string returned by vmaBuildVirtualBlockStatsString()."]
pub unsafe fn free_virtual_block_stats_string(virtual_block: VirtualBlock) -> ::std::ffi::c_char {
    extern "C" {
        fn vmaFreeVirtualBlockStatsString(
            virtual_block: VirtualBlock,
            p_stats_string: *mut ::std::ffi::c_char,
        );
    }
    let mut p_stats_string = ::std::mem::zeroed();
    vmaFreeVirtualBlockStatsString(virtual_block, &mut p_stats_string);
    p_stats_string
}
#[doc = "\\brief Builds and returns statistics as a null-terminated string in JSON format.\n\\param allocator\n\\param[out] ppStatsString Must be freed using vmaFreeStatsString() function.\n\\param detailedMap"]
pub unsafe fn build_stats_string(
    allocator: Allocator,
    detailed_map: vk::Bool32,
) -> *mut ::std::ffi::c_char {
    extern "C" {
        fn vmaBuildStatsString(
            allocator: Allocator,
            pp_stats_string: *mut *mut ::std::ffi::c_char,
            detailed_map: vk::Bool32,
        );
    }
    let mut pp_stats_string = ::std::mem::zeroed();
    vmaBuildStatsString(allocator, &mut pp_stats_string, detailed_map);
    pp_stats_string
}
pub unsafe fn free_stats_string(allocator: Allocator) -> ::std::ffi::c_char {
    extern "C" {
        fn vmaFreeStatsString(allocator: Allocator, p_stats_string: *mut ::std::ffi::c_char);
    }
    let mut p_stats_string = ::std::mem::zeroed();
    vmaFreeStatsString(allocator, &mut p_stats_string);
    p_stats_string
}
