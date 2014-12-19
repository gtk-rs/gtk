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

//! GtkFileChooserWidget — A file chooser widget

use gtk::{mod, ffi};
use gtk::ffi::FFIWidget;

struct_Widget!(FileChooserWidget);

impl FileChooserWidget {
    pub fn new(action: gtk::FileChooserAction) -> Option<FileChooserWidget> {
        let tmp_pointer = unsafe { ffi::gtk_file_chooser_widget_new(action) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }
}

impl_drop!(FileChooserWidget);
impl_TraitWidget!(FileChooserWidget);

impl gtk::ContainerTrait for FileChooserWidget {}
impl gtk::BoxTrait for FileChooserWidget {}

impl_widget_events!(FileChooserWidget);