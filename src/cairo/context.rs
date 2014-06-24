
#![macro_escape]

use libc::{c_int, c_uint, c_char, c_double};
use std::mem::transmute;

use cairo::types::{
	cairo_t,
	cairo_surface_t,
	cairo_pattern_t,
	//cairo_destroy_func_t,
	//key,
	//dashes,
	cairo_operator_t,
	cairo_rectangle_list_t,
	cairo_content_t
};
use cairo::enums::{Status, status, Antialias, LineCap, LineJoin, FillRule};

pub struct Context{
	pointer: *cairo_t
}

impl Context{
	fn check_state(&self){
		let status = self.status();

		if status != status::Success{
			unsafe{
				fail!("Cairo crashed with: {}", cairo_status_to_string(status))
			}
		}
	}

	pub fn new (target: *cairo_surface_t) -> Context {
		unsafe {
			Context {
				pointer: cairo_create(target)
			}
		}
	}

	//fn cairo_reference (cr: *cairo_t) -> *cairo_t;

	//fn cairo_destroy (cr: *cairo_t);

	pub fn status (&self) -> Status {
		unsafe {
			cairo_status(self.pointer)
		}
	}

	pub fn save (&self) {
		unsafe {
			cairo_save(self.pointer)
		}
		self.check_state()
	}

	pub fn restore (&self) {
		unsafe{
			cairo_restore(self.pointer)
		}
		self.check_state()
	}

	//fn cairo_get_target (cr: *cairo_t) -> *cairo_surface_t;

	//fn cairo_push_group (cr: *cairo_t);

	//fn cairo_push_group_with_content (cr: *cairo_t, content: cairo_content_t);

	//fn cairo_pop_group (cr: *cairo_t) -> *cairo_pattern_t;

	//fn cairo_pop_group_to_source (cr: *cairo_t);

	//fn cairo_get_group_target (cr: *cairo_t) -> *cairo_surface_t;

	//fn cairo_set_source_rgb (cr: *cairo_t, red: c_double, green: c_double, blue: c_double);

	//fn cairo_set_source_rgba (cr: *cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);

	//fn cairo_set_source (cr: *cairo_t, source: *cairo_pattern_t);

	//fn cairo_set_source_surface (cr: *cairo_t, surface: *cairo_surface_t, x: c_double, y: c_double);

	//fn cairo_get_source (cr: *cairo_t) -> *cairo_pattern_t;

	pub fn set_antialias(&self, antialias : Antialias){
		unsafe{
			cairo_set_antialias(self.pointer, antialias)
		}
		self.check_state()
	}

	pub fn get_antialias(&self) -> Antialias{
		unsafe{
			cairo_get_antialias(self.pointer)
		}
	}

	pub fn set_dash(&self, dashes: &[f64], num_dashes: i32, offset: f64){
		unsafe{
			cairo_set_dash(self.pointer, dashes.as_ptr(), num_dashes, offset)
		}
		self.check_state(); //Possible invalid dashes value
	}

	pub fn get_dash_count(&self) -> i32{
		unsafe{
			cairo_get_dash_count(self.pointer)
		}
	}

	pub fn get_dash(&self) -> (Vec<f64>, f64){
		let dash_count = self.get_dash_count() as uint;
		let dashes : Vec<f64> = Vec::with_capacity(dash_count);
		let offset : Box<f64> = box 0.0;

		unsafe{
			let offset_ptr : *f64 = transmute(offset);
			cairo_get_dash(self.pointer, dashes.as_ptr(), offset_ptr);
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
			cairo_set_fill_rule(self.pointer, fill_rule);
		}
		self.check_state();
	}

	pub fn get_fill_rule(&self) -> FillRule{
		unsafe{
			cairo_get_fill_rule(self.pointer)
		}
	}

	pub fn set_line_cap(&self, arg: LineCap){
		unsafe{
			cairo_set_line_cap(self.pointer, arg)
		}
		self.check_state();
	}

	pub fn get_line_cap(&self) -> LineCap{
		unsafe{
			cairo_get_line_cap(self.pointer)
		}
	}

	pub fn set_line_join(&self, arg: LineJoin){
		unsafe{
			cairo_set_line_join(self.pointer, arg)
		}
		self.check_state();
	}

	pub fn get_line_join(&self) -> LineJoin{
		unsafe{
			cairo_get_line_join(self.pointer)
		}
	}

	pub fn set_line_width(&self, arg: f64){
		unsafe{
			cairo_set_line_width(self.pointer, arg)
		}
		self.check_state();
	}

	pub fn get_line_width(&self) -> f64{
		unsafe{
			cairo_get_line_width(self.pointer)
		}
	}

	pub fn set_miter_limit(&self, arg: f64){
		unsafe{
			cairo_set_miter_limit(self.pointer, arg)
		}
		self.check_state();
	}

