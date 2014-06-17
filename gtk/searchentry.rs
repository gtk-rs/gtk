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

//! An entry which shows a search icon

use libc::{c_void};
use std::ptr;
use std::num::cast;

use traits::{GtkWidget, GtkEntry, Signal};
use ffi;
use std;
use std::owned;

/** 
* SearchEntry — An entry which shows a search icon
*
* # Signal availables:
* * `search-changed` : Run Last
*/
pub struct SearchEntry {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl SearchEntry {
    pub fn new() -> Option<SearchEntry> {
        let tmp_pointer = unsafe { ffi::gtk_search_entry_new() };
        check_pointer!(tmp_pointer, SearchEntry)
    }
}

impl_GtkWidget!(SearchEntry)
redirect_callback!(SearchEntry)
redirect_callback_widget!(SearchEntry)
struct_signal!(SearchEntry)
impl_signals!(SearchEntry)

impl GtkEntry for SearchEntry {}
