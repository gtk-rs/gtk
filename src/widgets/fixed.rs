// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_int;

use cast::GTK_FIXED;
use ffi;

struct_Widget!(Fixed);

impl Fixed {
    pub fn new() -> Option<Fixed> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_fixed_new() };
        check_pointer!(tmp_pointer, Fixed)
    }

    pub fn put<T: ::WidgetTrait>(&self,
                             widget: &T,
                             x: i32,
                             y: i32) -> () {
        unsafe {
            ffi::gtk_fixed_put(GTK_FIXED(self.pointer), widget.unwrap_widget(), x as c_int, y as c_int);
        }
    }

    // FIXME: search a new name
    pub fn move_<T: ::WidgetTrait>(&self,
                              widget: &T,
                              x: i32,
                              y: i32) -> () {
        unsafe {
            ffi::gtk_fixed_move(GTK_FIXED(self.pointer), widget.unwrap_widget(), x as c_int, y as c_int);
        }
    }
}

impl_drop!(Fixed);
impl_TraitWidget!(Fixed);

impl ::ContainerTrait for Fixed {}
