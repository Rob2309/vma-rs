use std::{
    ffi::{c_void, CStr, CString},
    mem::MaybeUninit,
    ptr::{null, null_mut},
};

use ash::vk::{self, BindBufferMemoryDeviceGroupInfo, ExtendsBindBufferMemoryInfo, Handle};
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

    pub fn get_info(&self) -> AllocatorInfo {
        unsafe {
            let mut res = MaybeUninit::uninit();
            vmaGetAllocatorInfo(self.vma, res.as_mut_ptr() as _);
            res.assume_init()
        }
    }

    pub fn get_physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        unsafe {
            let mut res = null();
            vmaGetPhysicalDeviceProperties(self.vma, &mut res);
            &*(res as *const _)
        }
    }

    pub fn get_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut res = null();
            vmaGetMemoryProperties(self.vma, &mut res);
            &*(res as *const _)
        }
    }

    pub fn get_memory_type_properties(&self, type_index: u32) -> vk::MemoryPropertyFlags {
        unsafe {
            let mut res = Default::default();
            vmaGetMemoryTypeProperties(self.vma, type_index, &mut res);
            vk::MemoryPropertyFlags::from_raw(res)
        }
    }

    pub fn set_current_frame_index(&self, index: u32) {
        unsafe {
            vmaSetCurrentFrameIndex(self.vma, index);
        }
    }

    pub fn calculate_statistics(&self) -> TotalStatistics {
        unsafe {
            let mut res = MaybeUninit::uninit();
            vmaCalculateStatistics(self.vma, res.as_mut_ptr() as _);
            res.assume_init()
        }
    }

    pub fn get_heap_budgets(&self, budgets: &mut [Budget]) {
        assert!(budgets.len() >= self.get_memory_properties().memory_heap_count as usize);
        unsafe {
            vmaGetHeapBudgets(self.vma, budgets.as_mut_ptr() as _);
        }
    }

    pub fn find_memory_type_index(
        &self,
        memory_type_bits: u32,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<u32, vk::Result> {
        let mut res = 0;
        let err = unsafe {
            vmaFindMemoryTypeIndex(
                self.vma,
                memory_type_bits,
                alloc_info as *const _ as *const _,
                &mut res,
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(res)
        }
    }

    pub fn find_memory_type_index_for_buffer_info(
        &self,
        buffer_info: &vk::BufferCreateInfo,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<u32, vk::Result> {
        let mut res = 0;
        let err = unsafe {
            vmaFindMemoryTypeIndexForBufferInfo(
                self.vma,
                buffer_info as *const _ as *const _,
                alloc_info as *const _ as *const _,
                &mut res,
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(res)
        }
    }

    pub fn find_memory_type_index_for_image_info(
        &self,
        image_info: &vk::ImageCreateInfo,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<u32, vk::Result> {
        let mut res = 0;
        let err = unsafe {
            vmaFindMemoryTypeIndexForImageInfo(
                self.vma,
                image_info as *const _ as *const _,
                alloc_info as *const _ as *const _,
                &mut res,
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(res)
        }
    }

    pub fn create_pool(&self, create_info: &PoolCreateInfo) -> Result<Pool, vk::Result> {
        unsafe {
            let mut res = null_mut();
            let err = vmaCreatePool(self.vma, create_info as *const _ as *const _, &mut res);
            if err != VK_SUCCESS {
                Err(vk::Result::from_raw(err))
            } else {
                Ok(Pool(res))
            }
        }
    }

    pub fn destroy_pool(&self, pool: Pool) {
        unsafe {
            vmaDestroyPool(self.vma, pool.0);
        }
    }

    pub fn get_pool_statistics(&self, pool: Pool) -> Statistics {
        unsafe {
            let mut res = MaybeUninit::uninit();
            vmaGetPoolStatistics(self.vma, pool.0, res.as_mut_ptr() as _);
            res.assume_init()
        }
    }

    pub fn calculate_pool_statistics(&self, pool: Pool) -> DetailedStatistics {
        unsafe {
            let mut res = MaybeUninit::uninit();
            vmaCalculatePoolStatistics(self.vma, pool.0, res.as_mut_ptr() as _);
            res.assume_init()
        }
    }

    pub fn check_pool_corruption(&self, pool: Pool) -> Result<bool, vk::Result> {
        let res = unsafe { vmaCheckPoolCorruption(self.vma, pool.0) };
        match res {
            VK_SUCCESS => Ok(true),
            VK_ERROR_UNKNOWN => Ok(false),
            _ => Err(vk::Result::from_raw(res)),
        }
    }

    pub fn get_pool_name(&self, pool: Pool) -> Option<String> {
        let mut res = null();
        unsafe {
            vmaGetPoolName(self.vma, pool.0, &mut res);
            if res.is_null() {
                None
            } else {
                CStr::from_ptr(res).to_str().map(|n| n.to_string()).ok()
            }
        }
    }

    pub fn set_pool_name(&self, pool: Pool, name: Option<&str>) {
        unsafe {
            let name = name.map(|n| CString::new(n).unwrap());
            let name = name.map_or(null(), |n| n.as_ptr());

            vmaSetPoolName(self.vma, pool.0, name);
        }
    }

    pub fn allocate_memory(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        alloc_info: &AllocationCreateInfo,
    ) -> Result<Allocation, vk::Result> {
        let mut alloc = null_mut();
        let err = unsafe {
            vmaAllocateMemory(
                self.vma,
                memory_requirements as *const _ as *const _,
                alloc_info as *const _ as *const _,
                &mut alloc,
                null_mut(),
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(Allocation(alloc))
        }
    }

    pub fn allocate_memory_pages(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        alloc_info: &AllocationCreateInfo,
        allocation_count: usize,
    ) -> Result<Vec<Allocation>, vk::Result> {
        let mut res = vec![Allocation(null_mut()); allocation_count];
        let err = unsafe {
            vmaAllocateMemoryPages(
                self.vma,
                memory_requirements as *const _ as *const _,
                alloc_info as *const _ as *const _,
                allocation_count,
                res.as_mut_ptr() as _,
                null_mut(),
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(res)
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

    pub fn free_memory(&self, allocation: Allocation) {
        unsafe {
            vmaFreeMemory(self.vma, allocation.0);
        }
    }

    pub fn free_memory_pages(&self, allocations: &[Allocation]) {
        unsafe {
            vmaFreeMemoryPages(self.vma, allocations.len(), allocations.as_ptr() as _);
        }
    }

    pub fn get_allocation_info(&self, allocation: Allocation) -> AllocationInfo {
        let mut res = MaybeUninit::uninit();
        unsafe {
            vmaGetAllocationInfo(self.vma, allocation.0, res.as_mut_ptr() as _);
            res.assume_init()
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_allocation_user_data(&self, allocation: Allocation, data: *mut c_void) {
        unsafe {
            vmaSetAllocationUserData(self.vma, allocation.0, data);
        }
    }

    pub fn set_allocation_name(&self, allocation: Allocation, name: Option<&str>) {
        unsafe {
            let name = name.map(|n| CString::new(n).unwrap());
            let name = name.map_or(null(), |n| n.as_ptr());
            vmaSetAllocationName(self.vma, allocation.0, name);
        }
    }

    pub fn get_allocation_memory_properties(
        &self,
        allocation: Allocation,
    ) -> vk::MemoryPropertyFlags {
        let mut res = 0;
        unsafe {
            vmaGetAllocationMemoryProperties(self.vma, allocation.0, &mut res);
        }
        vk::MemoryPropertyFlags::from_raw(res)
    }

    pub fn map_memory(&self, allocation: Allocation) -> Result<*mut c_void, vk::Result> {
        let mut ptr = null_mut();
        let err = unsafe { vmaMapMemory(self.vma, allocation.0, &mut ptr) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(ptr)
        }
    }

    pub fn unmap_memory(&self, allocation: Allocation) {
        unsafe {
            vmaUnmapMemory(self.vma, allocation.0);
        }
    }

    pub fn flush_allocation(
        &self,
        allocation: Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> Result<(), vk::Result> {
        let err = unsafe { vmaFlushAllocation(self.vma, allocation.0, offset, size) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn invalidate_allocation(
        &self,
        allocation: Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> Result<(), vk::Result> {
        let err = unsafe { vmaInvalidateAllocation(self.vma, allocation.0, offset, size) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn flush_allocations(
        &self,
        allocations: &[Allocation],
        offsets: &[vk::DeviceSize],
        sizes: &[vk::DeviceSize],
    ) -> Result<(), vk::Result> {
        assert!(offsets.len() >= allocations.len());
        assert!(sizes.len() >= allocations.len());

        let err = unsafe {
            vmaFlushAllocations(
                self.vma,
                allocations.len() as u32,
                allocations.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.as_ptr() as _,
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn invalidate_allocations(
        &self,
        allocations: &[Allocation],
        offsets: &[vk::DeviceSize],
        sizes: &[vk::DeviceSize],
    ) -> Result<(), vk::Result> {
        assert!(offsets.len() >= allocations.len());
        assert!(sizes.len() >= allocations.len());

        let err = unsafe {
            vmaInvalidateAllocations(
                self.vma,
                allocations.len() as u32,
                allocations.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.as_ptr() as _,
            )
        };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn check_corruption(&self, memory_type_bits: u32) -> Result<bool, vk::Result> {
        let res = unsafe { vmaCheckCorruption(self.vma, memory_type_bits) };
        match res {
            VK_SUCCESS => Ok(true),
            VK_ERROR_UNKNOWN => Ok(false),
            _ => Err(vk::Result::from_raw(res)),
        }
    }

    pub fn begin_defragmentation(
        &self,
        info: &DefragmentationInfo,
    ) -> Result<DefragmentationContext, vk::Result> {
        let mut res = null_mut();
        let err =
            unsafe { vmaBeginDefragmentation(self.vma, info as *const _ as *const _, &mut res) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(DefragmentationContext(res))
        }
    }

    pub fn end_defragmentation(&self, context: DefragmentationContext) {
        unsafe {
            vmaEndDefragmentation(self.vma, context.0, null_mut());
        }
    }

    pub fn begin_defragmentation_pass(
        &self,
        context: DefragmentationContext,
    ) -> Vec<DefragmentationMoveOperation> {
        unsafe {
            let mut info = MaybeUninit::uninit();
            let res = vmaBeginDefragmentationPass(self.vma, context.0, info.as_mut_ptr());
            if res == VK_SUCCESS {
                Vec::new()
            } else {
                let info = info.assume_init();
                assert!(!info.pMoves.is_null());

                let slice = std::slice::from_raw_parts(info.pMoves as _, info.moveCount as usize);
                slice.to_vec()
            }
        }
    }

    pub fn end_defragmentation_pass(
        &self,
        context: DefragmentationContext,
        moves: &[DefragmentationMoveOperation],
    ) -> bool {
        unsafe {
            let mut info = VmaDefragmentationPassMoveInfo {
                moveCount: moves.len() as _,
                pMoves: moves.as_ptr() as _,
            };
            let res = vmaEndDefragmentationPass(self.vma, context.0, &mut info);
            res == VK_SUCCESS
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

    pub fn bind_buffer_memory2(
        &self,
        allocation: Allocation,
        local_offset: vk::DeviceSize,
        buffer: vk::Buffer,
    ) -> Result<(), vk::Result> {
        unsafe {
            let res = vmaBindBufferMemory2(
                self.vma,
                allocation.0,
                local_offset,
                buffer.as_raw() as _,
                null(),
            );
            if res != VK_SUCCESS {
                Err(vk::Result::from_raw(res))
            } else {
                Ok(())
            }
        }
    }

    pub fn bind_buffer_memory2_with_next(
        &self,
        allocation: Allocation,
        local_offset: vk::DeviceSize,
        buffer: vk::Buffer,
        next: &impl vk::ExtendsBindBufferMemoryInfo,
    ) -> Result<(), vk::Result> {
        unsafe {
            let res = vmaBindBufferMemory2(
                self.vma,
                allocation.0,
                local_offset,
                buffer.as_raw() as _,
                next as *const _ as *const _,
            );
            if res != VK_SUCCESS {
                Err(vk::Result::from_raw(res))
            } else {
                Ok(())
            }
        }
    }

    pub fn bind_image_memory(
        &self,
        image: vk::Image,
        allocation: Allocation,
    ) -> Result<(), vk::Result> {
        let err = unsafe { vmaBindImageMemory(self.vma, allocation.0, image.as_raw() as _) };
        if err != VK_SUCCESS {
            Err(vk::Result::from_raw(err))
        } else {
            Ok(())
        }
    }

    pub fn bind_image_memory2(
        &self,
        allocation: Allocation,
        local_offset: vk::DeviceSize,
        image: vk::Image,
    ) -> Result<(), vk::Result> {
        unsafe {
            let res = vmaBindImageMemory2(
                self.vma,
                allocation.0,
                local_offset,
                image.as_raw() as _,
                null(),
            );
            vk::Result::from_raw(res).result()
        }
    }

    pub fn bind_image_memory2_with_next(
        &self,
        allocation: Allocation,
        local_offset: vk::DeviceSize,
        image: vk::Image,
        next: &impl vk::ExtendsBindImageMemoryInfo,
    ) -> Result<(), vk::Result> {
        unsafe {
            let res = vmaBindImageMemory2(
                self.vma,
                allocation.0,
                local_offset,
                image.as_raw() as _,
                next as *const _ as *const _,
            );
            vk::Result::from_raw(res).result()
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

    pub fn create_buffer_with_alignment(
        &self,
        buffer_info: &vk::BufferCreateInfo,
        alloc_info: &AllocationCreateInfo,
        min_alignment: vk::DeviceSize,
    ) -> Result<(vk::Buffer, Allocation), vk::Result> {
        let mut buffer = null_mut();
        let mut allocation = null_mut();
        let err = unsafe {
            vmaCreateBufferWithAlignment(
                self.vma,
                buffer_info as *const _ as _,
                alloc_info as *const _ as *const _,
                min_alignment,
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

    pub fn create_aliasing_buffer(
        &self,
        allocation: Allocation,
        buffer_info: &vk::BufferCreateInfo,
    ) -> Result<vk::Buffer, vk::Result> {
        let mut buffer = null_mut();
        let res = unsafe {
            vmaCreateAliasingBuffer(
                self.vma,
                allocation.0,
                buffer_info as *const _ as *const _,
                &mut buffer,
            )
        };
        if res != VK_SUCCESS {
            Err(vk::Result::from_raw(res))
        } else {
            Ok(vk::Buffer::from_raw(buffer as _))
        }
    }

    pub fn create_aliasing_buffer2(
        &self,
        allocation: Allocation,
        local_offset: vk::DeviceSize,
        buffer_info: &vk::BufferCreateInfo,
    ) -> Result<vk::Buffer, vk::Result> {
        let mut buffer = null_mut();
        let res = unsafe {
            vmaCreateAliasingBuffer2(
                self.vma,
                allocation.0,
                local_offset,
                buffer_info as *const _ as *const _,
                &mut buffer,
            )
        };
        if res != VK_SUCCESS {
            Err(vk::Result::from_raw(res))
        } else {
            Ok(vk::Buffer::from_raw(buffer as _))
        }
    }

    pub fn destroy_buffer(&self, buffer: vk::Buffer, alloc: Allocation) {
        unsafe {
            vmaDestroyBuffer(self.vma, buffer.as_raw() as _, alloc.0);
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

    pub fn create_aliasing_image(
        &self,
        allocation: Allocation,
        image_info: &vk::ImageCreateInfo,
    ) -> Result<vk::Image, vk::Result> {
        let mut image = null_mut();
        let res = unsafe {
            vmaCreateAliasingImage(
                self.vma,
                allocation.0,
                image_info as *const _ as *const _,
                &mut image,
            )
        };
        if res != VK_SUCCESS {
            Err(vk::Result::from_raw(res))
        } else {
            Ok(vk::Image::from_raw(image as _))
        }
    }

    pub fn create_aliasing_image2(
        &self,
        allocation: Allocation,
        local_offset: vk::DeviceSize,
        image_info: &vk::ImageCreateInfo,
    ) -> Result<vk::Image, vk::Result> {
        let mut image = null_mut();
        let res = unsafe {
            vmaCreateAliasingImage2(
                self.vma,
                allocation.0,
                local_offset,
                image_info as *const _ as *const _,
                &mut image,
            )
        };
        if res != VK_SUCCESS {
            Err(vk::Result::from_raw(res))
        } else {
            Ok(vk::Image::from_raw(image as _))
        }
    }

    pub fn destroy_image(&self, image: vk::Image, alloc: Allocation) {
        unsafe {
            vmaDestroyImage(self.vma, image.as_raw() as _, alloc.0);
        }
    }

    pub fn build_stats_string(&self, detailed_map: bool) -> String {
        unsafe {
            let mut ptr = null_mut();
            vmaBuildStatsString(self.vma, &mut ptr, if detailed_map { 1 } else { 0 });
            let res = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
            vmaFreeStatsString(self.vma, ptr);
            res
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
