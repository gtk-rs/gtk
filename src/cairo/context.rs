
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

#[repr(C)]
pub struct Context(*mut cairo_t);

impl Drop for Context{
    fn drop(&mut self){
        unsafe{
            ffi::cairo_destroy(self.get_ptr())
        }
    }
}

impl Context{
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

            CVec::new_with_dtor((*rectangle_list).rectangles, (*rectangle_list).num_rectangles as uint, proc(){
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
}