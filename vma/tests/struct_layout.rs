use std::{mem::{MaybeUninit, size_of, align_of}, ptr::addr_of};

use vma::AllocationCreateInfo;
use vma_sys::VmaAllocationCreateInfo;

macro_rules! check_layout {
    ($struct_a:ident == $struct_b:ident {
        $($field_a:ident == $field_b:ident),* $(,)?
    }) => {
        let uninit_a = MaybeUninit::<$struct_a>::uninit();
        let uninit_b = MaybeUninit::<$struct_b>::uninit();

        let ptr_a = uninit_a.as_ptr();
        let ptr_b = uninit_b.as_ptr();

        assert_eq!(size_of::<$struct_a>(), size_of::<$struct_b>());
        assert_eq!(align_of::<$struct_a>(), align_of::<$struct_b>());

        unsafe {
            $(
                assert_eq!(
                    addr_of!((*ptr_a).$field_a) as usize - ptr_a as usize,
                    addr_of!((*ptr_b).$field_b) as usize - ptr_b as usize,
                );
            )*
        }
    };
}

#[test]
fn allocation_create_info_layout() {
    check_layout!(
        AllocationCreateInfo == VmaAllocationCreateInfo {
            flags == flags,
            usage == usage,
            required_flags == requiredFlags,
            preferred_flags == preferredFlags,
            memory_type_bits == memoryTypeBits,
            pool == pool,
            user_data == pUserData,
            priority == priority,
        }
    );
}