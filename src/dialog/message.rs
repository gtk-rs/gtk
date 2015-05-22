// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::{GTK_MESSAGE_DIALOG, GTK_WINDOW};
use glib::translate::ToGlibPtr;

struct_Widget!(MessageDialog);

impl MessageDialog {
    pub fn new(parent: Option<&::Window>, flags: ::DialogFlags, _type: ::MessageType, buttons: ::ButtonsType) -> Option<MessageDialog> {
        let tmp_pointer = unsafe { ffi::gtk_message_dialog_new(match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => ::std::ptr::null_mut()
            }, flags, _type, buttons, ::std::ptr::null())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn new_with_markup(parent: Option<&::Window>, flags: ::DialogFlags, _type: ::MessageType, buttons: ::ButtonsType, markup: &str) -> Option<MessageDialog> {
        //gtk_message_dialog_new_with_markup
        match MessageDialog::new(parent, flags, _type, buttons) {
            Some(m) => {
                m.set_markup(markup);
                Some(m)
            }
            None => None
        }
    }

    pub fn set_markup(&self, markup: &str) -> () {
        unsafe {
            ffi::gtk_message_dialog_set_markup(GTK_MESSAGE_DIALOG(self.unwrap_widget()), markup.to_glib_none().0)
        }
    }

    pub fn get_message_area<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_message_dialog_get_message_area(GTK_MESSAGE_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(MessageDialog);
impl_TraitWidget!(MessageDialog);

impl ::ContainerTrait for MessageDialog {}
impl ::BinTrait for MessageDialog {}
impl ::WindowTrait for MessageDialog {}
impl ::DialogTrait for MessageDialog {}
