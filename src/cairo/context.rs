
use std::mem::transmute;
use std::c_vec::CVec;

use cairo::ffi;
use cairo::ffi::{
    cairo_t,
    cairo_surface_t
};
use cairo::enums::{Status, Antialias, LineCap, LineJoin, FillRule};
use cairo::patterns::{wrap_pattern, Pattern};

pub struct Rectangle{
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub struct Context{
    pub pointer: *cairo_t
}

impl Clone for Context{
    fn clone(&self) -> Context{
        Context {
            pointer: unsafe{
                ffi::cairo_reference(self.pointer)
            }
        }
    }
}

impl Drop for Context{
    fn drop(&mut self){
        unsafe{
            ffi::cairo_destroy(self.pointer)
        }
    }
}

impl Context{
    pub fn ensure_status(&self){
        self.status().ensure_valid();
    }

    pub fn new(target: *cairo_surface_t) -> Context {
        unsafe {
            Context {
                pointer: ffi::cairo_create(target)
            }
        }
    }

    pub fn status (&self) -> Status {
        unsafe {
            ffi::cairo_status(self.pointer)
        }
    }

    pub fn save (&self) {
        unsafe {
            ffi::cairo_save(self.pointer)
        }
        self.ensure_status()
    }

    pub fn restore (&self) {
        unsafe{
            ffi::cairo_restore(self.pointer)
        }
        self.ensure_status()
    }

    //fn ffi::cairo_get_target (cr: *cairo_t) -> *cairo_surface_t;

    pub fn push_group(&self){
        unsafe{
            ffi::cairo_push_group(self.pointer)
        }
    }

    /*
    pub fn push_group_with_content(&self, content: Content){
        unsafe{
            ffi::cairo_push_group_with_content(self.pointer, content)
        }
    }*/

    pub fn pop_group(&self) -> Box<Pattern>{
        unsafe{
            wrap_pattern(ffi::cairo_pop_group(self.pointer))
        }
    }

    pub fn pop_group_to_source(&self){
        unsafe{
            ffi::cairo_pop_group_to_source(self.pointer)
        }
    }

    //fn ffi::cairo_get_group_target (cr: *cairo_t) -> *cairo_surface_t;

    pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64){
        unsafe{
            ffi::cairo_set_source_rgb(self.pointer, red, green, blue)
        }
    }

    pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64){
        unsafe{
            ffi::cairo_set_source_rgba(self.pointer, red, green, blue, alpha)
        }
    }

    pub fn set_source(&self, source: &Pattern){
        unsafe{
            ffi::cairo_set_source(self.pointer, source.get_ptr());
        }
        self.ensure_status();
    }

    pub fn get_source(&self) -> Box<Pattern>{
        unsafe{
            wrap_pattern(ffi::cairo_get_source(self.pointer))
        }
    }

    //fn ffi::cairo_set_source_surface (cr: *cairo_t, surface: *cairo_surface_t, x: c_double, y: c_double);

    pub fn set_antialias(&self, antialias : Antialias){
        unsafe{
            ffi::cairo_set_antialias(self.pointer, antialias)
        }
        self.ensure_status()
    }

    pub fn get_antialias(&self) -> Antialias{
        unsafe{
            ffi::cairo_get_antialias(self.pointer)
        }
    }

    pub fn set_dash(&self, dashes: &[f64], num_dashes: i32, offset: f64){
        unsafe{
            ffi::cairo_set_dash(self.pointer, dashes.as_ptr(), num_dashes, offset)
        }
        self.ensure_status(); //Possible invalid dashes value
    }

    pub fn get_dash_count(&self) -> i32{
        unsafe{
            ffi::cairo_get_dash_count(self.pointer)
        }
    }

    pub fn get_dash(&self) -> (Vec<f64>, f64){
        let dash_count = self.get_dash_count() as uint;
        let dashes : Vec<f64> = Vec::with_capacity(dash_count);
        let offset : Box<f64> = box 0.0;

        unsafe{
            let offset_ptr : *f64 = transmute(offset);
            ffi::cairo_get_dash(self.pointer, dashes.as_ptr(), offset_ptr);
            (dashes, *offset_ptr)
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
            ffi::cairo_set_fill_rule(self.pointer, fill_rule);
        }
        self.ensure_status();
    }

    pub fn get_fill_rule(&self) -> FillRule{
        unsafe{
            ffi::cairo_get_fill_rule(self.pointer)
        }
    }

    pub fn set_line_cap(&self, arg: LineCap){
        unsafe{
            ffi::cairo_set_line_cap(self.pointer, arg)
        }
        self.ensure_status();
    }

    pub fn get_line_cap(&self) -> LineCap{
        unsafe{
            ffi::cairo_get_line_cap(self.pointer)
        }
    }

    pub fn set_line_join(&self, arg: LineJoin){
        unsafe{
            ffi::cairo_set_line_join(self.pointer, arg)
        }
        self.ensure_status();
    }

    pub fn get_line_join(&self) -> LineJoin{
        unsafe{
            ffi::cairo_get_line_join(self.pointer)
        }
    }

    pub fn set_line_width(&self, arg: f64){
        unsafe{
            ffi::cairo_set_line_width(self.pointer, arg)
        }
        self.ensure_status();
    }

    pub fn get_line_width(&self) -> f64{
        unsafe{
            ffi::cairo_get_line_width(self.pointer)
        }
    }

    pub fn set_miter_limit(&self, arg: f64){
        unsafe{
            ffi::cairo_set_miter_limit(self.pointer, arg)
        }
        self.ensure_status();
    }

    pub fn get_miter_limit(&self) -> f64{
        unsafe{
            ffi::cairo_get_miter_limit(self.pointer)
        }
    }

    pub fn set_tolerance(&self, arg: f64){
        unsafe{
            ffi::cairo_set_tolerance(self.pointer, arg)
        }
        self.ensure_status();
    }

    pub fn get_tolerance(&self) -> f64{
        unsafe{
            ffi::cairo_get_tolerance(self.pointer)
        }
    }

    pub fn clip(&self){
        unsafe{
            ffi::cairo_clip(self.pointer)
        }
    }

    pub fn clip_preserve(&self){
        unsafe{
            ffi::cairo_clip_preserve(self.pointer)
        }
    }

    pub fn clip_extents(&self) -> (f64,f64,f64,f64){
        let x1 : f64 = 0.0;
        let y1 : f64 = 0.0;
        let x2 : f64 = 0.0;
        let y2 : f64 = 0.0;

        unsafe{
            ffi::cairo_clip_extents(self.pointer, &x1, &y1, &x2, &y2);
        }

        (x1, y1, x2, y2)
    }

    pub fn in_clip(&self, x:f64, y:f64) -> bool{
        unsafe{
            ffi::cairo_in_clip(self.pointer, x, y).as_bool()
        }
    }

    pub fn reset_clip(&self){
        unsafe{
            ffi::cairo_reset_clip(self.pointer)
        }
        self.ensure_status()
    }

    pub fn copy_clip_rectangle_list(&self) -> CVec<Rectangle>{
        unsafe{
            let rectangle_list = ffi::cairo_copy_clip_rectangle_list(self.pointer);

            (*rectangle_list).status.ensure_valid();

            CVec::new_with_dtor((*rectangle_list).rectangles, (*rectangle_list).num_rectangles as uint, proc(){
                ffi::cairo_rectangle_list_destroy(rectangle_list)
            })
        }
    }

    pub fn fill(&self){
        unsafe{
            ffi::cairo_fill(self.pointer)
        }
    }

    pub fn fill_preserve(&self){
        unsafe{
            ffi::cairo_fill_preserve(self.pointer)
        }
    }

    pub fn fill_extents(&self) -> (f64,f64,f64,f64){
        let x1 : f64 = 0.0;
        let y1 : f64 = 0.0;
        let x2 : f64 = 0.0;
        let y2 : f64 = 0.0;

        unsafe{
            ffi::cairo_fill_extents(self.pointer, &x1, &y1, &x2, &y2);
        }

        (x1, y1, x2, y2)
    }

    pub fn in_fill(&self, x:f64, y:f64) -> bool{
        unsafe{
            ffi::cairo_in_fill(self.pointer, x, y).as_bool()
        }
    }

    pub fn mask(&self, pattern: &Pattern){
        unsafe{
            ffi::cairo_mask(self.pointer, pattern.get_ptr())
        }
    }

    //fn ffi::cairo_mask_surface (cr: *cairo_t, surface: *cairo_surface_t, surface_x: c_double, surface_y: c_double);

    pub fn paint(&self){
        unsafe{
            ffi::cairo_paint(self.pointer)
        }
    }

    pub fn paint_with_alpha(&self, alpha: f64){
        unsafe{
            ffi::cairo_paint_with_alpha(self.pointer, alpha)
        }
    }

    pub fn stroke(&self){
        unsafe{
            ffi::cairo_stroke(self.pointer)
        }
    }

    pub fn stroke_preserve(&self){
        unsafe{
            ffi::cairo_stroke_preserve(self.pointer)
        }
    }

    pub fn stroke_extents(&self) -> (f64,f64,f64,f64){
        let x1 : f64 = 0.0;
        let y1 : f64 = 0.0;
        let x2 : f64 = 0.0;
        let y2 : f64 = 0.0;

        unsafe{
            ffi::cairo_stroke_extents(self.pointer, &x1, &y1, &x2, &y2);
        }

        (x1, y1, x2, y2)
    }

    pub fn in_stroke(&self, x:f64, y:f64) -> bool{
        unsafe{
            ffi::cairo_in_stroke(self.pointer, x, y).as_bool()
        }
    }

    pub fn copy_page(&self){
        unsafe{
            ffi::cairo_copy_page(self.pointer)
        }
    }

    pub fn show_page(&self){
        unsafe{
            ffi::cairo_show_page(self.pointer)
        }
    }

    pub fn get_reference_count(&self) -> u32{
        unsafe{
            ffi::cairo_get_reference_count(self.pointer)
        }
    }
}