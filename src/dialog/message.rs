// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use Box;
use widgets::widget::Widget;
use window::Window;

use {
    ButtonsType,
    DialogFlags,
    MessageType,
};

pub type MessageDialog = Object<ffi::GtkMessageDialog>;

impl MessageDialog {
    pub fn new<W: Upcast<Window>>(parent: Option<&W>, flags: DialogFlags, type_: MessageType,
            buttons: ButtonsType) -> MessageDialog {
        unsafe { 
            Widget::from_glib_none(ffi::gtk_message_dialog_new(
                parent.map(|w| w.upcast()).to_glib_none().0, flags, type_, buttons, ptr::null()))
                .downcast_unchecked()
        }
    }

    pub fn new_with_markup<W: Upcast<Window>>(parent: Option<&W>, flags: DialogFlags,
            type_: MessageType, buttons: ButtonsType, markup: &str) -> MessageDialog {
        //gtk_message_dialog_new_with_markup
        let dlg = MessageDialog::new(parent, flags, type_, buttons);
        dlg.set_markup(markup);
        dlg
    }

    pub fn set_markup(&self, markup: &str) -> () {
        unsafe {
            ffi::gtk_message_dialog_set_markup(self.to_glib_none().0, markup.to_glib_none().0)
        }
    }

    pub fn get_message_area(&self) -> Box {
        unsafe {
            Widget::from_glib_none(ffi::gtk_message_dialog_get_message_area(self.to_glib_none().0))
                .downcast_unchecked()
        }
    }
}

impl types::StaticType for MessageDialog {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_message_dialog_get_type()) }
    }
}

unsafe impl Upcast<Widget> for MessageDialog { }
unsafe impl Upcast<::Container> for MessageDialog { }
unsafe impl Upcast<::Bin> for MessageDialog { }
unsafe impl Upcast<::Window> for MessageDialog { }
unsafe impl Upcast<::Dialog> for MessageDialog { }
unsafe impl Upcast<::Buildable> for MessageDialog { }
