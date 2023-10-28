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

{% for short_name in short_names %}
pub use icondata_{{short_name}}::*;
{%- endfor %}
