
use libc::{c_double};

use cairo::ffi;

#[repr(C)]
pub struct Matrix{
    xx: c_double,
    yx: c_double,

    xy: c_double,
    yy: c_double,

    x0: c_double,
    y0: c_double,
}

impl Matrix{
	pub fn null() -> Matrix{
		Matrix{
			xx: 0.0,
			yx: 0.0,
			xy: 0.0,
			yy: 0.0,
			x0: 0.0,
			y0: 0.0
		}
	}

	pub fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix{
		let mut matrix = Matrix::null();
		matrix.init(xx, yx, xy, yy, x0, y0);
		matrix
	}

	pub fn multiply(left: &Matrix, right: &Matrix) -> Matrix{
		let mut matrix = Matrix::null();
		unsafe{
			ffi::cairo_matrix_multiply(&mut matrix, left, right);
		}
		matrix
	}

	pub fn identity() -> Matrix{
		let mut matrix = Matrix::null();
		unsafe{
			ffi::cairo_matrix_init_identity(&mut matrix);
		}
		matrix
	}

	pub fn init(&mut self, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64){
		unsafe{
			ffi::cairo_matrix_init(self, xx, yx, xy, yy, x0, y0)
		}
	}

	pub fn translate(&mut self, tx: f64, ty: f64){
		unsafe{
			ffi::cairo_matrix_translate(self, tx, ty)
		}
	}

	pub fn scale(&mut self, sx: f64, sy: f64){
		unsafe{
			ffi::cairo_matrix_scale(self, sx, sy)
		}
	}

	pub fn rotate(&mut self, angle: f64){
		unsafe{
			ffi::cairo_matrix_rotate(self, angle)
		}
	}

	pub fn invert(&mut self){
		let result = unsafe{
			ffi::cairo_matrix_invert(self)
		};
		result.ensure_valid();
	}

	pub fn transform_distance(&self, _dx: f64, _dy: f64) -> (f64, f64){
		let mut dx = _dx;
		let mut dy = _dy;

		unsafe{
			ffi::cairo_matrix_transform_distance(self, &mut dx, &mut dy);
		}
		(dx, dy)
	}

	pub fn transform_point(&self, _x: f64, _y: f64) -> (f64, f64){
		let mut x = _x;
		let mut y = _y;

		unsafe{
			ffi::cairo_matrix_transform_point(self, &mut x, &mut y);
		}
		(x, y)
	}
}