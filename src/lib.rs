//! Ethereum Virtual Machine implementation in Rust

#![deny(warnings)]
#![forbid(unsafe_code, unused_variables)]

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use evm_core::*;
pub use evm_runtime::*;
pub use evm_gasometer as gasometer;

pub mod executor;
pub mod backend;
pub mod tracing;
