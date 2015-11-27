// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use cast::GTK_CELL_RENDERER_TEXT;

struct_Widget!(CellRendererText);

impl CellRendererText {
    pub fn new() -> Option<CellRendererText> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_cell_renderer_text_new() as *mut ffi::GtkWidget };

        check_pointer!(tmp_pointer, CellRendererText)
    }

    fn set_fixed_height_from_font(&self, number_of_rows: i32) {
        unsafe { ffi::gtk_cell_renderer_text_set_fixed_height_from_font(GTK_CELL_RENDERER_TEXT(self.pointer), number_of_rows) }
    }
}

impl_drop!(CellRendererText);
impl_TraitWidget!(CellRendererText);

impl ::CellRendererTrait for CellRendererText {}
