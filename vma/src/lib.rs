use std::ptr::{null, null_mut};

use ash::vk::{self, Handle};
use vma_sys::*;

mod enums;
pub use enums::*;

mod structs;
pub use structs::*;

#[derive(Debug)]
pub struct Allocator {
    vma: vma_sys::VmaAllocator,
}

unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

impl Allocator {
    pub fn new(
        physical_device: vk::PhysicalDevice,
        device: &ash::Device,
        instance: &ash::Instance,
        entry: &ash::Entry,
    ) -> Result<Self, vk::Result> {
        let vulkan_functions = VmaVulkanFunctions {
            vkGetInstanceProcAddr: unsafe {
                std::mem::transmute(entry.static_fn().get_instance_proc_addr)
            },
            vkGetDeviceProcAddr: unsafe {
                std::mem::transmute(instance.fp_v1_0().get_device_proc_addr)
            },
            ..unsafe { std::mem::zeroed() }
        };
        let create_info = VmaAllocatorCreateInfo {
            flags: 0,
            physicalDevice: physical_device.as_raw() as _,
            device: device.handle().as_raw() as _,
            preferredLargeHeapBlockSize: 0,
            pAllocationCallbacks: null(),
            pDeviceMemoryCallbacks: null(),
            pHeapSizeLimit: null(),
            pVulkanFunctions: &vulkan_functions,
            instance: instance.handle().as_raw() as _,
            vulkanApiVersion: vk::API_VERSION_1_3,
            pTypeExternalMemoryHandleTypes: null(),
        };

        let mut vma = null_mut();
        let err = unsafe { vmaCreateAllocator(&create_info, &mut vma) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(Self { vma })
        }
    }

    pub fn create_buffer(
        &self,
        buffer_info: &vk::BufferCreateInfo,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<(vk::Buffer, Allocation), vk::Result> {
        let mut buffer = null_mut();
        let mut allocation = null_mut();
        let err = unsafe {
            vmaCreateBuffer(
                self.vma,
                buffer_info as *const _ as _,
                alloc_info as *const _ as *const _,
                &mut buffer,
                &mut allocation,
                null_mut(),
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok((vk::Buffer::from_raw(buffer as _), Allocation(allocation)))
        }
    }

    pub fn destroy_buffer(&self, buffer: vk::Buffer, alloc: Allocation) {
        unsafe {
            vmaDestroyBuffer(self.vma, buffer.as_raw() as _, alloc.0);
        }
    }

    pub fn allocate_memory_for_buffer(
        &self,
        buffer: vk::Buffer,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<Allocation, vk::Result> {
        let mut allocation = null_mut();
        let err = unsafe {
            vmaAllocateMemoryForBuffer(
                self.vma,
                buffer.as_raw() as _,
                alloc_info as *const _ as *const _,
                &mut allocation,
                null_mut(),
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(Allocation(allocation))
        }
    }

    pub fn bind_buffer_memory(
        &self,
        buffer: vk::Buffer,
        allocation: Allocation,
    ) -> Result<(), vk::Result> {
        let err = unsafe { vmaBindBufferMemory(self.vma, allocation.0, buffer.as_raw() as _) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn create_image(
        &self,
        image_info: &vk::ImageCreateInfo,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<(vk::Image, Allocation), vk::Result> {
        let mut image = null_mut();
        let mut allocation = null_mut();
        let err = unsafe {
            vmaCreateImage(
                self.vma,
                image_info as *const _ as _,
                alloc_info as *const _ as *const _,
                &mut image,
                &mut allocation,
                null_mut(),
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok((vk::Image::from_raw(image as _), Allocation(allocation)))
        }
    }

    pub fn destroy_image(&self, image: vk::Image, alloc: Allocation) {
        unsafe {
            vmaDestroyImage(self.vma, image.as_raw() as _, alloc.0);
        }
    }

    pub fn allocate_memory_for_image(
        &self,
        image: vk::Image,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<Allocation, vk::Result> {
        let mut allocation = null_mut();
        let err = unsafe {
            vmaAllocateMemoryForImage(
                self.vma,
                image.as_raw() as _,
                alloc_info as *const _ as *const _,
                &mut allocation,
                null_mut(),
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(Allocation(allocation))
        }
    }

    pub fn bind_image_memory(&self, image: vk::Image, allocation: Allocation) -> Result<(), vk::Result> {
        let err = unsafe{vmaBindImageMemory(self.vma, allocation.0, image.as_raw() as _)};
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn free(&self, allocation: Allocation) {
        unsafe {
            vmaFreeMemory(self.vma, allocation.0);
        }
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe {
            vmaDestroyAllocator(self.vma);
        }
    }
}
