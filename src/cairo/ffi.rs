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

#[repr(C)]
pub struct cairo_t;
#[repr(C)]
pub struct cairo_surface_t;
#[repr(C)]
pub struct cairo_pattern_t;
#[repr(C)]
pub struct cairo_fill_rule_t;
#[repr(C)]
pub struct cairo_antialias_t;
#[repr(C)]
pub struct cairo_destroy_func_t;
#[repr(C)]
pub struct cairo_line_join_t;
#[repr(C)]
pub struct cairo_line_cap_t;
#[repr(C)]
pub struct cairo_operator_t;
#[repr(C)]
pub struct cairo_rectangle_list_t{
    pub status: Status,
    pub rectangles: *mut Rectangle,
    pub num_rectangles: c_int
}
#[repr(C)]
pub struct cairo_rectangle_int_t;
#[repr(C)]
pub struct cairo_content_t;
#[repr(C)]
pub struct cairo_path_t{
    pub status: cairo::Status,
    pub data: *mut (c_double, c_double),
    pub num_data: c_int
}
#[repr(C)]
pub struct cairo_path_data_header{
    pub data_type: PathDataType,
    pub length:    c_int
}
#[repr(C)]
pub struct cairo_glyph_t;

#[repr(C)]
pub struct cairo_bool_t{
    value: c_int
}

impl cairo_bool_t{
    pub fn as_bool(&self) -> bool{
        self.value != 0
    }
}
#[repr(C)]
pub struct cairo_region_t;
#[repr(C)]
pub struct cairo_font_face_t;
#[repr(C)]
pub struct cairo_scaled_font_t;
#[repr(C)]
pub struct cairo_font_options_t;
#[repr(C)]
pub struct cairo_extend_t;
#[repr(C)]
pub struct cairo_filter_t;
#[repr(C)]
pub struct cairo_region_overlap_t;