	pub fn get_miter_limit(&self) -> f64{
		unsafe{
			cairo_get_miter_limit(self.pointer)
		}
	}

	pub fn set_tolerance(&self, arg: f64){
		unsafe{
			cairo_set_tolerance(self.pointer, arg)
		}
		self.check_state();
	}

	pub fn get_tolerance(&self) -> f64{
		unsafe{
			cairo_get_tolerance(self.pointer)
		}
	}

	pub fn clip(&self){
		unsafe{
			cairo_clip(self.pointer)
		}
	}

	pub fn clip_preserve(&self){
		unsafe{
			cairo_clip_preserve(self.pointer)
		}
	}

	pub fn clip_extents(&self) -> (f64,f64,f64,f64){
		unsafe{
			let x1 : *f64 = transmute(box 0.0);
			let y1 : *f64 = transmute(box 0.0);
			let x2 : *f64 = transmute(box 0.0);
			let y2 : *f64 = transmute(box 0.0);
			cairo_clip_extents(self.pointer, x1, y1, x2, y2);
			(*x1, *y1, *x2, *y2)
		}
	}

	pub fn in_clip(&self, x:f64, y:f64) -> bool{
		unsafe{
			if cairo_in_clip(self.pointer, x, y) != 0 {
				true
			}else{
				false
			}
		}
	}

	pub fn reset_clip(&self){
		unsafe{
			cairo_reset_clip(self.pointer)
		}
		self.check_state()
	}


	//fn cairo_rectangle_list_destroy (rectangle_list: *cairo_rectangle_list_t);

	//fn cairo_copy_clip_rectangle_list (cr: *cairo_t) -> *cairo_rectangle_list_t;

	pub fn fill(&self){
		unsafe{
			cairo_fill(self.pointer)
		}
	}

	pub fn fill_preserve(&self){
		unsafe{
			cairo_fill_preserve(self.pointer)
		}
	}

	pub fn fill_extents(&self) -> (f64,f64,f64,f64){
		unsafe{
			let x1 : *f64 = transmute(box 0.0);
			let y1 : *f64 = transmute(box 0.0);
			let x2 : *f64 = transmute(box 0.0);
			let y2 : *f64 = transmute(box 0.0);
			cairo_fill_extents(self.pointer, x1, y1, x2, y2);
			(*x1, *y1, *x2, *y2)
		}
	}

	pub fn in_fill(&self, x:f64, y:f64) -> bool{
		unsafe{
			if cairo_in_fill(self.pointer, x, y) != 0 {
				true
			}else{
				false
			}
		}
	}

	//fn cairo_mask (cr: *cairo_t, pattern: *cairo_pattern_t);

	//fn cairo_mask_surface (cr: *cairo_t, surface: *cairo_surface_t, surface_x: c_double, surface_y: c_double);

	//fn cairo_paint (cr: *cairo_t);

	//fn cairo_paint_with_alpha (cr: *cairo_t, alpha: c_double);

	pub fn stroke(&self){
		unsafe{
			cairo_stroke(self.pointer)
		}
	}

	pub fn stroke_preserve(&self){
		unsafe{
			cairo_stroke_preserve(self.pointer)
		}
	}

	pub fn stroke_extents(&self) -> (f64,f64,f64,f64){
		unsafe{
			let x1 : *f64 = transmute(box 0.0);
			let y1 : *f64 = transmute(box 0.0);
			let x2 : *f64 = transmute(box 0.0);
			let y2 : *f64 = transmute(box 0.0);
			cairo_stroke_extents(self.pointer, x1, y1, x2, y2);
			(*x1, *y1, *x2, *y2)
		}
	}

	pub fn in_stroke(&self, x:f64, y:f64) -> bool{
		unsafe{
			if cairo_in_stroke(self.pointer, x, y) != 0 {
				true
			}else{
				false
			}
		}
	}

	pub fn copy_page(&self){
		unsafe{
			cairo_copy_page(self.pointer)
		}
	}

	pub fn show_page(&self){
		unsafe{
			cairo_show_page(self.pointer)
		}
	}

	pub fn get_reference_count(&self) -> c_uint{
		unsafe{
			cairo_get_reference_count(self.pointer)
		}
	}

	//fn cairo_set_user_data (cr: *cairo_t, cairo_user_data_key_t: const *key, user_data: *c_void, destroy: cairo_destroy_func_t) -> Status;

	//fn cairo_get_user_data (cr: *cairo_t, cairo_user_data_key_t: const *key) -> *c_c_void;
}

