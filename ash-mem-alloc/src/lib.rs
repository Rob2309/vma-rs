//! # ash-mem-alloc
//!
//! This crate provides auto generated [ash](https://github.com/ash-rs/ash)-style bindings to the beloved [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator).
//!
//! Every VMA struct has an associated `builder` just like ash structs:
//! ```rust ignore
//! use ash_mem_alloc::vma;
//!
//! let info = vma::AllocatorCreateInfo::builder()
//!     .device(device.handle())
//!     .instance(instance.handle())
//!     .physical_device(physical_device)
//!     .vulkan_functions(&functions);
//! ```
//!
//! While certain convenience features are implemented, all functions are unsafe and operate on raw `Vma` and `Vk` handles, just like in `ash`.
//! For the most part, functions will generally behave as one would expect from identical `ash` functions.

#[allow(clippy::missing_safety_doc)]
mod bindings;
mod function_ptrs;
mod handles;

pub mod vma {
    pub use super::bindings::*;
    pub use super::function_ptrs::*;
    pub use super::handles::*;
}
