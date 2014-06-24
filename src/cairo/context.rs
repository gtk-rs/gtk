

use std::mem::transmute;

use cairo::ffi;
use cairo::types::{
	cairo_t,
	cairo_surface_t,
	cairo_pattern_t,
	cairo_operator_t,
	cairo_rectangle_list_t,
	cairo_content_t
};
use cairo::enums::{Status, status, Antialias, LineCap, LineJoin, FillRule};

pub struct Context{
	pointer: *cairo_t
}

impl Context{
	pub fn check_state(&self){
		let status = self.status();

		if status != status::Success{
			unsafe{
				fail!("Cairo crashed with: {}", ffi::cairo_status_to_string(status))
			}
		}
	}

	pub fn new (target: *cairo_surface_t) -> Context {
		unsafe {
			Context {
				pointer: ffi::cairo_create(target)
			}
		}
	}

	//fn ffi::cairo_reference (cr: *cairo_t) -> *cairo_t;

	//fn ffi::cairo_destroy (cr: *cairo_t);

	pub fn status (&self) -> Status {
		unsafe {
			ffi::cairo_status(self.pointer)
		}
	}

	pub fn save (&self) {
		unsafe {
			ffi::cairo_save(self.pointer)
		}
		self.check_state()
	}

	pub fn restore (&self) {
		unsafe{
			ffi::cairo_restore(self.pointer)
		}
		self.check_state()
	}

	//fn ffi::cairo_get_target (cr: *cairo_t) -> *cairo_surface_t;

	//fn ffi::cairo_push_group (cr: *cairo_t);

	//fn ffi::cairo_push_group_with_content (cr: *cairo_t, content: cairo_content_t);

	//fn ffi::cairo_pop_group (cr: *cairo_t) -> *cairo_pattern_t;

	//fn ffi::cairo_pop_group_to_source (cr: *cairo_t);

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

	//fn ffi::cairo_set_source_rgb (cr: *cairo_t, red: c_double, green: c_double, blue: c_double);

	//fn ffi::cairo_set_source_rgba (cr: *cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);

	//fn ffi::cairo_set_source (cr: *cairo_t, source: *cairo_pattern_t);

	//fn ffi::cairo_set_source_surface (cr: *cairo_t, surface: *cairo_surface_t, x: c_double, y: c_double);

	//fn ffi::cairo_get_source (cr: *cairo_t) -> *cairo_pattern_t;

	pub fn set_antialias(&self, antialias : Antialias){
		unsafe{
			ffi::cairo_set_antialias(self.pointer, antialias)
		}
		self.check_state()
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
		self.check_state(); //Possible invalid dashes value
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
		self.check_state();
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
		self.check_state();
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
		self.check_state();
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
		self.check_state();
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
		self.check_state();
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
		self.check_state();
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
		unsafe{
			let x1 : *f64 = transmute(box 0.0);
			let y1 : *f64 = transmute(box 0.0);
			let x2 : *f64 = transmute(box 0.0);
			let y2 : *f64 = transmute(box 0.0);
			ffi::cairo_clip_extents(self.pointer, x1, y1, x2, y2);
			(*x1, *y1, *x2, *y2)
		}
	}

	pub fn in_clip(&self, x:f64, y:f64) -> bool{
		unsafe{
			if ffi::cairo_in_clip(self.pointer, x, y) != 0 {
				true
			}else{
				false
			}
		}
	}

	pub fn reset_clip(&self){
		unsafe{
			ffi::cairo_reset_clip(self.pointer)
		}
		self.check_state()
	}


	//fn ffi::cairo_rectangle_list_destroy (rectangle_list: *cairo_rectangle_list_t);

