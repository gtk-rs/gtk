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
use glib::translate::{ToGlibPtr, ToTmp};

struct_Widget!(ColorChooserDialog);

impl ColorChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<ColorChooserDialog> {
        let tmp_pointer = unsafe {
            let mut tmp_title = title.to_tmp_for_borrow();
            ffi::gtk_color_chooser_dialog_new(tmp_title.to_glib_ptr(),
                match parent {
                    Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                    None => ::std::ptr::null_mut()
                }
            )
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(ColorChooserDialog);
impl_TraitWidget!(ColorChooserDialog);

impl gtk::ContainerTrait for ColorChooserDialog {}
impl gtk::BinTrait for ColorChooserDialog {}
impl gtk::WindowTrait for ColorChooserDialog {}
impl gtk::DialogTrait for ColorChooserDialog {}
impl gtk::ColorChooserTrait for ColorChooserDialog {}

impl_widget_events!(ColorChooserDialog);
