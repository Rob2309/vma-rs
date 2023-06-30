#![doc = include_str!("../../README.md")]

#[allow(clippy::missing_safety_doc)]
mod bindings;
mod handles;
mod function_ptrs;

pub mod vma {
    pub use super::function_ptrs::*;
    pub use super::handles::*;
    pub use super::bindings::*;
}
