//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!

/// This enum provides every icon as a variant.
/// It implements [`Into<icondata_core::IconData>`][icondata_core::IconData].

{% for (feat, svg) in features_svgs.iter() -%}
pub const {{ feat|shouty_snake_case }}: icondata_core::IconData = icondata_core::IconData {
    {% let attributes = svg.svg_attributes() -%}
    style: {{ attributes.style|attribute_value }},
    x: {{ attributes.x|attribute_value }},
    y: {{ attributes.y|attribute_value }},
    width: {{ attributes.width|attribute_value }},
    height: {{ attributes.height|attribute_value }},
    view_box: {{ attributes.view_box|attribute_value }},
    stroke_linecap: {{ attributes.stroke_linecap|attribute_value }},
    stroke_linejoin: {{ attributes.stroke_linejoin|attribute_value }},
    stroke_width: {{ attributes.stroke_width|attribute_value }},
    stroke: {{ attributes.stroke|attribute_value }},
    fill: {{ attributes.fill|attribute_value }},
    data: r###"{{ svg.content.as_str() }}"###
};
{% endfor %}
