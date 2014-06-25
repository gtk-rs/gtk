
use libc::{c_int, c_uint, c_char, c_double};
use std::mem::transmute;
use std::iter::Iterator;
use cairo::enums::{
    PathDataType,
    PathMoveTo,
    PathLineTo,
    PathCurveTo,
    PathClosePath
};
use cairo::types::{
    cairo_path_t,
    cairo_path_data_header
};

pub struct Path{
    pub pointer : *cairo_path_t
}

impl Path{
    pub fn wrap(pointer: *cairo_path_t) -> Path{
        Path{
            pointer: pointer
        }
    }

    pub fn iter(&self) -> PathIterator{
        unsafe{
            let length = (*self.pointer).num_data as uint;
            let data_ptr = (*self.pointer).data;

            PathIterator {
                data: Vec::from_raw_parts(length, length, data_ptr),
                i: 0,
                num_data: length
            }
        }
    }
}

pub enum PathSegment{
    MoveTo((f64,f64)),
    LineTo((f64,f64)),
    CurveTo((f64, f64),(f64, f64),(f64, f64)),
    ClosePath
}

pub struct PathIterator{
    data: Vec<(f64, f64)>,
    i: uint,
    num_data: uint
}

impl Iterator<PathSegment> for PathIterator{
    fn next(&mut self) -> Option<PathSegment>{
        let i = self.i;

        if(i >= self.num_data){
            return None;
        }

        let (data_type, length) = unsafe{
            let data_header: &cairo_path_data_header = transmute(self.data.get(i));
            (data_header.data_type, data_header.length)
        };

        self.i += (length as uint);

        Some(match data_type {
            PathMoveTo => MoveTo(*self.data.get(i+1)),
            PathLineTo => LineTo(*self.data.get(i+1)),
            PathCurveTo => CurveTo(*self.data.get(i+1), *self.data.get(i+2), *self.data.get(i+3)),
            PathClosePath => ClosePath
        })
    }
}