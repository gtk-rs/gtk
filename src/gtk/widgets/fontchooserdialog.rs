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

use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk::traits;
use gtk::cast::{GTK_WINDOW};
use gtk;

struct_Widget!(FontChooserDialog)

impl FontChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<FontChooserDialog> {
        let tmp = unsafe {
            title.with_c_str(|c_str| {
                ffi::gtk_font_chooser_dialog_new(c_str, match parent {
                    Some(ref p) => GTK_WINDOW(p.get_widget()),
                    None => GTK_WINDOW(::std::ptr::null_mut())})
                })
        };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }
}

impl_drop!(FontChooserDialog)
impl_TraitWidget!(FontChooserDialog)

impl traits::Container for FontChooserDialog {}
impl traits::Bin for FontChooserDialog {}
impl traits::Window for FontChooserDialog {}
impl traits::Dialog for FontChooserDialog {}
impl traits::FontChooser for FontChooserDialog {}