use ffi;
use glib::translate::*;

pub struct PageRange(ffi::GtkPageRange);

impl PageRange {
    pub fn new(start: i32, end: i32) -> PageRange {
        skip_assert_initialized!();
        PageRange(ffi::GtkPageRange {
            start,
            end,
        })
    }

    pub fn get_start(&self) -> i32 {
        self.0.start
    }

    pub fn get_end(&self) -> i32 {
        self.0.end
    }
}

#[doc(hidden)]
impl ToGlib for PageRange {
    type GlibType = ffi::GtkPageRange;

    fn to_glib(&self) -> ffi::GtkPageRange {
        self.0
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkPageRange> for PageRange {
    type Storage = Box<ffi::GtkPageRange>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GtkPageRange, Self> {
        let page_range = Box::new(self.0);
        Stash(&*page_range, page_range)
    }
}
