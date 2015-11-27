// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_int;

use Orientation;
use cast::GTK_PANED;
use ffi;
use glib::to_gboolean;

/*
* # Available signals:
* * `accept-position` : Action
* * `cancel-position` : Action
* * `cycle-child-focus` : Action
* * `cycle-handle-focus` : Action
* * `move-handle` : Action
* * `toggle-handle-focus` : Action
*/
struct_Widget!(Paned);

impl Paned {
    pub fn new(orientation: Orientation) -> Option<Paned> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_paned_new(orientation) };
        check_pointer!(tmp_pointer, Paned)
    }

    pub fn add1<T: ::WidgetTrait>(&self, child: &T) -> () {
        unsafe {
            ffi::gtk_paned_add1(GTK_PANED(self.pointer), child.unwrap_widget())
        }
    }

    pub fn add2<T: ::WidgetTrait>(&self, child: &T) -> () {
        unsafe {
            ffi::gtk_paned_add2(GTK_PANED(self.pointer), child.unwrap_widget())
        }
    }

    pub fn pack1<T: ::WidgetTrait>(&self, child: &T, resize: bool, schrink: bool) -> () {
        unsafe {
            ffi::gtk_paned_pack1(GTK_PANED(self.pointer), child.unwrap_widget(),
                                 to_gboolean(resize), to_gboolean(schrink));
        }
    }

    pub fn pack2<T: ::WidgetTrait>(&self, child: &T, resize: bool, schrink: bool) -> () {
        unsafe {
            ffi::gtk_paned_pack2(GTK_PANED(self.pointer), child.unwrap_widget(),
                                 to_gboolean(resize), to_gboolean(schrink));
        }
    }

    pub fn set_position(&self, position: i32) -> () {
        unsafe {
            ffi::gtk_paned_set_position(GTK_PANED(self.pointer), position as c_int);
        }
    }

    pub fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_paned_get_position(GTK_PANED(self.pointer)) as i32
        }
    }

    pub fn get_handle_window(&self) -> ::Window {
        unsafe {
            ::FFIWidget::wrap_widget(
                ffi::gtk_paned_get_handle_window(GTK_PANED(self.pointer)) as *mut _)
        }
    }
}

impl_drop!(Paned);
impl_TraitWidget!(Paned);

impl ::ContainerTrait for Paned {}
