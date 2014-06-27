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
use gtk::traits;
use gtk::traits::Widget;
use gtk::cast::{GTK_WINDOW};
use gtk;

struct_Widget!(ColorChooserDialog)

impl ColorChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<ColorChooserDialog> {
        let tmp_pointer = unsafe {
            title.with_c_str(|c_str|{
                ffi::gtk_color_chooser_dialog_new(c_str, match parent {
                    Some(ref p) => GTK_WINDOW(p.get_widget()),
                    None => ::std::ptr::mut_null()
                })
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(traits::Widget::wrap(tmp_pointer))
        }
    }
}

impl_drop!(ColorChooserDialog)
impl_TraitWidget!(ColorChooserDialog)

impl traits::Container for ColorChooserDialog {}
impl traits::Bin for ColorChooserDialog {}
impl traits::Window for ColorChooserDialog {}
impl traits::Dialog for ColorChooserDialog {}
impl traits::ColorChooser for ColorChooserDialog {}