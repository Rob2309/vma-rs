use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

use ash::vk;
use vma::{
    AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage, Pool, PoolCreateFlags,
    PoolCreateInfo, VirtualAllocationCreateFlags, VirtualAllocationCreateInfo, VirtualBlock,
    VirtualBlockCreateFlags, VirtualBlockCreateInfo,
};

struct TestContext {
    vma: Allocator,
    device: ash::Device,
    instance: ash::Instance,
    #[allow(unused)]
    entry: ash::Entry,

    physical_device: vk::PhysicalDevice,
}

fn create_allocator() -> TestContext {
    let entry = unsafe { ash::Entry::load().expect("Failed to load ash") };
    let instance = unsafe {
        let name = CString::new("vma-tests").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_3)
            .application_name(&name)
            .application_version(vk::make_api_version(0, 0, 0, 1))
            .engine_name(&name)
            .engine_version(vk::make_api_version(0, 0, 0, 1));
        let create_info = vk::InstanceCreateInfo::builder().application_info(&app_info);

        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance")
    };
    let (device, physical_device) = unsafe {
        let physical_device = instance.enumerate_physical_devices().unwrap()[0];

        let prio = [1.0];
        let queue_infos = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&prio)
            .build()];

        let create_info = vk::DeviceCreateInfo::builder().queue_create_infos(&queue_infos);

        let device = instance
            .create_device(physical_device, &create_info, None)
            .expect("Failed to create Device");
        (device, physical_device)
    };

    let vma = Allocator::new(physical_device, &device, &instance, &entry)
        .expect("Failed to create Allocator");

    TestContext {
        entry,
        instance,
        device,
        physical_device,
        vma,
    }
}

#[test]
fn get_info() {
    let cx = create_allocator();

    let info = cx.vma.get_info();
    assert_eq!(cx.instance.handle(), info.instance);
    assert_eq!(cx.device.handle(), info.device);
    assert_eq!(cx.physical_device, info.physical_device);

    let props_a = cx.vma.get_physical_device_properties();
    let props_b = unsafe {
        cx.instance
            .get_physical_device_properties(cx.physical_device)
    };
    assert_eq!(props_a.api_version, props_b.api_version);
    assert_eq!(props_a.driver_version, props_b.driver_version);
    assert_eq!(props_a.vendor_id, props_b.vendor_id);
    assert_eq!(props_a.device_id, props_b.device_id);
    assert_eq!(props_a.device_type, props_b.device_type);
    assert_eq!(props_a.pipeline_cache_uuid, props_b.pipeline_cache_uuid);

    let props_a = cx.vma.get_memory_properties();
    let props_b = unsafe {
        cx.instance
            .get_physical_device_memory_properties(cx.physical_device)
    };
    assert_eq!(props_a.memory_type_count, props_b.memory_type_count);
    assert_eq!(props_a.memory_heap_count, props_b.memory_heap_count);
    for i in 0..props_a.memory_type_count as usize {
        assert_eq!(
            props_a.memory_types[i].heap_index,
            props_b.memory_types[i].heap_index
        );
        assert_eq!(
            props_a.memory_types[i].property_flags,
            props_b.memory_types[i].property_flags
        );

        assert_eq!(
            props_a.memory_types[i].property_flags,
            cx.vma.get_memory_type_properties(i as u32)
        );
    }
    for i in 0..props_a.memory_heap_count as usize {
        assert_eq!(props_a.memory_heaps[i].size, props_b.memory_heaps[i].size);
        assert_eq!(props_a.memory_heaps[i].flags, props_b.memory_heaps[i].flags);
    }
}

#[test]
fn create_buffer() {
    let cx = create_allocator();

    let buffer_info = vk::BufferCreateInfo::builder()
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .size(128)
        .usage(vk::BufferUsageFlags::TRANSFER_DST);
    let alloc_info = AllocationCreateInfo {
        flags: AllocationCreateFlags::empty(),
        usage: MemoryUsage::GPU_ONLY,
        required_flags: vk::MemoryPropertyFlags::empty(),
        preferred_flags: vk::MemoryPropertyFlags::empty(),
        memory_type_bits: 0,
        pool: Pool::null(),
        user_data: null_mut(),
        priority: 1.0,
    };

    let (buffer, alloc) = cx
        .vma
        .create_buffer(&buffer_info, &alloc_info)
        .expect("Failed to create buffer");

    cx.vma.destroy_buffer(buffer, alloc);
}

#[test]
fn memory_pools() {
    let cx = create_allocator();

    let buffer_info = vk::BufferCreateInfo::builder()
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .size(128)
        .usage(vk::BufferUsageFlags::TRANSFER_DST);

    let mut alloc_info = AllocationCreateInfo {
        flags: AllocationCreateFlags::empty(),
        usage: MemoryUsage::GPU_ONLY,
        required_flags: vk::MemoryPropertyFlags::empty(),
        preferred_flags: vk::MemoryPropertyFlags::empty(),
        memory_type_bits: 0,
        pool: Pool::null(),
        user_data: null_mut(),
        priority: 1.0,
    };

    let memory_type_index = cx
        .vma
        .find_memory_type_index_for_buffer_info(&buffer_info, &alloc_info)
        .expect("No memory type for buffer");

    let pool_info = PoolCreateInfo {
        memory_type_index,
        flags: PoolCreateFlags::empty(),
        block_size: 0,
        min_block_count: 0,
        max_block_count: 0,
        priority: 1.0,
        min_allocation_alignment: 0,
        memory_allocate_next: null_mut(),
    };

    let pool = cx
        .vma
        .create_pool(&pool_info)
        .expect("Failed to create pool");

    alloc_info.pool = pool;
    let (buffer, alloc) = cx
        .vma
        .create_buffer(&buffer_info, &alloc_info)
        .expect("Failed to create buffer in pool");

    cx.vma.set_pool_name(pool, Some("TestPool"));
    assert_eq!(cx.vma.get_pool_name(pool), Some("TestPool".to_owned()));

    cx.vma.destroy_buffer(buffer, alloc);
    cx.vma.destroy_pool(pool);
}

#[test]
fn virtual_blocks() {
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

    assert!(!block.is_empty());

    let info = block.get_allocation_info(allocation);
    assert_eq!(info.user_data, 42 as *mut c_void);
    assert_eq!(info.offset, offset);

    block.free(allocation);

    assert!(block.is_empty());
}