	//fn ffi::cairo_copy_clip_rectangle_list (cr: *cairo_t) -> *cairo_rectangle_list_t;

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
		unsafe{
			let x1 : *f64 = transmute(box 0.0);
			let y1 : *f64 = transmute(box 0.0);
			let x2 : *f64 = transmute(box 0.0);
			let y2 : *f64 = transmute(box 0.0);
			ffi::cairo_fill_extents(self.pointer, x1, y1, x2, y2);
			(*x1, *y1, *x2, *y2)
		}
	}

	pub fn in_fill(&self, x:f64, y:f64) -> bool{
		unsafe{
			if ffi::cairo_in_fill(self.pointer, x, y) != 0 {
				true
			}else{
				false
			}
		}
	}

	//fn ffi::cairo_mask (cr: *cairo_t, pattern: *cairo_pattern_t);

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
		unsafe{
			let x1 : *f64 = transmute(box 0.0);
			let y1 : *f64 = transmute(box 0.0);
			let x2 : *f64 = transmute(box 0.0);
			let y2 : *f64 = transmute(box 0.0);
			ffi::cairo_stroke_extents(self.pointer, x1, y1, x2, y2);
			(*x1, *y1, *x2, *y2)
		}
	}

	pub fn in_stroke(&self, x:f64, y:f64) -> bool{
		unsafe{
			if ffi::cairo_in_stroke(self.pointer, x, y) != 0 {
				true
			}else{
				false
			}
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

	//Cairo Path

	//fn ffi::cairo_set_user_data (cr: *cairo_t, cairo_user_data_key_t: const *key, user_data: *c_void, destroy: cairo_destroy_func_t) -> Status;

	//fn ffi::cairo_get_user_data (cr: *cairo_t, cairo_user_data_key_t: const *key) -> *c_c_void;

	//fn ffi::cairo_copy_path(cr: *cairo_t) -> cairo_path_t;

   //fn ffi::cairo_copy_path_flat(cr: *cairo_t) -> cairo_path_t;

   /*pub fn append_path(&self, path: *cairo_path_t){
      unsafe{
         ffi::cairo_append_path(self.pointer, path)
      }
   }*/

   //fn ffi::cairo_copy_path(cr: *cairo_t) -> cairo_path_t;

   //fn ffi::cairo_copy_path_flat(cr: *cairo_t) -> cairo_path_t;

   //fn ffi::cairo_append_path(cr: *cairo_t, path: *cairo_path_t);

   //fn ffi::cairo_has_current_point(cr: *cairo_t);

   //fn ffi::cairo_get_current_point(cr: *cairo_t, x: *c_double, y: *c_double);

   pub fn new_path(&self){
      unsafe{
         ffi::cairo_new_path(self.pointer)
      }
   }

   pub fn new_sub_path(&self){
      unsafe{
         ffi::cairo_new_sub_path(self.pointer)
      }
   }

   pub fn close_path(&self){
      unsafe{
         ffi::cairo_close_path(self.pointer)
      }
   }

   pub fn arc(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64){
      unsafe{
         ffi::cairo_arc(self.pointer, xc, yc, radius, angle1, angle2)
      }
   }

   pub fn arc_negative(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64){
      unsafe{
         ffi::cairo_arc_negative(self.pointer, xc, yc, radius, angle1, angle2)
      }
   }

   pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64){
      unsafe{
         ffi::cairo_curve_to(self.pointer, x1, y1, x2, y2, x3, y3)
      }
   }

   pub fn line_to(&self, x: f64, y: f64){
      unsafe{
         ffi::cairo_line_to(self.pointer, x, y)
      }
   }

   pub fn move_to(&self, x: f64, y: f64){
      unsafe{
         ffi::cairo_move_to(self.pointer, x, y)
      }
   }

   pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64){
      unsafe{
         ffi::cairo_rectangle(self.pointer, x, y, width, height)
      }
   }

   pub fn text_path(&self, str : &str){
      unsafe{
         str.with_c_str(|str|{
            ffi::cairo_text_path(self.pointer, str)
         })
      }
   }

   //fn ffi::cairo_glyph_path(cr: *cairo_t, glyphs: *cairo_glyph_t, num_glyphs: int);

   pub fn rel_curve_to(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64){
      unsafe{
         ffi::cairo_rel_curve_to(self.pointer, dx1, dy1, dx2, dy2, dx3, dy3)
      }
   }

   pub fn rel_line_to(&self, dx: f64, dy: f64){
      unsafe{
         ffi::cairo_rel_line_to(self.pointer, dx, dy)
      }
   }

   pub fn rel_move_to(&self, dx: f64, dy: f64){
      unsafe{
         ffi::cairo_rel_move_to(self.pointer, dx, dy)
      }
   }

   //fn ffi::cairo_rel_curve_to(cr: *cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double);

   //fn ffi::cairo_rel_line_to(cr: *cairo_t, dx: c_double, dy: c_double);

   //fn ffi::cairo_rel_move_to(cr: *cairo_t, dx: c_double, dy: c_double);

   //fn ffi::cairo_path_extents(cr: *cairo_t, x1: *c_double, y1: *c_double, x2: *c_double, y2: *c_double);
}