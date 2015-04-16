// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Create bars of buttons and other widgets

use libc::c_int;

use ffi;
use glib::{to_bool, to_gboolean};
use cast::{GTK_TOOLBAR, GTK_TOOLITEM};
use {IconSize, ReliefStyle, ToolbarStyle};

/// Toolbar — Create bars of buttons and other widgets
/*
* # Availables signals :
* * `focus-home-or-end` : Action
* * `orientation-changed` : Run First
* * `popup-context-menu` : Run Last
* * `style-changed` : Run First
*/
struct_Widget!(Toolbar);

impl Toolbar {
    pub fn new() -> Option<Toolbar> {
        let tmp_pointer = unsafe { ffi::gtk_toolbar_new() };
        check_pointer!(tmp_pointer, Toolbar)
    }

    pub fn insert<T: ::ToolItemTrait>(&self,
                                  item: &T,
                                  pos: i32) -> () {
        unsafe {
            ffi::gtk_toolbar_insert(GTK_TOOLBAR(self.pointer), GTK_TOOLITEM(item.unwrap_widget()), pos as c_int)
        }
    }

    pub fn item_index<T: ::ToolItemTrait>(&self, item: &T) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(GTK_TOOLBAR(self.pointer), GTK_TOOLITEM(item.unwrap_widget())) as i32
        }
    }

    pub fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_n_items(GTK_TOOLBAR(self.pointer)) as i32
        }
    }

    pub fn get_nth_item(&self, n: i32) -> Option<::ToolItem> {
        unsafe {
            let tmp_pointer = ffi::gtk_toolbar_get_nth_item(GTK_TOOLBAR(self.pointer), n as c_int) as *mut ffi::C_GtkWidget;
            if tmp_pointer.is_null() {
                None
            } else {
                Some(::FFIWidget::wrap_widget(tmp_pointer))
            }
        }
    }

    pub fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_drop_index(GTK_TOOLBAR(self.pointer), x as c_int, y as c_int) as i32
        }
    }

    pub fn set_drop_highlight_item<T: ::ToolItemTrait>(&self, item: &T, index: i32) -> () {
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(GTK_TOOLBAR(self.pointer), GTK_TOOLITEM(item.unwrap_widget()), index as c_int);
        }
    }

    pub fn set_show_arrow(&self, show_arrow: bool) -> () {
        unsafe { ffi::gtk_toolbar_set_show_arrow(GTK_TOOLBAR(self.pointer), to_gboolean(show_arrow)); }
    }

    pub fn unset_icon_size(&self) -> () {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn get_show_arrow(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toolbar_get_show_arrow(GTK_TOOLBAR(self.pointer))) }
    }

    pub fn get_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_toolbar_get_style(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn get_icon_size(&self) -> IconSize {
        unsafe {
            ffi::gtk_toolbar_get_icon_size(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_toolbar_get_relief_style(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn set_style(&self, style: ToolbarStyle) -> () {
        unsafe {
            ffi::gtk_toolbar_set_style(GTK_TOOLBAR(self.pointer), style);
        }
    }

    pub fn set_icon_size(&self, icon_size: IconSize) -> () {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(GTK_TOOLBAR(self.pointer), icon_size);
        }
    }

    pub fn unset_style(&self) -> () {
        unsafe {
            ffi::gtk_toolbar_unset_style(GTK_TOOLBAR(self.pointer));
        }
    }
}

impl_drop!(Toolbar);
impl_TraitWidget!(Toolbar);

impl ::ContainerTrait for Toolbar {}
impl ::ToolShellTrait for Toolbar {}
impl ::OrientableTrait for Toolbar {}

impl_widget_events!(Toolbar);
