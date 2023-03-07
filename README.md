
# vma-rs
This library provides ash-style bindings to the Vulkan Memory Allocator library (https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator).

Since the functions take the ash `vk::XXX` versions of Vulkan objects, it is only usable with the ash Vulkan bindings.
The API is designed similar to `ash`, meaning most objects have to be manually created/deleted without any library-side safety guarantees. However, the `vma::Allocator` and `vma::VirtualBlock` objects will destroy themselves automatically when dropped.

## Basic initialization
Initialization is as easy as calling `Allocator::new()` with the appropriate Vulkan objects.
```rust
let entry = unsafe { ash::Entry::load().expect("Failed to load ash") };
let (instance, device, physical_device) = init_vulkan(&entry);

let vma = vma::Allocator::new(&entry, &instance, physical_device, &device, vk::API_VERSION_1_0)
    .expect("Failed to create Allocator");
```

The rest of the functions pretty much exactly match their C counterparts.
