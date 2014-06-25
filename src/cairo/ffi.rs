use libc::{c_int, c_uint, c_char, c_double};
use cairo::types::{
	cairo_t,
	cairo_surface_t,
	cairo_pattern_t,
	cairo_operator_t,
	cairo_rectangle_list_t,
	cairo_content_t,
	cairo_path_t,
	cairo_glyph_t,
	cairo_bool_t
};
use cairo::enums::{Status, status, Antialias, LineCap, LineJoin, FillRule};

#[link(name = "cairo")]
extern "C" {
	pub fn cairo_create (target: *cairo_surface_t) -> *cairo_t;

	pub fn cairo_reference (cr: *cairo_t) -> *cairo_t;

	pub fn cairo_destroy (cr: *cairo_t);

	pub fn cairo_status (cr: *cairo_t) -> Status;

	pub fn cairo_save (cr: *cairo_t);

	pub fn cairo_restore (cr: *cairo_t);

	pub fn cairo_get_target (cr: *cairo_t) -> *cairo_surface_t;

	pub fn cairo_push_group (cr: *cairo_t);

	pub fn cairo_push_group_with_content (cr: *cairo_t, content: cairo_content_t);

	pub fn cairo_pop_group (cr: *cairo_t) -> *cairo_pattern_t;

	pub fn cairo_pop_group_to_source (cr: *cairo_t);

	pub fn cairo_get_group_target (cr: *cairo_t) -> *cairo_surface_t;

	pub fn cairo_set_source_rgb (cr: *cairo_t, red: c_double, green: c_double, blue: c_double);

	pub fn cairo_set_source_rgba (cr: *cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);

	pub fn cairo_set_source (cr: *cairo_t, source: *cairo_pattern_t);

	pub fn cairo_set_source_surface (cr: *cairo_t, surface: *cairo_surface_t, x: c_double, y: c_double);

	pub fn cairo_get_source (cr: *cairo_t) -> *cairo_pattern_t;

	pub fn cairo_set_antialias (cr: *cairo_t, antialias: Antialias);

	pub fn cairo_get_antialias (cr: *cairo_t) -> Antialias;

	pub fn cairo_set_dash (cr: *cairo_t, dashes : *c_double, num_dashes: c_int, offset: c_double);

	pub fn cairo_get_dash_count (cr: *cairo_t) -> c_int;

	pub fn cairo_get_dash (cr: *cairo_t, dashes: *c_double, offset: *c_double);

	pub fn cairo_set_fill_rule (cr: *cairo_t, fill_rule: FillRule);

	pub fn cairo_get_fill_rule (cr: *cairo_t) -> FillRule;

	pub fn cairo_set_line_cap (cr: *cairo_t, line_cap: LineCap);

	pub fn cairo_get_line_cap (cr: *cairo_t) -> LineCap;

	pub fn cairo_set_line_join (cr: *cairo_t, line_join: LineJoin);

	pub fn cairo_get_line_join (cr: *cairo_t) -> LineJoin;

	pub fn cairo_set_line_width (cr: *cairo_t, width: c_double);

	pub fn cairo_get_line_width (cr: *cairo_t) -> c_double;

	pub fn cairo_set_miter_limit (cr: *cairo_t, limit: c_double);

	pub fn cairo_get_miter_limit (cr: *cairo_t) -> c_double;

	pub fn cairo_set_operator (cr: *cairo_t, op: cairo_operator_t);

	pub fn cairo_get_operator (cr: *cairo_t) -> cairo_operator_t;

	pub fn cairo_set_tolerance (cr: *cairo_t, tolerance: c_double);

	pub fn cairo_get_tolerance (cr: *cairo_t) -> c_double;

	pub fn cairo_clip (cr: *cairo_t);

	pub fn cairo_clip_preserve (cr: *cairo_t);

	pub fn cairo_clip_extents (cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);

	pub fn cairo_in_clip (cr: *cairo_t, x: c_double, y: c_double) -> cairo_bool_t;

	pub fn cairo_reset_clip (cr: *cairo_t);

	pub fn cairo_rectangle_list_destroy (rectangle_list: *cairo_rectangle_list_t);

	pub fn cairo_copy_clip_rectangle_list (cr: *cairo_t) -> *cairo_rectangle_list_t;

	pub fn cairo_fill (cr: *cairo_t);

	pub fn cairo_fill_preserve (cr: *cairo_t);

	pub fn cairo_fill_extents (cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);

	pub fn cairo_in_fill (cr: *cairo_t, x: c_double, y: c_double) -> cairo_bool_t;

	pub fn cairo_mask (cr: *cairo_t, pattern: *cairo_pattern_t);

	pub fn cairo_mask_surface (cr: *cairo_t, surface: *cairo_surface_t, surface_x: c_double, surface_y: c_double);

	pub fn cairo_paint (cr: *cairo_t);

	pub fn cairo_paint_with_alpha (cr: *cairo_t, alpha: c_double);

	pub fn cairo_stroke (cr: *cairo_t);

	pub fn cairo_stroke_preserve (cr: *cairo_t);

	pub fn cairo_stroke_extents (cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);

	pub fn cairo_in_stroke (cr: *cairo_t, x: c_double, y: c_double) -> cairo_bool_t;

	pub fn cairo_copy_page (cr: *cairo_t);

	pub fn cairo_show_page (cr: *cairo_t);

	pub fn cairo_get_reference_count (cr: *cairo_t) -> c_uint;

	//Error handling
	pub fn cairo_status_to_string (status : Status) -> *c_char;

	//Paths
	pub fn cairo_copy_path(cr: *cairo_t) -> *cairo_path_t;

	pub fn cairo_copy_path_flat(cr: *cairo_t) -> *cairo_path_t;

	pub fn cairo_path_destroy(path: *cairo_path_t);

	pub fn cairo_append_path(cr: *cairo_t, path: *cairo_path_t);

	pub fn cairo_has_current_point(cr: *cairo_t) -> cairo_bool_t;

	pub fn cairo_get_current_point(cr: *cairo_t, x: *c_double, y: *c_double);

	pub fn cairo_new_path(cr: *cairo_t);

	pub fn cairo_new_sub_path(cr: *cairo_t);

	pub fn cairo_close_path(cr: *cairo_t);

	pub fn cairo_arc(cr: *cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);

	pub fn cairo_arc_negative(cr: *cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);

	pub fn cairo_curve_to(cr: *cairo_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);

	pub fn cairo_line_to(cr: *cairo_t, x: c_double, y: c_double);

	pub fn cairo_move_to(cr: *cairo_t, x: c_double, y: c_double);

	pub fn cairo_rectangle(cr: *cairo_t, x: c_double, y: c_double, width: c_double, height: c_double);

	pub fn cairo_glyph_path(cr: *cairo_t, glyphs: *cairo_glyph_t, num_glyphs: c_int);

	pub fn cairo_text_path(cr: *cairo_t, utf8: *c_char);

	pub fn cairo_rel_curve_to(cr: *cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double);

	pub fn cairo_rel_line_to(cr: *cairo_t, dx: c_double, dy: c_double);

	pub fn cairo_rel_move_to(cr: *cairo_t, dx: c_double, dy: c_double);

	pub fn cairo_path_extents(cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);
}