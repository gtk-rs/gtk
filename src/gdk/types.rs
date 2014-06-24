use libc::{c_void};

pub struct Rectangle { //FIXME should be just an alias to cairo_rectangle_int_t
    x: int,
    y: int,
    width: int,
    height: int
}

pub type Atom = *c_void;

pub struct Screen;