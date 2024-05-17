use ash::vk;
use ash_mem_alloc::vma::{self};
use std::ffi::c_void;

fn main() {
    let entry = unsafe { ash::Entry::load().unwrap() };

    let instance = {
        let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
        let info = vk::InstanceCreateInfo::default().application_info(&app_info);
        unsafe { entry.create_instance(&info, None).unwrap() }
    };

    let physical_device = unsafe { instance.enumerate_physical_devices().unwrap()[0] };

    let device = {
        let prio = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(0)
            .queue_priorities(&prio)];
        let info = vk::DeviceCreateInfo::default().queue_create_infos(&queues);
        unsafe {
            instance
                .create_device(physical_device, &info, None)
                .unwrap()
        }
    };

    let allocator = {
        let callbacks = vma::DeviceMemoryCallbacks::default()
            .allocate(Some(allocate_cb))
            .free(Some(free_cb));
        let functions = vma::VulkanFunctions::default()
            .get_instance_proc_addr(Some(entry.static_fn().get_instance_proc_addr))
            .get_device_proc_addr(Some(instance.fp_v1_0().get_device_proc_addr));
        let info = vma::AllocatorCreateInfo::default()
            .device(device.handle())
            .instance(instance.handle())
            .physical_device(physical_device)
            .vulkan_functions(&functions)
            .device_memory_callbacks(&callbacks);
        unsafe { vma::create_allocator(&info).unwrap() }
    };

    let (buffer, alloc, _info) = {
        let buffer_info = vk::BufferCreateInfo::default()
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .size(1024)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER);
        let alloc_info = vma::AllocationCreateInfo::default().usage(vma::MemoryUsage::AUTO);

        unsafe { vma::create_buffer(allocator, &buffer_info, &alloc_info).unwrap() }
    };

    unsafe {
        vma::set_allocation_name(allocator, alloc, Some(c"Test Allocation"));
    }

    let info = unsafe { vma::get_allocation_info(allocator, alloc) };

    println!(
        "Allocation has name: {}",
        info.get_name().unwrap().to_str().unwrap()
    );

    unsafe {
        vma::destroy_buffer(allocator, buffer, alloc);
        vma::destroy_allocator(allocator);
    }
}

extern "system" fn allocate_cb(
    allocator: vma::Allocator,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: vk::DeviceSize,
    user_data: *mut c_void,
) {
    println!("Allocating: allocator={allocator}, memory_type={memory_type}, memory={memory:p}, size={size}, user_data:{user_data:p}");
}

extern "system" fn free_cb(
    allocator: vma::Allocator,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: vk::DeviceSize,
    user_data: *mut c_void,
) {
    println!("Freeing: allocator={allocator}, memory_type={memory_type}, memory={memory:p}, size={size}, user_data:{user_data:p}");
}
