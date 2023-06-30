use ash::vk;

fn main() {
    let entry = unsafe { ash::Entry::load().unwrap() };
    
    let instance = {
        let app_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_3);
        let info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info);
        unsafe {
            entry.create_instance(&info, None).unwrap()
        }
    };

    let physical_device = unsafe{instance.enumerate_physical_devices().unwrap()[0]};

    let device = {
        let prio = [1.0];
        let queues = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(0)
                .queue_priorities(&prio)
                .build()
        ];
        let info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queues);
        unsafe {
            instance.create_device(physical_device, &info, None).unwrap()
        }
    };

    let allocator = {
        let functions = vma::VulkanFunctions::builder()
            .vk_get_instance_proc_addr(Some(entry.static_fn().get_instance_proc_addr))
            .vk_get_device_proc_addr(Some(instance.fp_v1_0().get_device_proc_addr));
        let info = vma::AllocatorCreateInfo::builder()
            .device(device.handle())
            .instance(instance.handle())
            .physical_device(physical_device)
            .p_vulkan_functions(&functions);
        unsafe {
            vma::create_allocator(&info).unwrap()
        }
    };

    let (buffer, alloc, _info) = {
        let buffer_info = vk::BufferCreateInfo::builder()
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .size(1024)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER);
        let alloc_info = vma::AllocationCreateInfo::builder()
            .usage(vma::MemoryUsage::VMA_MEMORY_USAGE_AUTO);

        unsafe {
            vma::create_buffer(allocator, &buffer_info, &alloc_info).unwrap()
        }
    };

    unsafe {
        vma::destroy_buffer(allocator, buffer, alloc);
        vma::destroy_allocator(allocator);
    }
}