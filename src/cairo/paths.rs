
use std::mem::transmute;
use std::iter::Iterator;
use std::c_vec::CVec;
use cairo::enums::{
    PathMoveTo,
    PathLineTo,
    PathCurveTo,
    PathClosePath
};
use cairo::ffi::{
    cairo_path_t,
    cairo_path_data_header
};
use cairo::ffi;
use cairo;

pub struct Path(*mut cairo_path_t);

impl Path{
    pub fn get_ptr(&self) -> *mut cairo_path_t{
        let Path(ptr) = *self;
        ptr
    }

    pub fn ensure_status(&self){
        unsafe{
            let ptr: *mut cairo_path_t = self.get_ptr();
            (*ptr).status.ensure_valid()
        }
    }

    pub fn wrap(pointer: *mut cairo_path_t) -> Path{
        Path(pointer)
    }

    pub fn iter(&self) -> PathSegments {
        unsafe{
            let ptr: *mut cairo_path_t = self.get_ptr();
            let length = (*ptr).num_data as uint;
            let data_ptr = (*ptr).data;

            PathSegments {
                data: CVec::new(data_ptr, length),
                i: 0,
                num_data: length
            }
        }
    }
}

impl Drop for Path{
    fn drop(&mut self){
        unsafe{
            ffi::cairo_path_destroy(self.get_ptr());
        }
    }
}

#[deriving(Show)]
pub enum PathSegment{
    MoveTo((f64,f64)),
    LineTo((f64,f64)),
    CurveTo((f64, f64),(f64, f64),(f64, f64)),
    ClosePath
}

pub struct PathSegments<'a>{
    data: CVec<(f64, f64)>,
    i: uint,
    num_data: uint
}

impl<'a> Iterator<PathSegment> for PathSegments<'a>{
    fn next(&mut self) -> Option<PathSegment>{
        let i = self.i;

        if i >= self.num_data{
            return None;
        }

        let (data_type, length) = unsafe{
            let data_header: &cairo_path_data_header = transmute(self.data.get(i));
            (data_header.data_type, data_header.length)
        };

        self.i += length as uint;

        let ref data = self.data;

        Some(match data_type {
            PathMoveTo => MoveTo(*data.get(i+1).unwrap()),
            PathLineTo => LineTo(*data.get(i+1).unwrap()),
            PathCurveTo => CurveTo(*data.get(i+1).unwrap(), *data.get(i+2).unwrap(), *data.get(i+3).unwrap()),
            PathClosePath => ClosePath
        })
    }
}

impl cairo::Context{
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

    //fn ffi::cairo_path_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double);
}