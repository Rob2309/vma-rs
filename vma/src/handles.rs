macro_rules! handle {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name(u64);
    };
}

handle!(Allocator);
handle!(Pool);
handle!(Allocation);
handle!(DefragmentationContext);
handle!(VirtualAllocation);
handle!(VirtualBlock);
