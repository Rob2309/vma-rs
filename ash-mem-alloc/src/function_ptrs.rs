#![allow(non_camel_case_types)]

use std::ffi::c_void;

use ash::vk;

use crate::vma;

pub type PFN_vmaAllocateDeviceMemoryFunction = Option<
    unsafe extern "system" fn(vma::Allocator, u32, vk::DeviceMemory, vk::DeviceSize, *mut c_void),
>;
pub type PFN_vmaFreeDeviceMemoryFunction = Option<
    unsafe extern "system" fn(vma::Allocator, u32, vk::DeviceMemory, vk::DeviceSize, *mut c_void),
>;
