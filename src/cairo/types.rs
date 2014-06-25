#![allow(non_camel_case_types)]

use cairo;
use libc::{c_int, c_double};
use cairo::enums::PathDataType;

pub struct cairo_t;
pub struct cairo_surface_t;
pub struct cairo_pattern_t;
pub struct cairo_fill_rule_t;
pub struct cairo_antialias_t;
pub struct cairo_destroy_func_t;
pub struct cairo_line_join_t;
pub struct cairo_line_cap_t;
pub struct cairo_operator_t;
pub struct cairo_rectangle_list_t;
pub struct cairo_rectangle_int_t;
pub struct cairo_content_t;
pub struct cairo_path_t{
	pub status: cairo::Status,
    pub data: *mut (c_double, c_double),
    pub num_data: c_int
}
pub struct cairo_path_data_header{
    pub data_type: PathDataType,
    pub length:    c_int
}
pub struct cairo_glyph_t;

pub struct cairo_bool_t{
	value: c_int
}

impl cairo_bool_t{
	pub fn as_bool(&self) -> bool{
		self.value != 0
	}
}
pub struct cairo_region_t;
pub struct cairo_font_face_t;
pub struct cairo_scaled_font_t;
pub struct cairo_font_options_t;
pub struct cairo_matrix_t;

pub struct cairo_font_extents_t{
    ascent: c_double,
    descent: c_double,
    height: c_double,
    max_x_advance: c_double,
    max_y_advance: c_double,
}

pub struct cairo_extend_t;
pub struct cairo_text_extents_t;
pub struct cairo_filter_t;
pub struct cairo_region_overlap_t;
pub struct cairo_pattern_type_t;