#[link(name = "cairo")]
extern "C" {

    //CAIRO CONTEXT
    pub fn cairo_create (target: *mut cairo_surface_t) -> *mut cairo_t;

    pub fn cairo_reference (cr: *mut cairo_t) -> *mut cairo_t;

    pub fn cairo_destroy (cr: *mut cairo_t);

    pub fn cairo_status (cr: *mut cairo_t) -> Status;

    pub fn cairo_save (cr: *mut cairo_t);

    pub fn cairo_restore (cr: *mut cairo_t);

    pub fn cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;

    pub fn cairo_push_group (cr: *mut cairo_t);

    pub fn cairo_push_group_with_content (cr: *mut cairo_t, content: cairo_content_t);

    pub fn cairo_pop_group (cr: *mut cairo_t) -> *mut cairo_pattern_t;

    pub fn cairo_pop_group_to_source (cr: *mut cairo_t);

    pub fn cairo_get_group_target (cr: *mut cairo_t) -> *mut cairo_surface_t;

    pub fn cairo_set_source_rgb (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);

    pub fn cairo_set_source_rgba (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);

    pub fn cairo_set_source (cr: *mut cairo_t, source: *mut cairo_pattern_t);

    pub fn cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, x: c_double, y: c_double);

    pub fn cairo_get_source (cr: *mut cairo_t) -> *mut cairo_pattern_t;

    pub fn cairo_set_antialias (cr: *mut cairo_t, antialias: Antialias);

    pub fn cairo_get_antialias (cr: *mut cairo_t) -> Antialias;

    pub fn cairo_set_dash (cr: *mut cairo_t, dashes : *const c_double, num_dashes: c_int, offset: c_double);

    pub fn cairo_get_dash_count (cr: *mut cairo_t) -> c_int;

    pub fn cairo_get_dash (cr: *mut cairo_t, dashes: *mut c_double, offset: *mut c_double);

    pub fn cairo_set_fill_rule (cr: *mut cairo_t, fill_rule: FillRule);

    pub fn cairo_get_fill_rule (cr: *mut cairo_t) -> FillRule;

    pub fn cairo_set_line_cap (cr: *mut cairo_t, line_cap: LineCap);

    pub fn cairo_get_line_cap (cr: *mut cairo_t) -> LineCap;

    pub fn cairo_set_line_join (cr: *mut cairo_t, line_join: LineJoin);

    pub fn cairo_get_line_join (cr: *mut cairo_t) -> LineJoin;

    pub fn cairo_set_line_width (cr: *mut cairo_t, width: c_double);

    pub fn cairo_get_line_width (cr: *mut cairo_t) -> c_double;

    pub fn cairo_set_miter_limit (cr: *mut cairo_t, limit: c_double);

    pub fn cairo_get_miter_limit (cr: *mut cairo_t) -> c_double;

    pub fn cairo_set_operator (cr: *mut cairo_t, op: cairo_operator_t);

    pub fn cairo_get_operator (cr: *mut cairo_t) -> cairo_operator_t;

    pub fn cairo_set_tolerance (cr: *mut cairo_t, tolerance: c_double);

    pub fn cairo_get_tolerance (cr: *mut cairo_t) -> c_double;

    pub fn cairo_clip (cr: *mut cairo_t);

    pub fn cairo_clip_preserve (cr: *mut cairo_t);

    pub fn cairo_clip_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);

    pub fn cairo_in_clip (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;

    pub fn cairo_reset_clip (cr: *mut cairo_t);

    pub fn cairo_rectangle_list_destroy (rectangle_list: *mut cairo_rectangle_list_t);

    pub fn cairo_copy_clip_rectangle_list (cr: *mut cairo_t) -> *mut cairo_rectangle_list_t;

    pub fn cairo_fill (cr: *mut cairo_t);

    pub fn cairo_fill_preserve (cr: *mut cairo_t);

    pub fn cairo_fill_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);

    pub fn cairo_in_fill (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;

    pub fn cairo_mask (cr: *mut cairo_t, pattern: *mut cairo_pattern_t);

    pub fn cairo_mask_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double);

    pub fn cairo_paint (cr: *mut cairo_t);

    pub fn cairo_paint_with_alpha (cr: *mut cairo_t, alpha: c_double);

    pub fn cairo_stroke (cr: *mut cairo_t);

    pub fn cairo_stroke_preserve (cr: *mut cairo_t);

    pub fn cairo_stroke_extents (cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);

    pub fn cairo_in_stroke (cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;

    pub fn cairo_copy_page (cr: *mut cairo_t);

    pub fn cairo_show_page (cr: *mut cairo_t);

    pub fn cairo_get_reference_count (cr: *mut cairo_t) -> c_uint;


    //CAIRO UTILS: Error handling

    pub fn cairo_status_to_string (status : Status) -> *const c_char;


    //CAIRO PATHS

    pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;

    pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;

    pub fn cairo_path_destroy(path: *mut cairo_path_t);

    pub fn cairo_append_path(cr: *mut cairo_t, path: *mut cairo_path_t);

    pub fn cairo_has_current_point(cr: *mut cairo_t) -> cairo_bool_t;

    pub fn cairo_get_current_point(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);

    pub fn cairo_new_path(cr: *mut cairo_t);

    pub fn cairo_new_sub_path(cr: *mut cairo_t);

    pub fn cairo_close_path(cr: *mut cairo_t);

    pub fn cairo_arc(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);

    pub fn cairo_arc_negative(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double);

    pub fn cairo_curve_to(cr: *mut cairo_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);

    pub fn cairo_line_to(cr: *mut cairo_t, x: c_double, y: c_double);

    pub fn cairo_move_to(cr: *mut cairo_t, x: c_double, y: c_double);

    pub fn cairo_rectangle(cr: *mut cairo_t, x: c_double, y: c_double, width: c_double, height: c_double);

    pub fn cairo_glyph_path(cr: *mut cairo_t, glyphs: *mut Glyph, num_glyphs: c_int);

    pub fn cairo_text_path(cr: *mut cairo_t, utf8: *const c_char);

    pub fn cairo_rel_curve_to(cr: *mut cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double);

    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: c_double, dy: c_double);

    pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: c_double, dy: c_double);

    pub fn cairo_path_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);


    //CAIRO TRANSFORMATIONS

    pub fn cairo_translate(cr: *mut cairo_t, tx: c_double, ty: c_double);

    pub fn cairo_scale(cr: *mut cairo_t, sx: c_double, sy: c_double);

    pub fn cairo_rotate(cr: *mut cairo_t, angle: c_double);

    pub fn cairo_transform(cr: *mut cairo_t, matrix: *mut Matrix);

    pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *mut Matrix);

    pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut Matrix);

    pub fn cairo_identity_matrix(cr: *mut cairo_t);

    pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);

    pub fn cairo_user_to_device_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double);

    pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double);

    pub fn cairo_device_to_user_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double);


    //CAIRO PATTERNS

    pub fn cairo_pattern_add_color_stop_rgb(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double);

    pub fn cairo_pattern_add_color_stop_rgba(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double, alpha: c_double);

    pub fn cairo_pattern_get_color_stop_count(pattern: *mut cairo_pattern_t, count: *mut c_int) -> Status;

    pub fn cairo_pattern_get_color_stop_rgba(pattern: *mut cairo_pattern_t, index: c_int, offset: *mut c_double, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> Status;

    pub fn cairo_pattern_create_rgb(red: c_double, green: c_double, blue: c_double) -> *mut cairo_pattern_t;

    pub fn cairo_pattern_create_rgba(red: c_double, green: c_double, blue: c_double, alpha: c_double) -> *mut cairo_pattern_t;

    pub fn cairo_pattern_get_rgba(pattern: *mut cairo_pattern_t, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> Status;

    pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;

    pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: *mut *mut cairo_surface_t) -> Status;

    pub fn cairo_pattern_create_linear(x0: c_double, y0: c_double, x1: c_double, y1: c_double) -> *mut cairo_pattern_t;

    pub fn cairo_pattern_get_linear_points(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, x1: *mut c_double, y1: *mut c_double) -> Status;

    pub fn cairo_pattern_create_radial(cx0: c_double, cy0: c_double, radius0: c_double, cx1: c_double, cy1: c_double, radius1: c_double) -> *mut cairo_pattern_t;

    pub fn cairo_pattern_get_radial_circles(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, r0: *mut c_double, x1: *mut c_double, y1: *mut c_double, r1: *mut c_double) -> Status;

    pub fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t;

    pub fn cairo_mesh_pattern_begin_patch(pattern: *mut cairo_pattern_t);

    pub fn cairo_mesh_pattern_end_patch(pattern: *mut cairo_pattern_t);

    pub fn cairo_mesh_pattern_move_to(pattern: *mut cairo_pattern_t, x: c_double, y: c_double);

    pub fn cairo_mesh_pattern_line_to(pattern: *mut cairo_pattern_t, x: c_double, y: c_double);

    pub fn cairo_mesh_pattern_curve_to(pattern: *mut cairo_pattern_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double);

    pub fn cairo_mesh_pattern_set_control_point(pattern: *mut cairo_pattern_t, point_num: c_uint, x: c_double, y: c_double);

    pub fn cairo_mesh_pattern_set_corner_color_rgb(pattern: *mut cairo_pattern_t, corner_num: c_uint, red: c_double, green: c_double, blue: c_double);

    pub fn cairo_mesh_pattern_set_corner_color_rgba(pattern: *mut cairo_pattern_t, corner_num: c_uint, red: c_double, green: c_double, blue: c_double, alpha: c_double);

    pub fn cairo_mesh_pattern_get_patch_count(pattern: *mut cairo_pattern_t, count: *mut c_uint) -> Status;

    pub fn cairo_mesh_pattern_get_path(pattern: *mut cairo_pattern_t, patch_num: c_uint) -> *mut cairo_path_t;

    pub fn cairo_mesh_pattern_get_control_point(pattern: *mut cairo_pattern_t, patch_num: c_uint, point_num: c_uint, x: *mut c_double, y: *mut c_double) -> Status;

    pub fn cairo_mesh_pattern_get_corner_color_rgba(pattern: *mut cairo_pattern_t, patch_num: c_uint, corner_num: c_uint, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> Status;

    pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;

    pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);

    pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) -> Status;

    pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t, extend: Extend);

    pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> Extend;

    pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t, filter: Filter);

    pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) -> Filter;

    pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *mut Matrix);

    pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut Matrix);

    pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) -> PatternType;

    pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) -> c_uint;

    //pub fn cairo_pattern_set_user_data(pattern: *mut cairo_pattern_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> Status;

    //pub fn cairo_pattern_get_user_data(pattern: *mut cairo_pattern_t, key: *mut cairo_user_data_key_t) -> *mut void;


    //CAIRO REGIONS

    pub fn cairo_region_create() -> *mut cairo_region_t;

    pub fn cairo_region_create_rectangle(rectangle: *mut cairo_rectangle_int_t) -> *mut cairo_region_t;

    pub fn cairo_region_create_rectangles(rects: *mut cairo_rectangle_int_t, count: c_int) -> *mut cairo_region_t;

    pub fn cairo_region_copy(original: *mut cairo_region_t) -> *mut cairo_region_t;

    pub fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;

    pub fn cairo_region_destroy(region: *mut cairo_region_t);

    pub fn cairo_region_status(region: *mut cairo_region_t) -> Status;

    pub fn cairo_region_get_extents(region: *mut cairo_region_t, extents: *mut cairo_rectangle_int_t);

    pub fn cairo_region_num_rectangles(region: *mut cairo_region_t) -> c_int;

    pub fn cairo_region_get_rectangle(region: *mut cairo_region_t, nth: c_int, rectangle: *mut cairo_rectangle_int_t);

    pub fn cairo_region_is_empty(region: *mut cairo_region_t) -> cairo_bool_t;

    pub fn cairo_region_contains_point(region: *mut cairo_region_t, x: c_int, y: c_int) -> cairo_bool_t;

    //enum                cairo_region_overlap_t;
    pub fn cairo_region_contains_rectangle(region: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> cairo_region_overlap_t;

    pub fn cairo_region_equal(a: *mut cairo_region_t, b: *mut cairo_region_t) -> cairo_bool_t;

    pub fn cairo_region_translate(region: *mut cairo_region_t, dx: c_int, dy: c_int);

    pub fn cairo_region_intersect(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;

    pub fn cairo_region_intersect_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;

    pub fn cairo_region_subtract(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;

    pub fn cairo_region_subtract_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;

    pub fn cairo_region_union(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;

    pub fn cairo_region_union_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;

    pub fn cairo_region_xor(dst: *mut cairo_region_t, other: *mut cairo_region_t) -> Status;

    pub fn cairo_region_xor_rectangle(dst: *mut cairo_region_t, rectangle: *mut cairo_rectangle_int_t) -> Status;


    //text

    pub fn cairo_select_font_face(cr: *mut cairo_t, family: *const c_char, slant: FontSlant, weight: FontWeight);

    pub fn cairo_set_font_size(cr: *mut cairo_t, size: c_double);

    pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *const Matrix);

    pub fn cairo_get_font_matrix(cr: *mut cairo_t, matrix: *mut Matrix);

    pub fn cairo_set_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);

    pub fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);

    pub fn cairo_set_font_face(cr: *mut cairo_t, font_face: *mut cairo_font_face_t);

    pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;

    pub fn cairo_set_scaled_font(cr: *mut cairo_t, scaled_font: *mut cairo_scaled_font_t);

    pub fn cairo_get_scaled_font(cr: *mut cairo_t) -> *mut cairo_scaled_font_t;

    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const c_char);

    pub fn cairo_show_glyphs(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int);

    pub fn cairo_show_text_glyphs(cr: *mut cairo_t, utf8: *const c_char, utf8_len: c_int, glyphs: *const Glyph, num_glyphs: c_int, clusters: *const TextCluster, num_clusters: c_int, cluster_flags: TextClusterFlags);

    pub fn cairo_font_extents(cr: *mut cairo_t, extents: *mut FontExtents);

    pub fn cairo_text_extents(cr: *mut cairo_t, utf8: *const c_char, extents: *mut TextExtents);

    pub fn cairo_glyph_extents(cr: *mut cairo_t, glyphs: *const Glyph, num_glyphs: c_int, extents: *mut TextExtents);

    pub fn cairo_toy_font_face_create(family: *const c_char, slant: FontSlant, weight: FontWeight) -> *mut cairo_font_face_t;

    pub fn cairo_toy_font_face_get_family(font_face: *mut cairo_font_face_t) -> *const c_char;

    pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) -> FontSlant;

    pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t) -> FontWeight;

    pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *mut Glyph;

    pub fn cairo_glyph_free(glyphs: *mut Glyph);

    pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *mut TextCluster;

    pub fn cairo_text_cluster_free(clusters: *mut TextCluster);


    //CAIRO RASTER

    //pub fn cairo_pattern_create_raster_source(user_data: *mut void, content: cairo_content_t, width: c_int, height: c_int) -> *mut cairo_pattern_t;

    //pub fn cairo_raster_source_pattern_set_callback_data(pattern: *mut cairo_pattern_t, data: *mut void);

    //pub fn cairo_raster_source_pattern_get_callback_data(pattern: *mut cairo_pattern_t) -> *mut void;

    /* FIXME how do we do these _func_t types?
    pub fn cairo_raster_source_pattern_set_acquire(pattern: *mut cairo_pattern_t, acquire: cairo_raster_source_acquire_func_t, release: cairo_raster_source_release_func_t);

    pub fn cairo_raster_source_pattern_get_acquire(pattern: *mut cairo_pattern_t, acquire: *mut cairo_raster_source_acquire_func_t, release: *mut cairo_raster_source_release_func_t);

    pub fn cairo_raster_source_pattern_set_snapshot(pattern: *mut cairo_pattern_t, snapshot: cairo_raster_source_snapshot_func_t);

    pub fn cairo_raster_source_pattern_get_snapshot(pattern: *mut cairo_pattern_t) -> cairo_raster_source_snapshot_func_t;

    pub fn cairo_raster_source_pattern_set_copy(pattern: *mut cairo_pattern_t, copy: cairo_raster_source_copy_func_t);

    pub fn cairo_raster_source_pattern_get_copy(pattern: *mut cairo_pattern_t) -> cairo_raster_source_copy_func_t;

    pub fn cairo_raster_source_pattern_set_finish(pattern: *mut cairo_pattern_t, finish: cairo_raster_source_finish_func_t);

    pub fn cairo_raster_source_pattern_get_finish(pattern: *mut cairo_pattern_t) -> cairo_raster_source_finish_func_t;
    */

    //cairo_surface_t     (*mut cairo_raster_source_acquire_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void, target: *mut cairo_surface_t, extents: *mut cairo_rectangle_int_t);
    //void                (*mut cairo_raster_source_release_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void, surface: *mut cairo_surface_t);
    //Status      (*mut cairo_raster_source_snapshot_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void);
    //Status      (*mut cairo_raster_source_copy_func_t)  (pattern: *mut cairo_pattern_t, callback_data: *mut void, other: *mut cairo_pattern_t);
    //void                (*mut cairo_raster_source_finish_func_t)
    //                                                        (pattern: *mut cairo_pattern_t, callback_data: *mut void);

    //CAIRO FONT
    pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) -> *mut cairo_font_face_t;

    pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);

    pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) -> Status;

    pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) -> FontType;

    pub fn cairo_font_face_get_reference_count(font_face: *mut cairo_font_face_t) -> c_uint;

    //pub fn cairo_font_face_set_user_data(font_face: *mut cairo_font_face_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> Status;

    //pub fn cairo_font_face_get_user_data(font_face: *mut cairo_font_face_t, key: *mut cairo_user_data_key_t) -> *mut void;


    //CAIRO SCALED FONT
    pub fn cairo_scaled_font_create(font_face: *mut cairo_font_face_t, font_matrix: *mut Matrix, ctm: *mut Matrix, options: *mut cairo_font_options_t) -> *mut cairo_scaled_font_t;

    pub fn cairo_scaled_font_reference(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_scaled_font_t;

    pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);

    pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> Status;

    //                    FontExtents;
    pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut FontExtents);

    //                    TextExtents;
    pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *mut c_char, extents: *mut TextExtents);

    pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *mut Glyph, num_glyphs: c_int, extents: *mut TextExtents);

    pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *mut c_char, utf8_len: c_int, glyphs: *mut *mut Glyph, num_glyphs: *mut c_int, clusters: *mut *mut TextCluster, num_clusters: *mut c_int, cluster_flags: *mut TextClusterFlags) -> Status;

    pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;

    pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);

    pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut Matrix);

    pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut Matrix);

    pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *mut Matrix);

    pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t) -> FontType;

    pub fn cairo_scaled_font_get_reference_count(font_face: *mut cairo_scaled_font_t) -> c_uint;

    //pub fn cairo_scaled_font_set_user_data(scaled_font: *mut cairo_scaled_font_t, key: *mut cairo_user_data_key_t, user_data: *mut void, destroy: cairo_destroy_func_t) -> Status;

    //pub fn cairo_scaled_font_get_user_data(scaled_font: *mut cairo_scaled_font_t, key: *mut cairo_user_data_key_t) -> *mut void;


    //CAIRO FONT OPTIONS

    pub fn cairo_font_options_create() -> *mut cairo_font_options_t;

    pub fn cairo_font_options_copy(original: *mut cairo_font_options_t) -> *mut cairo_font_options_t;

    pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t);

    pub fn cairo_font_options_status(options: *mut cairo_font_options_t) -> Status;

    pub fn cairo_font_options_merge(options: *mut cairo_font_options_t, other: *mut cairo_font_options_t);

    pub fn cairo_font_options_hash(options: *mut cairo_font_options_t) -> c_ulong;

    pub fn cairo_font_options_equal(options: *mut cairo_font_options_t, other: *mut cairo_font_options_t) -> cairo_bool_t;

    pub fn cairo_font_options_set_antialias(options: *mut cairo_font_options_t, antialias: Antialias);

    pub fn cairo_font_options_get_antialias(options: *mut cairo_font_options_t) -> Antialias;

    pub fn cairo_font_options_set_subpixel_order(options: *mut cairo_font_options_t, subpixel_order: SubpixelOrder);

    pub fn cairo_font_options_get_subpixel_order(options: *mut cairo_font_options_t) -> SubpixelOrder;

    pub fn cairo_font_options_set_hint_style(options: *mut cairo_font_options_t, hint_style: HintStyle);

    pub fn cairo_font_options_get_hint_style(options: *mut cairo_font_options_t) -> HintStyle;

    pub fn cairo_font_options_set_hint_metrics(options: *mut cairo_font_options_t, hint_metrics: HintMetrics);

    pub fn cairo_font_options_get_hint_metrics(options: *mut cairo_font_options_t) -> HintMetrics;

    // CAIRO MATRIX

    pub fn cairo_matrix_multiply(matrix: *mut Matrix, left: *const Matrix, right: *const Matrix);

    pub fn cairo_matrix_init(matrix: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);

    pub fn cairo_matrix_init_identity(matrix: *mut Matrix);

    pub fn cairo_matrix_translate(matrix: *mut Matrix, tx: f64, ty: f64);

    pub fn cairo_matrix_scale(matrix: *mut Matrix, sx: f64, sy: f64);

    pub fn cairo_matrix_rotate(matrix: *mut Matrix, angle: f64);

    pub fn cairo_matrix_invert(matrix: *mut Matrix) -> Status;

    pub fn cairo_matrix_transform_distance(matrix: *const Matrix, dx: *mut f64, dy: *mut f64);

    pub fn cairo_matrix_transform_point(matrix: *const Matrix, x: *mut f64, y: *mut f64);

}