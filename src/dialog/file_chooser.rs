// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::ToGlibPtr;
use ffi;
use FFIWidget;
use cast::{GTK_WINDOW};
use DialogButtons;

struct_Widget!(FileChooserDialog);

impl FileChooserDialog {
    pub fn new<T: DialogButtons>(title: &str, parent: Option<&::Window>,
                                 action: ::FileChooserAction, buttons: T) -> FileChooserDialog {
        unsafe {
            let parent = match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => GTK_WINDOW(::std::ptr::null_mut())
            };

            ::FFIWidget::wrap_widget(
                buttons.invoke3(
                    ffi::gtk_file_chooser_dialog_new,
                    title.to_glib_none().0,
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