#[link(name = "cairo")]
extern "C" {
	fn cairo_create (target: *cairo_surface_t) -> *cairo_t;

	fn cairo_reference (cr: *cairo_t) -> *cairo_t;

	fn cairo_destroy (cr: *cairo_t);

	fn cairo_status (cr: *cairo_t) -> Status;

	fn cairo_save (cr: *cairo_t);

	fn cairo_restore (cr: *cairo_t);

	fn cairo_get_target (cr: *cairo_t) -> *cairo_surface_t;

	fn cairo_push_group (cr: *cairo_t);

	fn cairo_push_group_with_content (cr: *cairo_t, content: cairo_content_t);

	fn cairo_pop_group (cr: *cairo_t) -> *cairo_pattern_t;

	fn cairo_pop_group_to_source (cr: *cairo_t);

	fn cairo_get_group_target (cr: *cairo_t) -> *cairo_surface_t;

	fn cairo_set_source_rgb (cr: *cairo_t, red: c_double, green: c_double, blue: c_double);

	fn cairo_set_source_rgba (cr: *cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);

	fn cairo_set_source (cr: *cairo_t, source: *cairo_pattern_t);

	fn cairo_set_source_surface (cr: *cairo_t, surface: *cairo_surface_t, x: c_double, y: c_double);

	fn cairo_get_source (cr: *cairo_t) -> *cairo_pattern_t;

	fn cairo_set_antialias (cr: *cairo_t, antialias: Antialias);

	fn cairo_get_antialias (cr: *cairo_t) -> Antialias;

	fn cairo_set_dash (cr: *cairo_t, dashes : *c_double, num_dashes: c_int, offset: c_double);

	fn cairo_get_dash_count (cr: *cairo_t) -> c_int;

	fn cairo_get_dash (cr: *cairo_t, dashes: *c_double, offset: *c_double);

	fn cairo_set_fill_rule (cr: *cairo_t, fill_rule: FillRule);

	fn cairo_get_fill_rule (cr: *cairo_t) -> FillRule;

	fn cairo_set_line_cap (cr: *cairo_t, line_cap: LineCap);

	fn cairo_get_line_cap (cr: *cairo_t) -> LineCap;

	fn cairo_set_line_join (cr: *cairo_t, line_join: LineJoin);

	fn cairo_get_line_join (cr: *cairo_t) -> LineJoin;

	fn cairo_set_line_width (cr: *cairo_t, width: c_double);

	fn cairo_get_line_width (cr: *cairo_t) -> c_double;

	fn cairo_set_miter_limit (cr: *cairo_t, limit: c_double);

	fn cairo_get_miter_limit (cr: *cairo_t) -> c_double;

	fn cairo_set_operator (cr: *cairo_t, op: cairo_operator_t);

	fn cairo_get_operator (cr: *cairo_t) -> cairo_operator_t;

	fn cairo_set_tolerance (cr: *cairo_t, tolerance: c_double);

	fn cairo_get_tolerance (cr: *cairo_t) -> c_double;

	fn cairo_clip (cr: *cairo_t);

	fn cairo_clip_preserve (cr: *cairo_t);

	fn cairo_clip_extents (cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);

	fn cairo_in_clip (cr: *cairo_t, x: c_double, y: c_double) -> c_int;

	fn cairo_reset_clip (cr: *cairo_t);

	fn cairo_rectangle_list_destroy (rectangle_list: *cairo_rectangle_list_t);

	fn cairo_copy_clip_rectangle_list (cr: *cairo_t) -> *cairo_rectangle_list_t;

	fn cairo_fill (cr: *cairo_t);

	fn cairo_fill_preserve (cr: *cairo_t);

	fn cairo_fill_extents (cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);

	fn cairo_in_fill (cr: *cairo_t, x: c_double, y: c_double) -> c_int;

	fn cairo_mask (cr: *cairo_t, pattern: *cairo_pattern_t);

	fn cairo_mask_surface (cr: *cairo_t, surface: *cairo_surface_t, surface_x: c_double, surface_y: c_double);

	fn cairo_paint (cr: *cairo_t);

	fn cairo_paint_with_alpha (cr: *cairo_t, alpha: c_double);

	fn cairo_stroke (cr: *cairo_t);

	fn cairo_stroke_preserve (cr: *cairo_t);

	fn cairo_stroke_extents (cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);

	fn cairo_in_stroke (cr: *cairo_t, x: c_double, y: c_double) -> c_int;

	fn cairo_copy_page (cr: *cairo_t);

	fn cairo_show_page (cr: *cairo_t);

	fn cairo_get_reference_count (cr: *cairo_t) -> c_uint;

	//fn cairo_set_user_data (cr: *cairo_t, cairo_user_data_key_t: /*const*/ *key, user_data: *c_void, destroy: cairo_destroy_func_t) -> Status;

	//fn cairo_get_user_data (cr: *cairo_t, cairo_user_data_key_t: /*const*/ *key) -> *c_void;

	//Error handling

	fn cairo_status_to_string (status : Status) -> *c_char;
}