use ash::vk;
use vma::AllocatorCreateFlags;



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
            instance.create_device(instance.enumerate_physical_devices().unwrap()[0], &info, None).unwrap()
        }
    };

    let allocator = {
        let functions = vma::VulkanFunctions::builder()
            .vk_get_instance_proc_addr(Some(entry.static_fn().get_instance_proc_addr))
            .vk_get_device_proc_addr(Some(instance.fp_v1_0().get_device_proc_addr));
        let info = vma::AllocatorCreateInfo::builder()
            .device(device.handle())
            .instance(instance.handle())
            .p_vulkan_functions(&functions);
        unsafe {
            vma::create_allocator(&info).unwrap()
        }
    };
}