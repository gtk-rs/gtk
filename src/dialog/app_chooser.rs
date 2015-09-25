// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::{GTK_WINDOW, GTK_APP_CHOOSER_DIALOG};
use glib::translate::{from_glib_none, ToGlibPtr};

struct_Widget!(AppChooserDialog);

impl AppChooserDialog {
    pub fn new_for_content_type(parent: Option<&::Window>, flags: ::DialogFlags, content_type: &str) -> Option<AppChooserDialog> {
        let tmp_pointer = unsafe {
            let parent = match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => ::std::ptr::null_mut()
            };

            ffi::gtk_app_chooser_dialog_new_for_content_type(parent, flags,
                                                             content_type.to_glib_none().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn widget<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_dialog_get_widget(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_heading(&self, heading: &str) -> () {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget()), heading.to_glib_none().0)
        }
    }

    pub fn get_heading(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_app_chooser_dialog_get_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())))
        }
    }
}

impl_drop!(AppChooserDialog);
impl_TraitWidget!(AppChooserDialog);

impl ::ContainerTrait for AppChooserDialog {}
impl ::BinTrait for AppChooserDialog {}
impl ::WindowTrait for AppChooserDialog {}
impl ::DialogTrait for AppChooserDialog {}
impl ::AppChooserTrait for AppChooserDialog {}
