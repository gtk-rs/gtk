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

use pango::{ffi, Rectangle};
use libc::c_int;
use std::default::Default;

/// A structure specifying a transformation between user-space coordinates and device coordinates.
/// The transformation is given by:
/// 
/// x_device = x_user * matrix->xx + y_user * matrix->xy + matrix->x0;
/// y_device = x_user * matrix->yx + y_user * matrix->yy + matrix->y0;
pub type Matrix = ffi::PangoMatrix;

impl Matrix {
    pub fn new(xx: f64, xy: f64, yx: f64, yy: f64, x0: f64, y0: f64) -> Matrix {
        Matrix {
            xx: xx,
            xy: xy,
            yx: yx,
            yy: yy,
            x0: x0,
            y0: y0
        }
    }

    pub fn copy(&self) -> Matrix {
        Matrix {
            xx: self.xx,
            xy: self.xy,
            yx: self.yx,
            yy: self.yy,
            x0: self.x0,
            y0: self.y0
        }
    }

    pub fn translate(&mut self, t_x: f64, t_y: f64) {
        unsafe { ffi::pango_matrix_translate(self, t_x, t_y) }
    }

    pub fn scale(&mut self, scale_x: f64, scale_y: f64) {
        unsafe { ffi::pango_matrix_scale(self, scale_x, scale_y) }
    }

    pub fn rotate(&mut self, degrees: f64) {
        unsafe { ffi::pango_matrix_rotate(self, degrees) }
    }

    pub fn concat(&mut self, new_matrix: &Matrix) {
        unsafe { ffi::pango_matrix_concat(self, new_matrix) }
    }

    pub fn transform_point(&self, x: &mut f64, y: &mut f64) {
        unsafe { ffi::pango_matrix_transform_point(self, x, y) }
    }

    pub fn transform_distance(&self, dx: &mut f64, dy: &mut f64) {
        unsafe { ffi::pango_matrix_transform_distance(self, dx, dy) }
    }

    pub fn transform_rectangle(&self, rect: &mut Rectangle) {
        unsafe { ffi::pango_matrix_transform_rectangle(self, rect) }
    }

    pub fn transform_pixel_rectangle(&self, rect: &mut Rectangle) {
        unsafe { ffi::pango_matrix_transform_pixel_rectangle(self, rect) }
    }

    pub fn get_font_scale_factor(&mut self) -> f64 {
        unsafe { ffi::pango_matrix_get_font_scale_factor(self) }
    }
}

impl Default for Matrix {
    fn default() -> Matrix {
        Matrix {
            xx: 1.,
            xy: 0.,
            yx: 0.,
            yy: 1.,
            x0: 0.,
            y0: 0.
        }
    }
}