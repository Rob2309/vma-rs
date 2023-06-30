use std::ffi::c_void;

use ash::vk;

fn main() {
    let entry = unsafe { ash::Entry::load().unwrap() };

    let instance = {
        let app_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
        let info = vk::InstanceCreateInfo::builder().application_info(&app_info);
        unsafe { entry.create_instance(&info, None).unwrap() }
    };

    let physical_device = unsafe { instance.enumerate_physical_devices().unwrap()[0] };

    let device = {
        let prio = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&prio)
            .build()];
        let info = vk::DeviceCreateInfo::builder().queue_create_infos(&queues);
        unsafe {
            instance
                .create_device(physical_device, &info, None)
                .unwrap()
        }
    };

    let allocator = {
        let callbacks = vma::DeviceMemoryCallbacks::builder()
            .allocate(Some(allocate_cb))
            .free(Some(free_cb));
        let functions = vma::VulkanFunctions::builder()
            .get_instance_proc_addr(Some(entry.static_fn().get_instance_proc_addr))
            .get_device_proc_addr(Some(instance.fp_v1_0().get_device_proc_addr));
        let info = vma::AllocatorCreateInfo::builder()
            .device(device.handle())
            .instance(instance.handle())
            .physical_device(physical_device)
            .vulkan_functions(&functions)
            .device_memory_callbacks(&callbacks);
        unsafe { vma::create_allocator(&info).unwrap() }
    };

    let (buffer, alloc, _info) = {
        let buffer_info = vk::BufferCreateInfo::builder()
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .size(1024)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER);
        let alloc_info = vma::AllocationCreateInfo::builder().usage(vma::MemoryUsage::AUTO);

        unsafe { vma::create_buffer(allocator, &buffer_info, &alloc_info).unwrap() }
    };

    unsafe {
        vma::destroy_buffer(allocator, buffer, alloc);
        vma::destroy_allocator(allocator);
    }
}

extern "system" fn allocate_cb(allocator: vma::Allocator, memory_type: u32, memory: vk::DeviceMemory, size: vk::DeviceSize, user_data: *mut c_void) {
    println!("Allocating: allocator={allocator}, memory_type={memory_type}, memory={memory:p}, size={size}, user_data:{user_data:p}");
}

extern "system" fn free_cb(allocator: vma::Allocator, memory_type: u32, memory: vk::DeviceMemory, size: vk::DeviceSize, user_data: *mut c_void) {
    println!("Freeing: allocator={allocator}, memory_type={memory_type}, memory={memory:p}, size={size}, user_data:{user_data:p}");
}
