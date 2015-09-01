// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::{GTK_WINDOW};
use glib::translate::ToGlibPtr;

struct_Widget!(FontChooserDialog);

impl FontChooserDialog {
    pub fn new(title: &str, parent: Option<&::Window>) -> Option<FontChooserDialog> {
        let tmp = unsafe {
            ffi::gtk_font_chooser_dialog_new(title.to_glib_none().0,
                match parent {
                    Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                    None => GTK_WINDOW(::std::ptr::null_mut())
                }
            )
        };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp))
        }
    }
}

impl_drop!(FontChooserDialog);
impl_TraitWidget!(FontChooserDialog);

impl ::ContainerTrait for FontChooserDialog {}
impl ::BinTrait for FontChooserDialog {}
impl ::WindowTrait for FontChooserDialog {}
impl ::DialogTrait for FontChooserDialog {}
impl ::FontChooserTrait for FontChooserDialog {}
