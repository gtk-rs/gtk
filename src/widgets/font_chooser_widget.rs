// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use FFIWidget;

struct_Widget!(FontChooserWidget);

impl FontChooserWidget {
    pub fn new() -> Option<FontChooserWidget> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_color_chooser_widget_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(FontChooserWidget);
impl_TraitWidget!(FontChooserWidget);

impl ::ContainerTrait for FontChooserWidget {}
impl ::BoxTrait for FontChooserWidget {}
