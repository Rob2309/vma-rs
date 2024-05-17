use crate::ffi;

macro_rules! handle {
    ($name:ident, $ffi:ty) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($ffi);

        unsafe impl Send for $name { }
        unsafe impl Sync for $name { }

        impl $name {
            pub const fn from_raw(raw: $ffi) -> Self {
                Self(raw)
            }
            pub const fn into_raw(self) -> $ffi {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(unsafe { ::std::mem::zeroed() })
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                write!(f, concat!(stringify!($name), "({:#08p})"), self.0)
            }
        }
        impl ::std::fmt::Pointer for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{:#08p}", self.0)
            }
        }
    };
}

handle!(Allocator, ffi::VmaAllocator);
handle!(Pool, ffi::VmaPool);
handle!(Allocation, ffi::VmaAllocation);
handle!(DefragmentationContext, ffi::VmaDefragmentationContext);
handle!(VirtualAllocation, ffi::VmaVirtualAllocation);
handle!(VirtualBlock, ffi::VmaVirtualBlock);
