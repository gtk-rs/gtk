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

use ffi;
use FFIWidget;
use cast::{GTK_WINDOW};
use glib::translate::ToGlibPtr;

struct_Widget!(FontChooserDialog);

impl FontChooserDialog {
    pub fn new(title: &str, parent: Option<::Window>) -> Option<FontChooserDialog> {
        let tmp = unsafe {
            ffi::gtk_font_chooser_dialog_new(title.borrow_to_glib().0,
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

impl_widget_events!(FontChooserDialog);
