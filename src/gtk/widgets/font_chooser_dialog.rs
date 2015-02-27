// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use gtk::{self, ffi};
use gtk::FFIWidget;
use gtk::cast::{GTK_WINDOW};
use std::ffi::CString;

struct_Widget!(FontChooserDialog);

impl FontChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<FontChooserDialog> {
        let tmp = unsafe {
            let c_str = CString::from_slice(title.as_bytes());

            ffi::gtk_font_chooser_dialog_new(c_str.as_ptr(),
                match parent {
                    Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                    None => GTK_WINDOW(::std::ptr::null_mut())
                }
            )
        };

        if tmp.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp))
        }
    }
}

impl_drop!(FontChooserDialog);
impl_TraitWidget!(FontChooserDialog);

impl gtk::ContainerTrait for FontChooserDialog {}
impl gtk::BinTrait for FontChooserDialog {}
impl gtk::WindowTrait for FontChooserDialog {}
impl gtk::DialogTrait for FontChooserDialog {}
impl gtk::FontChooserTrait for FontChooserDialog {}

impl_widget_events!(FontChooserDialog);
