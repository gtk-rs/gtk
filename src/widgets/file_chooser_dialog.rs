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

use glib::translate::ToGlibPtr;
use ffi;
use FFIWidget;
use cast::{GTK_WINDOW};
use DialogButtons;

struct_Widget!(FileChooserDialog);

impl FileChooserDialog {
    pub fn new<T: DialogButtons>(title: &str, parent: Option<::Window>,
                                 action: ::FileChooserAction, buttons: T) -> FileChooserDialog {
        unsafe {
            let parent = match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => GTK_WINDOW(::std::ptr::null_mut())
            };

            ::FFIWidget::wrap_widget(
                buttons.invoke3(
                    ffi::gtk_file_chooser_dialog_new,
                    title.borrow_to_glib().0,
                    parent,
                    action))

        }
    }
}

impl_drop!(FileChooserDialog);
impl_TraitWidget!(FileChooserDialog);

impl ::ContainerTrait for FileChooserDialog {}
impl ::BinTrait for FileChooserDialog {}
impl ::WindowTrait for FileChooserDialog {}
impl ::DialogTrait for FileChooserDialog {}
impl ::FileChooserTrait for FileChooserDialog {}

impl_widget_events!(FileChooserDialog);
