
# vma-rs
This library provides ash-style bindings to the Vulkan Memory Allocator library (https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator).

Since the functions take the ash `vk::XXX` versions of Vulkan objects, it is only usable with the ash Vulkan bindings.
The API is designed similar to `ash`, meaning objects have to be manually created/deleted without any library-side safety guarantees.

## Basic initialization
Initialization is as easy as calling `Allocator::new()` with the appropriate Vulkan objects.
```rust
let entry = unsafe { ash::Entry::load().expect("Failed to load ash") };
let (instance, device, physical_device) = init_vulkan(&entry);

let vma = vma::Allocator::new(&entry, &instance, physical_device, &device, vk::API_VERSION_1_0)
    .expect("Failed to create Allocator");
```

## Raw allocation algorithm
VMA also exposes an interface for directly using the allocation algorithm without actually allocating any memory.
This is exposed via the `vma::VirtualBlock` struct.
```rust
let block = VirtualBlock::new(&VirtualBlockCreateInfo {
    size: 1024,
    flags: VirtualBlockCreateFlags::empty(),
    allocation_callbacks: None,
})
.expect("Failed to create virtual block");

let (allocation, offset) = block
    .allocate(&VirtualAllocationCreateInfo {
        size: 123,
        alignment: 128,
        flags: VirtualAllocationCreateFlags::empty(),
        user_data: 42 as *mut c_void,
    })
    .expect("Failed to allocate");
```

The rest of the functions pretty much exactly match their C counterparts.
