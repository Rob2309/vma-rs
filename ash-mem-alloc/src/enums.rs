#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags for created #VmaAllocator."]
pub struct AllocatorCreateFlags(u32);
impl AllocatorCreateFlags {
    #[doc = "\\brief Allocator and all objects created from it will not be synchronized internally, so you must guarantee they are used from only one thread at a time or synchronized externally by you.\n\n Using this flag may increase performance because internal mutexes are not used."]
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1);
    #[doc = "\\brief Enables usage of VK_KHR_dedicated_allocation extension.\n\n The flag works only if VmaAllocatorCreateInfo::vulkanApiVersion `== VK_API_VERSION_1_0`.\n When it is `VK_API_VERSION_1_1`, the flag is ignored because the extension has been promoted to Vulkan 1.1.\n\n Using this extension will automatically allocate dedicated blocks of memory for\n some buffers and images instead of suballocating place for them out of bigger\n memory blocks (as if you explicitly used #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT\n flag) when it is recommended by the driver. It may improve performance on some\n GPUs.\n\n You may set this flag only if you found out that following device extensions are\n supported, you enabled them while creating Vulkan device passed as\n VmaAllocatorCreateInfo::device, and you want them to be used internally by this\n library:\n\n - VK_KHR_get_memory_requirements2 (device extension)\n - VK_KHR_dedicated_allocation (device extension)\n\n When this flag is set, you can experience following warnings reported by Vulkan\n validation layer. You can ignore them.\n\n > vkBindBufferMemory(): Binding memory to buffer 0x2d but vkGetBufferMemoryRequirements() has not been called on that buffer."]
    pub const KHR_DEDICATED_ALLOCATION: Self = Self(2);
    #[doc = "Enables usage of VK_KHR_bind_memory2 extension.\n\n The flag works only if VmaAllocatorCreateInfo::vulkanApiVersion `== VK_API_VERSION_1_0`.\n When it is `VK_API_VERSION_1_1`, the flag is ignored because the extension has been promoted to Vulkan 1.1.\n\n You may set this flag only if you found out that this device extension is supported,\n you enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,\n and you want it to be used internally by this library.\n\n The extension provides functions `vkBindBufferMemory2KHR` and `vkBindImageMemory2KHR`,\n which allow to pass a chain of `pNext` structures while binding.\n This flag is required if you use `pNext` parameter in vmaBindBufferMemory2() or vmaBindImageMemory2()."]
    pub const KHR_BIND_MEMORY2: Self = Self(4);
    #[doc = "Enables usage of VK_EXT_memory_budget extension.\n\n You may set this flag only if you found out that this device extension is supported,\n you enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,\n and you want it to be used internally by this library, along with another instance extension\n VK_KHR_get_physical_device_properties2, which is required by it (or Vulkan 1.1, where this extension is promoted).\n\n The extension provides query for current memory usage and budget, which will probably\n be more accurate than an estimation used by the library otherwise."]
    pub const EXT_MEMORY_BUDGET: Self = Self(8);
    #[doc = "Enables usage of VK_AMD_device_coherent_memory extension.\n\n You may set this flag only if you:\n\n - found out that this device extension is supported and enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,\n - checked that `VkPhysicalDeviceCoherentMemoryFeaturesAMD::deviceCoherentMemory` is true and set it while creating the Vulkan device,\n - want it to be used internally by this library.\n\n The extension and accompanying device feature provide access to memory types with\n `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` and `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD` flags.\n They are useful mostly for writing breadcrumb markers - a common method for debugging GPU crash/hang/TDR.\n\n When the extension is not enabled, such memory types are still enumerated, but their usage is illegal.\n To protect from this error, if you don't create the allocator with this flag, it will refuse to allocate any memory or create a custom pool in such memory type,\n returning `VK_ERROR_FEATURE_NOT_PRESENT`."]
    pub const AMD_DEVICE_COHERENT_MEMORY: Self = Self(16);
    #[doc = "Enables usage of \"buffer device address\" feature, which allows you to use function\n `vkGetBufferDeviceAddress*` to get raw GPU pointer to a buffer and pass it for usage inside a shader.\n\n You may set this flag only if you:\n\n 1. (For Vulkan version < 1.2) Found as available and enabled device extension\n VK_KHR_buffer_device_address.\n This extension is promoted to core Vulkan 1.2.\n 2. Found as available and enabled device feature `VkPhysicalDeviceBufferDeviceAddressFeatures::bufferDeviceAddress`.\n\n When this flag is set, you can create buffers with `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` using VMA.\n The library automatically adds `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` to\n allocated memory blocks wherever it might be needed.\n\n For more information, see documentation chapter \\ref enabling_buffer_device_address."]
    pub const BUFFER_DEVICE_ADDRESS: Self = Self(32);
    #[doc = "Enables usage of VK_EXT_memory_priority extension in the library.\n\n You may set this flag only if you found available and enabled this device extension,\n along with `VkPhysicalDeviceMemoryPriorityFeaturesEXT::memoryPriority == VK_TRUE`,\n while creating Vulkan device passed as VmaAllocatorCreateInfo::device.\n\n When this flag is used, VmaAllocationCreateInfo::priority and VmaPoolCreateInfo::priority\n are used to set priorities of allocated Vulkan memory. Without it, these variables are ignored.\n\n A priority must be a floating-point value between 0 and 1, indicating the priority of the allocation relative to other memory allocations.\n Larger values are higher priority. The granularity of the priorities is implementation-dependent.\n It is automatically passed to every call to `vkAllocateMemory` done by the library using structure `VkMemoryPriorityAllocateInfoEXT`.\n The value to be used for default priority is 0.5.\n For more details, see the documentation of the VK_EXT_memory_priority extension."]
    pub const EXT_MEMORY_PRIORITY: Self = Self(64);
    #[doc = "Enables usage of VK_KHR_maintenance4 extension in the library.\n\n You may set this flag only if you found available and enabled this device extension,\n while creating Vulkan device passed as VmaAllocatorCreateInfo::device."]
    pub const KHR_MAINTENANCE4: Self = Self(128);
    #[doc = "Enables usage of VK_KHR_maintenance5 extension in the library.\n\n You should set this flag if you found available and enabled this device extension,\n while creating Vulkan device passed as VmaAllocatorCreateInfo::device."]
    pub const KHR_MAINTENANCE5: Self = Self(256);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
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
impl AllocatorCreateFlags {
    #[doc = r" Checks whether `other` is a subset of `self`"]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Checks whether `other` and `self` have bits in common"]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "\\brief Intended usage of the allocated memory."]
pub struct MemoryUsage(u32);
impl MemoryUsage {
    #[doc = "No intended memory usage specified.\n Use other members of VmaAllocationCreateInfo to specify your requirements."]
    pub const UNKNOWN: Self = Self(0);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n Prefers `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`."]
    pub const GPU_ONLY: Self = Self(1);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` and `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`."]
    pub const CPU_ONLY: Self = Self(2);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`, prefers `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`."]
    pub const CPU_TO_GPU: Self = Self(3);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`, prefers `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`."]
    pub const GPU_TO_CPU: Self = Self(4);
    #[doc = "\\deprecated Obsolete, preserved for backward compatibility.\n Prefers not `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`."]
    pub const CPU_COPY: Self = Self(5);
    #[doc = "Lazily allocated GPU memory having `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`.\n Exists mostly on mobile platforms. Using it on desktop PC or other GPUs with no such memory type present will fail the allocation.\n\n Usage: Memory for transient attachment images (color attachments, depth attachments etc.), created with `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`.\n\n Allocations with this usage are always created as dedicated - it implies #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT."]
    pub const GPU_LAZILY_ALLOCATED: Self = Self(6);
    #[doc = "Selects best memory type automatically.\n This flag is recommended for most common use cases.\n\n When using this flag, if you want to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT),\n you must pass one of the flags: #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT\n in VmaAllocationCreateInfo::flags.\n\n It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.\n vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()\n and not with generic memory allocation functions."]
    pub const AUTO: Self = Self(7);
    #[doc = "Selects best memory type automatically with preference for GPU (device) memory.\n\n When using this flag, if you want to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT),\n you must pass one of the flags: #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT\n in VmaAllocationCreateInfo::flags.\n\n It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.\n vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()\n and not with generic memory allocation functions."]
    pub const AUTO_PREFER_DEVICE: Self = Self(8);
    #[doc = "Selects best memory type automatically with preference for CPU (host) memory.\n\n When using this flag, if you want to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT),\n you must pass one of the flags: #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT\n in VmaAllocationCreateInfo::flags.\n\n It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.\n vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()\n and not with generic memory allocation functions."]
    pub const AUTO_PREFER_HOST: Self = Self(9);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaAllocationCreateInfo::flags."]
pub struct AllocationCreateFlags(u32);
impl AllocationCreateFlags {
    #[doc = "\\brief Set this flag if the allocation should have its own memory block.\n\n Use it for special, big resources, like fullscreen images used as attachments.\n\n If you use this flag while creating a buffer or an image, `VkMemoryDedicatedAllocateInfo`\n structure is applied if possible."]
    pub const DEDICATED_MEMORY: Self = Self(1);
    #[doc = "\\brief Set this flag to only try to allocate from existing `VkDeviceMemory` blocks and never create new such block.\n\n If new allocation cannot be placed in any of the existing blocks, allocation\n fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY` error.\n\n You should not use #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT and\n #VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT at the same time. It makes no sense."]
    pub const NEVER_ALLOCATE: Self = Self(2);
    #[doc = "\\brief Set this flag to use a memory that will be persistently mapped and retrieve pointer to it.\n\n Pointer to mapped memory will be returned through VmaAllocationInfo::pMappedData.\n\n It is valid to use this flag for allocation made from memory type that is not\n `HOST_VISIBLE`. This flag is then ignored and memory is not mapped. This is\n useful if you need an allocation that is efficient to use on GPU\n (`DEVICE_LOCAL`) and still want to map it directly if possible on platforms that\n support it (e.g. Intel GPU)."]
    pub const MAPPED: Self = Self(4);
    #[doc = "\\deprecated Preserved for backward compatibility. Consider using vmaSetAllocationName() instead.\n\n Set this flag to treat VmaAllocationCreateInfo::pUserData as pointer to a\n null-terminated string. Instead of copying pointer value, a local copy of the\n string is made and stored in allocation's `pName`. The string is automatically\n freed together with the allocation. It is also used in vmaBuildStatsString()."]
    pub const USER_DATA_COPY_STRING: Self = Self(32);
    #[doc = "Allocation will be created from upper stack in a double stack pool.\n\n This flag is only allowed for custom pools created with #VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT flag."]
    pub const UPPER_ADDRESS: Self = Self(64);
    #[doc = "Create both buffer/image and allocation, but don't bind them together.\n It is useful when you want to bind yourself to do some more advanced binding, e.g. using some extensions.\n The flag is meaningful only with functions that bind by default: vmaCreateBuffer(), vmaCreateImage().\n Otherwise it is ignored.\n\n If you want to make sure the new buffer/image is not tied to the new memory allocation\n through `VkMemoryDedicatedAllocateInfoKHR` structure in case the allocation ends up in its own memory block,\n use also flag #VMA_ALLOCATION_CREATE_CAN_ALIAS_BIT."]
    pub const DONT_BIND: Self = Self(128);
    #[doc = "Create allocation only if additional device memory required for it, if any, won't exceed\n memory budget. Otherwise return `VK_ERROR_OUT_OF_DEVICE_MEMORY`."]
    pub const WITHIN_BUDGET: Self = Self(256);
    #[doc = "\\brief Set this flag if the allocated memory will have aliasing resources.\n\n Usage of this flag prevents supplying `VkMemoryDedicatedAllocateInfoKHR` when #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT is specified.\n Otherwise created dedicated memory will not be suitable for aliasing resources, resulting in Vulkan Validation Layer errors."]
    pub const CAN_ALIAS: Self = Self(512);
    #[doc = "Requests possibility to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT).\n\n - If you use #VMA_MEMORY_USAGE_AUTO or other `VMA_MEMORY_USAGE_AUTO*` value,\n   you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.\n - If you use other value of #VmaMemoryUsage, this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.\n   This includes allocations created in \\ref custom_memory_pools.\n\n Declares that mapped memory will only be written sequentially, e.g. using `memcpy()` or a loop writing number-by-number,\n never read or accessed randomly, so a memory type can be selected that is uncached and write-combined.\n\n \\warning Violating this declaration may work correctly, but will likely be very slow.\n Watch out for implicit reads introduced by doing e.g. `pMappedData[i] += x;`\n Better prepare your data in a local variable and `memcpy()` it to the mapped pointer all at once."]
    pub const HOST_ACCESS_SEQUENTIAL_WRITE: Self = Self(1024);
    #[doc = "Requests possibility to map the allocation (using vmaMapMemory() or #VMA_ALLOCATION_CREATE_MAPPED_BIT).\n\n - If you use #VMA_MEMORY_USAGE_AUTO or other `VMA_MEMORY_USAGE_AUTO*` value,\n   you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.\n - If you use other value of #VmaMemoryUsage, this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.\n   This includes allocations created in \\ref custom_memory_pools.\n\n Declares that mapped memory can be read, written, and accessed in random order,\n so a `HOST_CACHED` memory type is preferred."]
    pub const HOST_ACCESS_RANDOM: Self = Self(2048);
    #[doc = "Together with #VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT or #VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT,\n it says that despite request for host access, a not-`HOST_VISIBLE` memory type can be selected\n if it may improve performance.\n\n By using this flag, you declare that you will check if the allocation ended up in a `HOST_VISIBLE` memory type\n (e.g. using vmaGetAllocationMemoryProperties()) and if not, you will create some \"staging\" buffer and\n issue an explicit transfer to write/read your data.\n To prepare for this possibility, don't forget to add appropriate flags like\n `VK_BUFFER_USAGE_TRANSFER_DST_BIT`, `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` to the parameters of created buffer or image."]
    pub const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD: Self = Self(4096);
    #[doc = "Allocation strategy that chooses smallest possible free range for the allocation\n to minimize memory usage and fragmentation, possibly at the expense of allocation time."]
    pub const STRATEGY_MIN_MEMORY: Self = Self(65536);
    #[doc = "Allocation strategy that chooses first suitable free range for the allocation -\n not necessarily in terms of the smallest offset but the one that is easiest and fastest to find\n to minimize allocation time, possibly at the expense of allocation quality."]
    pub const STRATEGY_MIN_TIME: Self = Self(131072);
    #[doc = "Allocation strategy that chooses always the lowest offset in available space.\n This is not the most efficient strategy but achieves highly packed data.\n Used internally by defragmentation, not recommended in typical usage."]
    pub const STRATEGY_MIN_OFFSET: Self = Self(262144);
    #[doc = "Alias to #VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT."]
    pub const STRATEGY_BEST_FIT: Self = Self(65536);
    #[doc = "Alias to #VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT."]
    pub const STRATEGY_FIRST_FIT: Self = Self(131072);
    #[doc = "A bit mask to extract only `STRATEGY` bits from entire set of flags."]
    pub const STRATEGY_MASK: Self = Self(458752);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
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
impl AllocationCreateFlags {
    #[doc = r" Checks whether `other` is a subset of `self`"]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Checks whether `other` and `self` have bits in common"]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaPoolCreateInfo::flags."]
pub struct PoolCreateFlags(u32);
impl PoolCreateFlags {
    #[doc = "\\brief Use this flag if you always allocate only buffers and linear images or only optimal images out of this pool and so Buffer-Image Granularity can be ignored.\n\n This is an optional optimization flag.\n\n If you always allocate using vmaCreateBuffer(), vmaCreateImage(),\n vmaAllocateMemoryForBuffer(), then you don't need to use it because allocator\n knows exact type of your allocations so it can handle Buffer-Image Granularity\n in the optimal way.\n\n If you also allocate using vmaAllocateMemoryForImage() or vmaAllocateMemory(),\n exact type of such allocations is not known, so allocator must be conservative\n in handling Buffer-Image Granularity, which can lead to suboptimal allocation\n (wasted memory). In that case, if you can make sure you always allocate only\n buffers and linear images or only optimal images out of this pool, use this flag\n to make allocator disregard Buffer-Image Granularity and so make allocations\n faster and more optimal."]
    pub const IGNORE_BUFFER_IMAGE_GRANULARITY: Self = Self(2);
    #[doc = "\\brief Enables alternative, linear allocation algorithm in this pool.\n\n Specify this flag to enable linear allocation algorithm, which always creates\n new allocations after last one and doesn't reuse space from allocations freed in\n between. It trades memory consumption for simplified algorithm and data\n structure, which has better performance and uses less memory for metadata.\n\n By using this flag, you can achieve behavior of free-at-once, stack,\n ring buffer, and double stack.\n For details, see documentation chapter \\ref linear_algorithm."]
    pub const LINEAR_ALGORITHM: Self = Self(4);
    #[doc = "Bit mask to extract only `ALGORITHM` bits from entire set of flags."]
    pub const ALGORITHM_MASK: Self = Self(4);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
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
impl PoolCreateFlags {
    #[doc = r" Checks whether `other` is a subset of `self`"]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Checks whether `other` and `self` have bits in common"]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
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
    #[doc = "\\brief Use the most roboust algorithm at the cost of time to compute and number of copies to make.\n Only available when bufferImageGranularity is greater than 1, since it aims to reduce\n alignment issues between different types of resources.\n Otherwise falls back to same behavior as #VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FULL_BIT."]
    pub const FLAG_ALGORITHM_EXTENSIVE: Self = Self(8);
    #[doc = "A bit mask to extract only `ALGORITHM` bits from entire set of flags."]
    pub const FLAG_ALGORITHM_MASK: Self = Self(15);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
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
impl DefragmentationFlags {
    #[doc = r" Checks whether `other` is a subset of `self`"]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Checks whether `other` and `self` have bits in common"]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
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
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaVirtualBlockCreateInfo::flags."]
pub struct VirtualBlockCreateFlags(u32);
impl VirtualBlockCreateFlags {
    #[doc = "\\brief Enables alternative, linear allocation algorithm in this virtual block.\n\n Specify this flag to enable linear allocation algorithm, which always creates\n new allocations after last one and doesn't reuse space from allocations freed in\n between. It trades memory consumption for simplified algorithm and data\n structure, which has better performance and uses less memory for metadata.\n\n By using this flag, you can achieve behavior of free-at-once, stack,\n ring buffer, and double stack.\n For details, see documentation chapter \\ref linear_algorithm."]
    pub const LINEAR_ALGORITHM: Self = Self(1);
    #[doc = "\\brief Bit mask to extract only `ALGORITHM` bits from entire set of flags."]
    pub const ALGORITHM_MASK: Self = Self(1);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
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
impl VirtualBlockCreateFlags {
    #[doc = r" Checks whether `other` is a subset of `self`"]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Checks whether `other` and `self` have bits in common"]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "Flags to be passed as VmaVirtualAllocationCreateInfo::flags."]
pub struct VirtualAllocationCreateFlags(u32);
impl VirtualAllocationCreateFlags {
    #[doc = "\\brief Allocation will be created from upper stack in a double stack pool.\n\n This flag is only allowed for virtual blocks created with #VMA_VIRTUAL_BLOCK_CREATE_LINEAR_ALGORITHM_BIT flag."]
    pub const UPPER_ADDRESS: Self = Self(64);
    #[doc = "\\brief Allocation strategy that tries to minimize memory usage."]
    pub const STRATEGY_MIN_MEMORY: Self = Self(65536);
    #[doc = "\\brief Allocation strategy that tries to minimize allocation time."]
    pub const STRATEGY_MIN_TIME: Self = Self(131072);
    #[doc = "Allocation strategy that chooses always the lowest offset in available space.\n This is not the most efficient strategy but achieves highly packed data."]
    pub const STRATEGY_MIN_OFFSET: Self = Self(262144);
    #[doc = "\\brief A bit mask to extract only `STRATEGY` bits from entire set of flags.\n\n These strategy flags are binary compatible with equivalent flags in #VmaAllocationCreateFlagBits."]
    pub const STRATEGY_MASK: Self = Self(458752);
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn into_raw(self) -> u32 {
        self.0
    }
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }
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
impl VirtualAllocationCreateFlags {
    #[doc = r" Checks whether `other` is a subset of `self`"]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Checks whether `other` and `self` have bits in common"]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
