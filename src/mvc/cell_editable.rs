// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Interface for widgets which can are used for editing cells

use ffi;
use cast::GTK_CELL_EDITABLE;

pub trait CellEditableTrait : ::WidgetTrait {
    fn editing_done(&self) {
        unsafe { ffi::gtk_cell_editable_editing_done(GTK_CELL_EDITABLE(self.unwrap_widget())) }
    }

    fn remove_widget(&self) {
        unsafe { ffi::gtk_cell_editable_remove_widget(GTK_CELL_EDITABLE(self.unwrap_widget())) }
    }
}