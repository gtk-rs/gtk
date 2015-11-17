// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkFileChooserWidget — A file chooser widget

use ffi;
use FFIWidget;

struct_Widget!(FileChooserWidget);

impl FileChooserWidget {
    pub fn new(action: ::FileChooserAction) -> Option<FileChooserWidget> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_file_chooser_widget_new(action) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(FileChooserWidget);
impl_TraitWidget!(FileChooserWidget);

impl ::ContainerTrait for FileChooserWidget {}
impl ::BoxTrait for FileChooserWidget {}
