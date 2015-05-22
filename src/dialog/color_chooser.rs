// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;
use cast::{GTK_WINDOW};
use glib::translate::ToGlibPtr;

struct_Widget!(ColorChooserDialog);

impl ColorChooserDialog {
    pub fn new(title: &str, parent: Option<&::Window>) -> Option<ColorChooserDialog> {
        let tmp_pointer = unsafe {
            ffi::gtk_color_chooser_dialog_new(title.to_glib_none().0,
                match parent {
                    Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                    None => ::std::ptr::null_mut()
                }
            )
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(ColorChooserDialog);
impl_TraitWidget!(ColorChooserDialog);

impl ::ContainerTrait for ColorChooserDialog {}
impl ::BinTrait for ColorChooserDialog {}
impl ::WindowTrait for ColorChooserDialog {}
impl ::DialogTrait for ColorChooserDialog {}
impl ::ColorChooserTrait for ColorChooserDialog {}
