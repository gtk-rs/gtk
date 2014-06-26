#![allow(non_camel_case_types)]

use cairo::context::Rectangle;
use libc::{c_int, c_uint, c_char, c_double, c_ulong};
use cairo::enums::{
    Status,
    Antialias,
    LineCap,
    LineJoin,
    FillRule,
    FontSlant,
    FontWeight,
    TextClusterFlags,
    FontType,
    SubpixelOrder,
    HintStyle,
    HintMetrics,
    Extend,
    Filter,
    PathDataType,
    PatternType
};
use cairo::fonts::{
    FontExtents,
    Glyph,
    TextCluster,
    TextExtents
};
use cairo::matrices::Matrix;
use cairo;

pub struct cairo_t;
pub struct cairo_surface_t;
pub struct cairo_pattern_t;
pub struct cairo_fill_rule_t;
pub struct cairo_antialias_t;
pub struct cairo_destroy_func_t;
pub struct cairo_line_join_t;
pub struct cairo_line_cap_t;
pub struct cairo_operator_t;
pub struct cairo_rectangle_list_t{
    pub status: Status,
    pub rectangles: *mut Rectangle,
    pub num_rectangles: c_int
}
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
pub struct cairo_extend_t;
pub struct cairo_filter_t;
pub struct cairo_region_overlap_t;

