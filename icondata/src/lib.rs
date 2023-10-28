//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them. It re-exports all icons from the icondata_* crates.
//!
//! This crate is meant to be used to build component libraries for web frameworks.
//! To do so, an [`Icon`] enum is provided, which can be used to select any icon from the
//! collection. This enum is marked as non_exhaustive, as new icon libraries may be added in the future.
//!
//! The [`Icon`] enum can be converted into an [`IconData`] struct, which contains the SVG data.
//!
//!

pub use icondata_core::IconData;


pub use icondata_ai::*;
pub use icondata_bi::*;
pub use icondata_bs::*;
pub use icondata_cg::*;
pub use icondata_ch::*;
pub use icondata_fa::*;
pub use icondata_fi::*;
pub use icondata_hi::*;
pub use icondata_im::*;
pub use icondata_io::*;
pub use icondata_lu::*;
pub use icondata_oc::*;
pub use icondata_ri::*;
pub use icondata_si::*;
pub use icondata_tb::*;
pub use icondata_ti::*;
pub use icondata_vs::*;
pub use icondata_wi::*;