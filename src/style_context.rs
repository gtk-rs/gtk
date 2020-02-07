// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use libc::c_void;
use pango::FontDescription;
use pango_sys::PangoFontDescription;
use StateFlags;
use StyleContext;

pub trait StyleContextExtManual: 'static {
    fn get_font(&self, state: StateFlags) -> Option<FontDescription>;
}

impl<O: IsA<StyleContext>> StyleContextExtManual for O {
    fn get_font(&self, state: StateFlags) -> Option<FontDescription> {
        unsafe {
            let font_description_ptr: *mut PangoFontDescription = std::ptr::null_mut();
            gtk_sys::gtk_style_context_get(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                b"font\0".as_ptr() as *const _,
                &font_description_ptr,
                std::ptr::null::<c_void>(),
            );
            from_glib_full(font_description_ptr)
        }
    }
}