#[link(name = "cairo")]
extern "C" {

    //CAIRO CONTEXT
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


    //CAIRO UTILS: Error handling

    pub fn cairo_status_to_string (status : Status) -> *c_char;


    //CAIRO PATHS

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

    pub fn cairo_glyph_path(cr: *cairo_t, glyphs: *Glyph, num_glyphs: c_int);

    pub fn cairo_text_path(cr: *cairo_t, utf8: *c_char);

    pub fn cairo_rel_curve_to(cr: *cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double);

    pub fn cairo_rel_line_to(cr: *cairo_t, dx: c_double, dy: c_double);

    pub fn cairo_rel_move_to(cr: *cairo_t, dx: c_double, dy: c_double);

    pub fn cairo_path_extents(cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);


    //CAIRO TRANSFORMATIONS

    pub fn cairo_translate(cr: *cairo_t, tx: c_double, ty: c_double);

    pub fn cairo_scale(cr: *cairo_t, sx: c_double, sy: c_double);

    pub fn cairo_rotate(cr: *cairo_t, angle: c_double);

    pub fn cairo_transform(cr: *cairo_t, matrix: *Matrix);

    pub fn cairo_set_matrix(cr: *cairo_t, matrix: *Matrix);

    pub fn cairo_get_matrix(cr: *cairo_t, matrix: *Matrix);

    pub fn cairo_identity_matrix(cr: *cairo_t);

    pub fn cairo_user_to_device(cr: *cairo_t, x: *c_double, y: *c_double);

    pub fn cairo_user_to_device_distance(cr: *cairo_t, dx: *c_double, dy: *c_double);

    pub fn cairo_device_to_user(cr: *cairo_t, x: *c_double, y: *c_double);

    pub fn cairo_device_to_user_distance(cr: *cairo_t, dx: *c_double, dy: *c_double);


    //CAIRO PATTERNS

    pub fn cairo_pattern_add_color_stop_rgb(pattern: *cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double);

    pub fn cairo_pattern_add_color_stop_rgba(pattern: *cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double, alpha: c_double);

    pub fn cairo_pattern_get_color_stop_count(pattern: *cairo_pattern_t, count: *c_int) -> Status;

    pub fn cairo_pattern_get_color_stop_rgba(pattern: *cairo_pattern_t, index: c_int, offset: *c_double, red: *c_double, green: *c_double, blue: *c_double, alpha: *c_double) -> Status;

    pub fn cairo_pattern_create_rgb(red: c_double, green: c_double, blue: c_double) -> *cairo_pattern_t;

    pub fn cairo_pattern_create_rgba(red: c_double, green: c_double, blue: c_double, alpha: c_double) -> *cairo_pattern_t;

    pub fn cairo_pattern_get_rgba(pattern: *cairo_pattern_t, red: *c_double, green: *c_double, blue: *c_double, alpha: *c_double) -> Status;

    pub fn cairo_pattern_create_for_surface(surface: *cairo_surface_t) -> *cairo_pattern_t;

    pub fn cairo_pattern_get_surface(pattern: *cairo_pattern_t, surface: **cairo_surface_t) -> Status;

    pub fn cairo_pattern_create_linear(x0: c_double, y0: c_double, x1: c_double, y1: c_double) -> *cairo_pattern_t;

    pub fn cairo_pattern_get_linear_points(pattern: *cairo_pattern_t, x0: *c_double, y0: *c_double, x1: *c_double, y1: *c_double) -> Status;

    pub fn cairo_pattern_create_radial(cx0: c_double, cy0: c_double, radius0: c_double, cx1: c_double, cy1: c_double, radius1: c_double) -> *cairo_pattern_t;

    pub fn cairo_pattern_get_radial_circles(pattern: *cairo_pattern_t, x0: *c_double, y0: *c_double, r0: *c_double, x1: *c_double, y1: *c_double, r1: *c_double) -> Status;

    pub fn cairo_pattern_create_mesh() -> *cairo_pattern_t;

    pub fn cairo_mesh_pattern_begin_patch(pattern: *cairo_pattern_t);

    pub fn cairo_mesh_pattern_end_patch(pattern: *cairo_pattern_t);

    pub fn cairo_mesh_pattern_move_to(pattern: *cairo_pattern_t, x: c_double, y: c_double);

    pub fn cairo_mesh_pattern_line_to(pattern: *cairo_pattern_t, x: c_double, y: c_double);

    pub fn cairo_mesh_pattern_curve_to(pattern: *cairo_pattern_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);

    pub fn cairo_mesh_pattern_set_control_point(pattern: *cairo_pattern_t, point_num: c_uint, x: c_double, y: c_double);

    pub fn cairo_mesh_pattern_set_corner_color_rgb(pattern: *cairo_pattern_t, corner_num: c_uint, red: c_double, green: c_double, blue: c_double);

    pub fn cairo_mesh_pattern_set_corner_color_rgba(pattern: *cairo_pattern_t, corner_num: c_uint, red: c_double, green: c_double, blue: c_double, alpha: c_double);

    pub fn cairo_mesh_pattern_get_patch_count(pattern: *cairo_pattern_t, count: *c_uint) -> Status;

    pub fn cairo_mesh_pattern_get_path(pattern: *cairo_pattern_t, patch_num: c_uint) -> *cairo_path_t;

    pub fn cairo_mesh_pattern_get_control_point(pattern: *cairo_pattern_t, patch_num: c_uint, point_num: c_uint, x: *c_double, y: *c_double) -> Status;

    pub fn cairo_mesh_pattern_get_corner_color_rgba(pattern: *cairo_pattern_t, patch_num: c_uint, corner_num: c_uint, red: *c_double, green: *c_double, blue: *c_double, alpha: *c_double) -> Status;

    pub fn cairo_pattern_reference(pattern: *cairo_pattern_t) -> *cairo_pattern_t;

    pub fn cairo_pattern_destroy(pattern: *cairo_pattern_t);

    pub fn cairo_pattern_status(pattern: *cairo_pattern_t) -> Status;

    pub fn cairo_pattern_set_extend(pattern: *cairo_pattern_t, extend: Extend);

    pub fn cairo_pattern_get_extend(pattern: *cairo_pattern_t) -> Extend;

    pub fn cairo_pattern_set_filter(pattern: *cairo_pattern_t, filter: Filter);

    pub fn cairo_pattern_get_filter(pattern: *cairo_pattern_t) -> Filter;

    pub fn cairo_pattern_set_matrix(pattern: *cairo_pattern_t, matrix: *Matrix);

    pub fn cairo_pattern_get_matrix(pattern: *cairo_pattern_t, matrix: *Matrix);

    pub fn cairo_pattern_get_type(pattern: *cairo_pattern_t) -> PatternType;

    pub fn cairo_pattern_get_reference_count(pattern: *cairo_pattern_t) -> c_uint;

    //pub fn cairo_pattern_set_user_data(pattern: *cairo_pattern_t, key: *cairo_user_data_key_t, user_data: *void, destroy: cairo_destroy_func_t) -> Status;

    //pub fn cairo_pattern_get_user_data(pattern: *cairo_pattern_t, key: *cairo_user_data_key_t) -> *void;


    //CAIRO REGIONS

    pub fn cairo_region_create() -> *cairo_region_t;

    pub fn cairo_region_create_rectangle(rectangle: *cairo_rectangle_int_t) -> *cairo_region_t;

    pub fn cairo_region_create_rectangles(rects: *cairo_rectangle_int_t, count: c_int) -> *cairo_region_t;

    pub fn cairo_region_copy(original: *cairo_region_t) -> *cairo_region_t;

    pub fn cairo_region_reference(region: *cairo_region_t) -> *cairo_region_t;

    pub fn cairo_region_destroy(region: *cairo_region_t);

    pub fn cairo_region_status(region: *cairo_region_t) -> Status;

    pub fn cairo_region_get_extents(region: *cairo_region_t, extents: *cairo_rectangle_int_t);

    pub fn cairo_region_num_rectangles(region: *cairo_region_t) -> c_int;

    pub fn cairo_region_get_rectangle(region: *cairo_region_t, nth: c_int, rectangle: *cairo_rectangle_int_t);

    pub fn cairo_region_is_empty(region: *cairo_region_t) -> cairo_bool_t;

    pub fn cairo_region_contains_point(region: *cairo_region_t, x: c_int, y: c_int) -> cairo_bool_t;

    //enum                cairo_region_overlap_t;
    pub fn cairo_region_contains_rectangle(region: *cairo_region_t, rectangle: *cairo_rectangle_int_t) -> cairo_region_overlap_t;

    pub fn cairo_region_equal(a: *cairo_region_t, b: *cairo_region_t) -> cairo_bool_t;

    pub fn cairo_region_translate(region: *cairo_region_t, dx: c_int, dy: c_int);

    pub fn cairo_region_intersect(dst: *cairo_region_t, other: *cairo_region_t) -> Status;

    pub fn cairo_region_intersect_rectangle(dst: *cairo_region_t, rectangle: *cairo_rectangle_int_t) -> Status;

    pub fn cairo_region_subtract(dst: *cairo_region_t, other: *cairo_region_t) -> Status;

    pub fn cairo_region_subtract_rectangle(dst: *cairo_region_t, rectangle: *cairo_rectangle_int_t) -> Status;

    pub fn cairo_region_union(dst: *cairo_region_t, other: *cairo_region_t) -> Status;

    pub fn cairo_region_union_rectangle(dst: *cairo_region_t, rectangle: *cairo_rectangle_int_t) -> Status;

    pub fn cairo_region_xor(dst: *cairo_region_t, other: *cairo_region_t) -> Status;

    pub fn cairo_region_xor_rectangle(dst: *cairo_region_t, rectangle: *cairo_rectangle_int_t) -> Status;


    //text

    pub fn cairo_select_font_face(cr: *cairo_t, family: *c_char, slant: FontSlant, weight: FontWeight);

    pub fn cairo_set_font_size(cr: *cairo_t, size: c_double);

    pub fn cairo_set_font_matrix(cr: *cairo_t, matrix: *Matrix);

    pub fn cairo_get_font_matrix(cr: *cairo_t, matrix: *Matrix);

    pub fn cairo_set_font_options(cr: *cairo_t, options: *cairo_font_options_t);

    pub fn cairo_get_font_options(cr: *cairo_t, options: *cairo_font_options_t);

    pub fn cairo_set_font_face(cr: *cairo_t, font_face: *cairo_font_face_t);

    pub fn cairo_get_font_face(cr: *cairo_t) -> *cairo_font_face_t;

    pub fn cairo_set_scaled_font(cr: *cairo_t, scaled_font: *cairo_scaled_font_t);

    pub fn cairo_get_scaled_font(cr: *cairo_t) -> *cairo_scaled_font_t;

    pub fn cairo_show_text(cr: *cairo_t, utf8: *c_char);

    pub fn cairo_show_glyphs(cr: *cairo_t, glyphs: *Glyph, num_glyphs: c_int);

    pub fn cairo_show_text_glyphs(cr: *cairo_t, utf8: *c_char, utf8_len: c_int, glyphs: *Glyph, num_glyphs: c_int, clusters: *TextCluster, num_clusters: c_int, cluster_flags: TextClusterFlags);

    pub fn cairo_font_extents(cr: *cairo_t, extents: *FontExtents);

    pub fn cairo_text_extents(cr: *cairo_t, utf8: *c_char, extents: *TextExtents);

    pub fn cairo_glyph_extents(cr: *cairo_t, glyphs: *Glyph, num_glyphs: c_int, extents: *TextExtents);

    pub fn cairo_toy_font_face_create(family: *c_char, slant: FontSlant, weight: FontWeight) -> *cairo_font_face_t;

    pub fn cairo_toy_font_face_get_family(font_face: *cairo_font_face_t) -> *c_char;

    pub fn cairo_toy_font_face_get_slant(font_face: *cairo_font_face_t) -> FontSlant;

    pub fn cairo_toy_font_face_get_weight(font_face: *cairo_font_face_t) -> FontWeight;

    pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *Glyph;

    pub fn cairo_glyph_free(glyphs: *Glyph);

    pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *TextCluster;

    pub fn cairo_text_cluster_free(clusters: *TextCluster);


    //CAIRO RASTER

    //pub fn cairo_pattern_create_raster_source(user_data: *void, content: cairo_content_t, width: c_int, height: c_int) -> *cairo_pattern_t;

    //pub fn cairo_raster_source_pattern_set_callback_data(pattern: *cairo_pattern_t, data: *void);

    //pub fn cairo_raster_source_pattern_get_callback_data(pattern: *cairo_pattern_t) -> *void;

    /* FIXME how do we do these _func_t types?
    pub fn cairo_raster_source_pattern_set_acquire(pattern: *cairo_pattern_t, acquire: cairo_raster_source_acquire_func_t, release: cairo_raster_source_release_func_t);

    pub fn cairo_raster_source_pattern_get_acquire(pattern: *cairo_pattern_t, acquire: *cairo_raster_source_acquire_func_t, release: *cairo_raster_source_release_func_t);

    pub fn cairo_raster_source_pattern_set_snapshot(pattern: *cairo_pattern_t, snapshot: cairo_raster_source_snapshot_func_t);

    pub fn cairo_raster_source_pattern_get_snapshot(pattern: *cairo_pattern_t) -> cairo_raster_source_snapshot_func_t;

    pub fn cairo_raster_source_pattern_set_copy(pattern: *cairo_pattern_t, copy: cairo_raster_source_copy_func_t);

    pub fn cairo_raster_source_pattern_get_copy(pattern: *cairo_pattern_t) -> cairo_raster_source_copy_func_t;

    pub fn cairo_raster_source_pattern_set_finish(pattern: *cairo_pattern_t, finish: cairo_raster_source_finish_func_t);

    pub fn cairo_raster_source_pattern_get_finish(pattern: *cairo_pattern_t) -> cairo_raster_source_finish_func_t;
    */

    //cairo_surface_t     (*cairo_raster_source_acquire_func_t)
    //                                                        (pattern: *cairo_pattern_t, callback_data: *void, target: *cairo_surface_t, extents: *cairo_rectangle_int_t);
    //void                (*cairo_raster_source_release_func_t)
    //                                                        (pattern: *cairo_pattern_t, callback_data: *void, surface: *cairo_surface_t);
    //Status      (*cairo_raster_source_snapshot_func_t)
    //                                                        (pattern: *cairo_pattern_t, callback_data: *void);
    //Status      (*cairo_raster_source_copy_func_t)  (pattern: *cairo_pattern_t, callback_data: *void, other: *cairo_pattern_t);
    //void                (*cairo_raster_source_finish_func_t)
    //                                                        (pattern: *cairo_pattern_t, callback_data: *void);

    //CAIRO FONT
    pub fn cairo_font_face_reference(font_face: *cairo_font_face_t) -> *cairo_font_face_t;

    pub fn cairo_font_face_destroy(font_face: *cairo_font_face_t);

    pub fn cairo_font_face_status(font_face: *cairo_font_face_t) -> Status;

    pub fn cairo_font_face_get_type(font_face: *cairo_font_face_t) -> FontType;

    pub fn cairo_font_face_get_reference_count(font_face: *cairo_font_face_t) -> c_uint;

    //pub fn cairo_font_face_set_user_data(font_face: *cairo_font_face_t, key: *cairo_user_data_key_t, user_data: *void, destroy: cairo_destroy_func_t) -> Status;

    //pub fn cairo_font_face_get_user_data(font_face: *cairo_font_face_t, key: *cairo_user_data_key_t) -> *void;


    //CAIRO SCALED FONT
    pub fn cairo_scaled_font_create(font_face: *cairo_font_face_t, font_matrix: *Matrix, ctm: *Matrix, options: *cairo_font_options_t) -> *cairo_scaled_font_t;

    pub fn cairo_scaled_font_reference(scaled_font: *cairo_scaled_font_t) -> *cairo_scaled_font_t;

    pub fn cairo_scaled_font_destroy(scaled_font: *cairo_scaled_font_t);

    pub fn cairo_scaled_font_status(scaled_font: *cairo_scaled_font_t) -> Status;

    //                    FontExtents;
    pub fn cairo_scaled_font_extents(scaled_font: *cairo_scaled_font_t, extents: *FontExtents);

    //                    TextExtents;
    pub fn cairo_scaled_font_text_extents(scaled_font: *cairo_scaled_font_t, utf8: *c_char, extents: *TextExtents);

    pub fn cairo_scaled_font_glyph_extents(scaled_font: *cairo_scaled_font_t, glyphs: *Glyph, num_glyphs: c_int, extents: *TextExtents);

    pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *cairo_scaled_font_t, x: c_double, y: c_double, utf8: *c_char, utf8_len: c_int, glyphs: **Glyph, num_glyphs: *c_int, clusters: **TextCluster, num_clusters: *c_int, cluster_flags: *TextClusterFlags) -> Status;

    pub fn cairo_scaled_font_get_font_face(scaled_font: *cairo_scaled_font_t) -> *cairo_font_face_t;

    pub fn cairo_scaled_font_get_font_options(scaled_font: *cairo_scaled_font_t, options: *cairo_font_options_t);

    pub fn cairo_scaled_font_get_font_matrix(scaled_font: *cairo_scaled_font_t, font_matrix: *Matrix);

    pub fn cairo_scaled_font_get_ctm(scaled_font: *cairo_scaled_font_t, ctm: *Matrix);

    pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *cairo_scaled_font_t, scale_matrix: *Matrix);

    pub fn cairo_scaled_font_get_type(scaled_font: *cairo_scaled_font_t) -> FontType;

    pub fn cairo_scaled_font_get_reference_count(font_face: *cairo_scaled_font_t) -> c_uint;

    //pub fn cairo_scaled_font_set_user_data(scaled_font: *cairo_scaled_font_t, key: *cairo_user_data_key_t, user_data: *void, destroy: cairo_destroy_func_t) -> Status;

    //pub fn cairo_scaled_font_get_user_data(scaled_font: *cairo_scaled_font_t, key: *cairo_user_data_key_t) -> *void;


    //CAIRO FONT OPTIONS

    pub fn cairo_font_options_create() -> *cairo_font_options_t;

    pub fn cairo_font_options_copy(original: *cairo_font_options_t) -> *cairo_font_options_t;

    pub fn cairo_font_options_destroy(options: *cairo_font_options_t);

    pub fn cairo_font_options_status(options: *cairo_font_options_t) -> Status;

    pub fn cairo_font_options_merge(options: *cairo_font_options_t, other: *cairo_font_options_t);

    pub fn cairo_font_options_hash(options: *cairo_font_options_t) -> c_ulong;

    pub fn cairo_font_options_equal(options: *cairo_font_options_t, other: *cairo_font_options_t) -> cairo_bool_t;

    pub fn cairo_font_options_set_antialias(options: *cairo_font_options_t, antialias: Antialias);

    pub fn cairo_font_options_get_antialias(options: *cairo_font_options_t) -> Antialias;

    pub fn cairo_font_options_set_subpixel_order(options: *cairo_font_options_t, subpixel_order: SubpixelOrder);

    pub fn cairo_font_options_get_subpixel_order(options: *cairo_font_options_t) -> SubpixelOrder;

    pub fn cairo_font_options_set_hint_style(options: *cairo_font_options_t, hint_style: HintStyle);

    pub fn cairo_font_options_get_hint_style(options: *cairo_font_options_t) -> HintStyle;

    pub fn cairo_font_options_set_hint_metrics(options: *cairo_font_options_t, hint_metrics: HintMetrics);

    pub fn cairo_font_options_get_hint_metrics(options: *cairo_font_options_t) -> HintMetrics;

    // CAIRO MATRIX

    pub fn cairo_matrix_multiply(matrix: *mut Matrix, left: *Matrix, right: *Matrix);

    pub fn cairo_matrix_init(matrix: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);

    pub fn cairo_matrix_init_identity(matrix: *mut Matrix);

    pub fn cairo_matrix_translate(matrix: *mut Matrix, tx: f64, ty: f64);

    pub fn cairo_matrix_scale(matrix: *mut Matrix, sx: f64, sy: f64);

    pub fn cairo_matrix_rotate(matrix: *mut Matrix, angle: f64);

    pub fn cairo_matrix_invert(matrix: *mut Matrix) -> Status;

    pub fn cairo_matrix_transform_distance(matrix: *Matrix, dx: *f64, dy: *f64);

    pub fn cairo_matrix_transform_point(matrix: *Matrix, x: *f64, y: *f64);

}