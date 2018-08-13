// workarounds
#![allow(unused_doc_comments)]

// language extensions
#![feature(box_syntax, plugin, allocator_api, alloc, rust_2018_preview)]
#![cfg_attr(feature = "assignment_operators", feature(augmented_assignments, op_assign_traits))]

// crates
extern crate alloc;
extern crate bytes;
extern crate num_rational;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;

extern crate byte_slice_cast;

pub mod rational {
    pub use num_rational::*;
}

pub mod audiosample;
pub mod frame;
pub mod packet;
pub mod params;
pub mod pixel;
pub mod timeinfo;
pub mod value;
