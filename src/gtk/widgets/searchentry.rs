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



use ffi;
use gtk::traits::*;
/**
* SearchEntry — An entry which shows a search icon
*
* # Signal availables:
* * `search-changed` : Run Last
*/
struct_Widget!(SearchEntry)


impl SearchEntry {
    pub fn new() -> Option<SearchEntry> {
        let tmp_pointer = unsafe { ffi::gtk_search_entry_new() };
        check_pointer!(tmp_pointer, SearchEntry)
    }
}

impl_GtkWidget!(SearchEntry)


impl EntryTrait for SearchEntry {}
