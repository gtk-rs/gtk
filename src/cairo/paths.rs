
use libc::{c_int, c_uint, c_char, c_double};
use std::mem::transmute;

use cairo::types::{
   cairo_t,
   cairo_surface_t,
   cairo_pattern_t,
   cairo_operator_t,
   cairo_rectangle_list_t,
   cairo_content_t,
   cairo_path_t,
   cairo_glyph_t
};
use cairo::enums::{Status, status, Antialias, LineCap, LineJoin, FillRule};
use cairo::Context;
