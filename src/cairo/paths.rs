
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
