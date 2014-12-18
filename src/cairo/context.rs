// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use std::c_vec::CVec;
use std::mem::transmute;
use libc::{c_double, c_int};
use cairo::paths::Path;
use cairo::fonts::{TextExtents, TextCluster, FontExtents, ScaledFont, FontOptions, FontFace, Glyph};
use cairo::matrices::Matrix;
use cairo::enums::{
    FontSlant,
    FontWeight,
    TextClusterFlags
};

use cairo::ffi;
use cairo::ffi::{
    cairo_t,
    cairo_surface_t
};
use cairo::enums::{Status, Antialias, LineCap, LineJoin, FillRule};
use cairo::patterns::{wrap_pattern, Pattern};

#[repr(C)]
#[deriving(Copy)]
pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[repr(C)]
pub struct Context(*mut cairo_t);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe{
            ffi::cairo_destroy(self.get_ptr())
        }
    }
}

impl Context {
    pub fn get_ptr(&self) -> *mut cairo_t{
        let Context(ptr) = *self;
        ptr
    }

    pub fn wrap(ptr: *mut cairo_t) -> Context{
        unsafe{
            Context(ffi::cairo_reference(ptr))
        }
    }

    pub fn reference(&self) -> Context{
        unsafe{
            Context(ffi::cairo_reference(self.get_ptr()))
        }
    }

    pub fn ensure_status(&self){
        self.status().ensure_valid();
    }

    pub fn new(target: *mut cairo_surface_t) -> Context {
        unsafe {
            Context(ffi::cairo_create(target))
        }
    }

    pub fn status (&self) -> Status {
        unsafe {
            ffi::cairo_status(self.get_ptr())
        }
    }

    pub fn save (&self) {
        unsafe {
            ffi::cairo_save(self.get_ptr())
        }
        self.ensure_status()
    }

    pub fn restore (&self) {
        unsafe{
            ffi::cairo_restore(self.get_ptr())
        }
        self.ensure_status()
    }

    //fn ffi::cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;

    pub fn push_group(&self){
        unsafe{
            ffi::cairo_push_group(self.get_ptr())
        }
    }

    /*
    pub fn push_group_with_content(&self, content: Content){
        unsafe{
            ffi::cairo_push_group_with_content(self.get_ptr(), content)
        }
    }*/

    pub fn pop_group(&self) -> Box<Pattern>{
        unsafe{
            wrap_pattern(ffi::cairo_pop_group(self.get_ptr()))
        }
    }

    pub fn pop_group_to_source(&self){
        unsafe{
            ffi::cairo_pop_group_to_source(self.get_ptr())
        }
    }

    //fn ffi::cairo_get_group_target (cr: *mut cairo_t) -> *mut cairo_surface_t;

    pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64){
        unsafe{
            ffi::cairo_set_source_rgb(self.get_ptr(), red, green, blue)
        }
    }

    pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64){
        unsafe{
            ffi::cairo_set_source_rgba(self.get_ptr(), red, green, blue, alpha)
        }
    }

    pub fn set_source(&self, source: &Pattern){
        unsafe{
            ffi::cairo_set_source(self.get_ptr(), source.get_ptr());
        }
        self.ensure_status();
    }

    pub fn get_source(&self) -> Box<Pattern>{
        unsafe{
            wrap_pattern(ffi::cairo_get_source(self.get_ptr()))
        }
    }

    //fn ffi::cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, x: c_double, y: c_double);

    pub fn set_antialias(&self, antialias : Antialias){
        unsafe{
            ffi::cairo_set_antialias(self.get_ptr(), antialias)
        }
        self.ensure_status()
    }

    pub fn get_antialias(&self) -> Antialias{
        unsafe{
            ffi::cairo_get_antialias(self.get_ptr())
        }
    }

    pub fn set_dash(&self, dashes: &[f64], num_dashes: i32, offset: f64){
        unsafe{
            ffi::cairo_set_dash(self.get_ptr(), dashes.as_ptr(), num_dashes, offset)
        }
        self.ensure_status(); //Possible invalid dashes value
    }

    pub fn get_dash_count(&self) -> i32{
        unsafe{
            ffi::cairo_get_dash_count(self.get_ptr())
        }
    }

    pub fn get_dash(&self) -> (Vec<f64>, f64){
        let dash_count = self.get_dash_count() as uint;
        let mut dashes: Vec<f64> = Vec::with_capacity(dash_count);
        let mut offset: f64 = 0.0;

        unsafe{
            ffi::cairo_get_dash(self.get_ptr(), dashes.as_mut_ptr(), &mut offset);
            (dashes, offset)
        }
    }

    pub fn get_dash_dashes(&self) -> Vec<f64>{
        let (dashes, _) = self.get_dash();
        dashes
    }

    pub fn get_dash_offset(&self) -> f64{
        let (_, offset) = self.get_dash();
        offset
    }


    pub fn set_fill_rule(&self, fill_rule : FillRule){
        unsafe{
            ffi::cairo_set_fill_rule(self.get_ptr(), fill_rule);
        }
        self.ensure_status();
    }

    pub fn get_fill_rule(&self) -> FillRule{
        unsafe{
            ffi::cairo_get_fill_rule(self.get_ptr())
        }
    }

    pub fn set_line_cap(&self, arg: LineCap){
        unsafe{
            ffi::cairo_set_line_cap(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    pub fn get_line_cap(&self) -> LineCap{
        unsafe{
            ffi::cairo_get_line_cap(self.get_ptr())
        }
    }

    pub fn set_line_join(&self, arg: LineJoin){
        unsafe{
            ffi::cairo_set_line_join(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    pub fn get_line_join(&self) -> LineJoin{
        unsafe{
            ffi::cairo_get_line_join(self.get_ptr())
        }
    }

    pub fn set_line_width(&self, arg: f64){
        unsafe{
            ffi::cairo_set_line_width(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    pub fn get_line_width(&self) -> f64{
        unsafe{
            ffi::cairo_get_line_width(self.get_ptr())
        }
    }

    pub fn set_miter_limit(&self, arg: f64){
        unsafe{
            ffi::cairo_set_miter_limit(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    pub fn get_miter_limit(&self) -> f64{
        unsafe{
            ffi::cairo_get_miter_limit(self.get_ptr())
        }
    }

    pub fn set_tolerance(&self, arg: f64){
        unsafe{
            ffi::cairo_set_tolerance(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    pub fn get_tolerance(&self) -> f64{
        unsafe{
            ffi::cairo_get_tolerance(self.get_ptr())
        }
    }

    pub fn clip(&self){
        unsafe{
            ffi::cairo_clip(self.get_ptr())
        }
    }

    pub fn clip_preserve(&self){
        unsafe{
            ffi::cairo_clip_preserve(self.get_ptr())
        }
    }

    pub fn clip_extents(&self) -> (f64,f64,f64,f64){
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe{
            ffi::cairo_clip_extents(self.get_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }

        (x1, y1, x2, y2)
    }

    pub fn in_clip(&self, x:f64, y:f64) -> bool{
        unsafe{
            ffi::cairo_in_clip(self.get_ptr(), x, y).as_bool()
        }
    }

    pub fn reset_clip(&self){
        unsafe{
            ffi::cairo_reset_clip(self.get_ptr())
        }
        self.ensure_status()
    }

    pub fn copy_clip_rectangle_list(&self) -> CVec<Rectangle>{
        unsafe{
            let rectangle_list = ffi::cairo_copy_clip_rectangle_list(self.get_ptr());

            (*rectangle_list).status.ensure_valid();

            CVec::new_with_dtor((*rectangle_list).rectangles, (*rectangle_list).num_rectangles as uint, move || {
                ffi::cairo_rectangle_list_destroy(rectangle_list)
            })
        }
    }

    pub fn fill(&self){
        unsafe{
            ffi::cairo_fill(self.get_ptr())
        }
    }

    pub fn fill_preserve(&self){
        unsafe{
            ffi::cairo_fill_preserve(self.get_ptr())
        }
    }

    pub fn fill_extents(&self) -> (f64,f64,f64,f64){
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe{
            ffi::cairo_fill_extents(self.get_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }

        (x1, y1, x2, y2)
    }

    pub fn in_fill(&self, x:f64, y:f64) -> bool{
        unsafe{
            ffi::cairo_in_fill(self.get_ptr(), x, y).as_bool()
        }
    }

    pub fn mask(&self, pattern: &Pattern){
        unsafe{
            ffi::cairo_mask(self.get_ptr(), pattern.get_ptr())
        }
    }

    //fn ffi::cairo_mask_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double);

    pub fn paint(&self){
        unsafe{
            ffi::cairo_paint(self.get_ptr())
        }
    }

    pub fn paint_with_alpha(&self, alpha: f64){
        unsafe{
            ffi::cairo_paint_with_alpha(self.get_ptr(), alpha)
        }
    }

    pub fn stroke(&self){
        unsafe{
            ffi::cairo_stroke(self.get_ptr())
        }
    }

    pub fn stroke_preserve(&self){
        unsafe{
            ffi::cairo_stroke_preserve(self.get_ptr())
        }
    }

    pub fn stroke_extents(&self) -> (f64,f64,f64,f64){
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe{
            ffi::cairo_stroke_extents(self.get_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }

        (x1, y1, x2, y2)
    }

    pub fn in_stroke(&self, x:f64, y:f64) -> bool{
        unsafe{
            ffi::cairo_in_stroke(self.get_ptr(), x, y).as_bool()
        }
    }

    pub fn copy_page(&self){
        unsafe{
            ffi::cairo_copy_page(self.get_ptr())
        }
    }

    pub fn show_page(&self){
        unsafe{
            ffi::cairo_show_page(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> u32{
        unsafe{
            ffi::cairo_get_reference_count(self.get_ptr())
        }
    }

    // transformations stuff

     pub fn translate(&self, tx: f64, ty: f64){
        unsafe{
            ffi::cairo_translate(self.get_ptr(), tx, ty)
        }
    }

    pub fn scale(&self, sx: f64, sy: f64){
        unsafe{
            ffi::cairo_scale(self.get_ptr(), sx, sy)
        }
    }

    pub fn rotate(&self, angle: f64){
        unsafe{
            ffi::cairo_rotate(self.get_ptr(), angle)
        }
    }

    //pub fn cairo_transform(cr: *cairo_t, matrix: *cairo_matrix_t);

    //pub fn cairo_set_matrix(cr: *cairo_t, matrix: *cairo_matrix_t);

    //pub fn cairo_get_matrix(cr: *cairo_t, matrix: *cairo_matrix_t);

    pub fn identity_matrix(&self){
        unsafe{
            ffi::cairo_identity_matrix(self.get_ptr())
        }
    }

    pub fn user_to_device(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe{
            let x_ptr: *mut c_double = transmute(box x);
            let y_ptr: *mut c_double = transmute(box y);

            ffi::cairo_user_to_device(self.get_ptr(), x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    pub fn user_to_device_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe{
            let dx_ptr: *mut c_double = transmute(box dx);
            let dy_ptr: *mut c_double = transmute(box dy);

            ffi::cairo_user_to_device_distance(self.get_ptr(), dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    pub fn device_to_user(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe{
            let x_ptr: *mut c_double = transmute(box x);
            let y_ptr: *mut c_double = transmute(box y);

            ffi::cairo_device_to_user(self.get_ptr(), x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    pub fn device_to_user_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe{
            let dx_ptr: *mut c_double = transmute(box dx);
            let dy_ptr: *mut c_double = transmute(box dy);

            ffi::cairo_device_to_user_distance(self.get_ptr(), dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    // font stuff

    pub fn select_font_face<S: ToCStr>(&self, family: S, slant: FontSlant, weight: FontWeight){
        unsafe{
            family.with_c_str(|family|{
                ffi::cairo_select_font_face(self.get_ptr(), family, slant, weight)
            })
        }
    }

    pub fn set_font_size(&self, size: f64){
        unsafe{
            ffi::cairo_set_font_size(self.get_ptr(), size)
        }
    }

    //FIXME probably needs a heap allocation
    pub fn set_font_matrix(&self, matrix: Matrix){
        unsafe{
            ffi::cairo_set_font_matrix(self.get_ptr(), &matrix)
        }
    }

    pub fn get_font_matrix(&self) -> Matrix{
        let mut matrix = Matrix::null();
        unsafe{
            ffi::cairo_get_font_matrix(self.get_ptr(), &mut matrix);
        }
        matrix
    }

    pub fn set_font_options(&self, options: FontOptions){
        unsafe{
            ffi::cairo_set_font_options(self.get_ptr(), options.get_ptr())
        }
    }

    pub fn get_font_options(&self) -> FontOptions{
        let out = FontOptions::new();
        unsafe{
            ffi::cairo_get_font_options(self.get_ptr(), out.get_ptr());
        }
        out
    }

    pub fn set_font_face(&self, font_face: FontFace){
        unsafe{
            ffi::cairo_set_font_face(self.get_ptr(), font_face.get_ptr())
        }
    }

    pub fn get_font_face(&self) -> FontFace{
        unsafe{
            FontFace(ffi::cairo_get_font_face(self.get_ptr()))
        }
    }

    pub fn set_scaled_font(&self, scaled_font: ScaledFont){
        unsafe{
            ffi::cairo_set_scaled_font(self.get_ptr(), scaled_font.get_ptr())
        }
    }

    pub fn get_scaled_font(&self) -> ScaledFont{
        unsafe{
            ScaledFont(ffi::cairo_get_scaled_font(self.get_ptr()))
        }
    }

    pub fn show_text<S: ToCStr>(&self, text: S){
        unsafe{
            text.with_c_str(|text|{
                ffi::cairo_show_text(self.get_ptr(), text)
            })
        }
    }

    pub fn show_glyphs(&self, vec: &[Glyph]){
        unsafe{
            let slice: &[Glyph] = vec.as_slice();
            ffi::cairo_show_glyphs(self.get_ptr(), slice.as_ptr(), slice.len() as c_int)
        }
    }

    pub fn show_text_glyphs<S: ToCStr>(&self,
                                       text: S,
                                       glyph_vec: &[Glyph],
                                       cluster_vec: &[TextCluster],
                                       cluster_flags: TextClusterFlags){
        unsafe{
            let glyphs: &[Glyph] = glyph_vec.as_slice();
            let clusters: &[TextCluster] = cluster_vec.as_slice();

            text.with_c_str(|text| {
                ffi::cairo_show_text_glyphs(self.get_ptr(),
                                       text,
                                       -1 as c_int, //NUL terminated
                                       glyphs.as_ptr(),
                                       glyphs.len() as c_int,
                                       clusters.as_ptr(),
                                       clusters.len() as c_int,
                                       cluster_flags)
            })
        }
    }

    pub fn font_extents(&self) -> FontExtents{
        let mut extents = FontExtents{
            ascent: 0.0,
            descent: 0.0,
            height: 0.0,
            max_x_advance: 0.0,
            max_y_advance: 0.0,
        };

        unsafe{
            ffi::cairo_font_extents(self.get_ptr(), &mut extents);
        }

        extents
    }

    pub fn text_extents<S: ToCStr>(&self, text: S) -> TextExtents{
        let mut extents = TextExtents{
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        text.with_c_str(|text|{
            unsafe{
                ffi::cairo_text_extents(self.get_ptr(), text, &mut extents)
            }
        });

        extents
    }

    pub fn glyph_extents(&self, glyph_vec: &[Glyph]) -> TextExtents{
        let mut extents = TextExtents{
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        let glyphs = glyph_vec.as_slice();

        unsafe{
            ffi::cairo_glyph_extents(self.get_ptr(), glyphs.as_ptr(), glyphs.len() as c_int, &mut extents);
        }

        extents
    }

    // paths stuff

     pub fn copy_path(&self) -> Path{
        unsafe{
            Path::wrap(ffi::cairo_copy_path(self.get_ptr()))
        }
    }

    pub fn copy_path_flat(&self) -> Path{
        unsafe{
            Path::wrap(ffi::cairo_copy_path_flat(self.get_ptr()))
        }
    }

    pub fn append_path(&self, path: &Path){
        unsafe{
            ffi::cairo_append_path(self.get_ptr(), transmute(path))
        }
    }

    pub fn has_current_point(&self) -> bool{
        unsafe{
            ffi::cairo_has_current_point(self.get_ptr()).as_bool()
        }
    }

    pub fn get_current_point(&self) -> (f64, f64) {
        unsafe{
            let x = transmute(box 0.0f64);
            let y = transmute(box 0.0f64);
            ffi::cairo_get_current_point(self.get_ptr(), x, y);
            (*x, *y)
        }
    }

    pub fn new_path(&self){
        unsafe{
            ffi::cairo_new_path(self.get_ptr())
        }
    }

    pub fn new_sub_path(&self){
        unsafe{
            ffi::cairo_new_sub_path(self.get_ptr())
        }
    }

    pub fn close_path(&self){
        unsafe{
            ffi::cairo_close_path(self.get_ptr())
        }
    }

    pub fn arc(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64){
        unsafe{
            ffi::cairo_arc(self.get_ptr(), xc, yc, radius, angle1, angle2)
        }
    }

    pub fn arc_negative(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64){
        unsafe{
            ffi::cairo_arc_negative(self.get_ptr(), xc, yc, radius, angle1, angle2)
        }
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64){
        unsafe{
            ffi::cairo_curve_to(self.get_ptr(), x1, y1, x2, y2, x3, y3)
        }
    }

    pub fn line_to(&self, x: f64, y: f64){
        unsafe{
            ffi::cairo_line_to(self.get_ptr(), x, y)
        }
    }

    pub fn move_to(&self, x: f64, y: f64){
        unsafe{
            ffi::cairo_move_to(self.get_ptr(), x, y)
        }
    }

    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64){
        unsafe{
            ffi::cairo_rectangle(self.get_ptr(), x, y, width, height)
        }
    }

    pub fn text_path(&self, str : &str){
        unsafe{
            str.with_c_str(|str|{
                ffi::cairo_text_path(self.get_ptr(), str)
            })
        }
    }

    //fn ffi::cairo_glyph_path(cr: *mut cairo_t, glyphs: *mut cairo_glyph_t, num_glyphs: int);

    pub fn rel_curve_to(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64){
        unsafe{
            ffi::cairo_rel_curve_to(self.get_ptr(), dx1, dy1, dx2, dy2, dx3, dy3)
        }
    }

    pub fn rel_line_to(&self, dx: f64, dy: f64){
        unsafe{
            ffi::cairo_rel_line_to(self.get_ptr(), dx, dy)
        }
    }

    pub fn rel_move_to(&self, dx: f64, dy: f64){
        unsafe{
            ffi::cairo_rel_move_to(self.get_ptr(), dx, dy)
        }
    }
}