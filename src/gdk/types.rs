use libc::{c_void};

#[repr(C)]
pub struct Rectangle { //FIXME should be just an alias to cairo_rectangle_int_t
    x: int,
    y: int,
    width: int,
    height: int
}

pub type Atom = *mut c_void;
pub type Key = u32;

#[repr(C)]
pub struct Screen;