macro_rules! handle {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name(u64);

        impl ::std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                write!(f, concat!(stringify!($name), "(0x{:08x})"), self.0)
            }
        }
        impl ::std::fmt::Pointer for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "0x{:08x}", self.0)
            }
        }
    };
}

handle!(Allocator);
handle!(Pool);
handle!(Allocation);
handle!(DefragmentationContext);
handle!(VirtualAllocation);
handle!(VirtualBlock);
