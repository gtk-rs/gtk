// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A Box::new(with) a centered child

// FIXME: add missing methods (3.12)

use cast::{GTK_HEADER_BAR};
use ffi;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::{to_bool, to_gboolean};

/// GtkHeaderBar — A Box::new(with) a centered child
struct_Widget!(HeaderBar);

impl HeaderBar {
    pub fn new() -> Option<HeaderBar> {
        let tmp_pointer = unsafe { ffi::gtk_header_bar_new() };
        check_pointer!(tmp_pointer, HeaderBar)
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_header_bar_set_title(GTK_HEADER_BAR(self.pointer),
                                          title.to_glib_none().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(GTK_HEADER_BAR(self.pointer)))
        }
    }

    pub fn set_subtitle(&self, subtitle: &str) {
        unsafe {
            ffi::gtk_header_bar_set_subtitle(GTK_HEADER_BAR(self.pointer),
                                             subtitle.to_glib_none().0)
        }
    }

    pub fn get_subtitle(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(GTK_HEADER_BAR(self.pointer)))
        }
    }

    pub fn set_custom_title<T: ::WidgetTrait>(&self, title_widget: Option<&T>) {
        unsafe {
            ffi::gtk_header_bar_set_custom_title(GTK_HEADER_BAR(self.pointer),
                                                 unwrap_widget!(title_widget))
        }
    }

    pub fn get_custom_title<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_header_bar_get_custom_title(GTK_HEADER_BAR(self.pointer))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn pack_start<T: ::WidgetTrait>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_start(GTK_HEADER_BAR(self.pointer),
                                           child.unwrap_widget())
        }
    }

    pub fn pack_end<T: ::WidgetTrait>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_end(GTK_HEADER_BAR(self.pointer),
                                         child.unwrap_widget())
        }
    }

    pub fn is_show_close_button(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_header_bar_get_show_close_button(GTK_HEADER_BAR(self.pointer)))
        }
    }

    pub fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_close_button(GTK_HEADER_BAR(self.pointer),
                                                      to_gboolean(setting))
        }
    }
}

impl_drop!(HeaderBar);
impl_TraitWidget!(HeaderBar);

impl ::ContainerTrait for HeaderBar {}
