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

use gtk::traits::Widget;
use gtk::cast::GTK_SCROLLED_WINDOW;
use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk::PolicyType;

pub trait ScrolledWindow: Widget {
    fn set_policy(&self, h_scrollbar_policy: PolicyType, v_scrollbar_policy: PolicyType) {
        unsafe {
            ffi::gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(self.get_widget()), h_scrollbar_policy, v_scrollbar_policy);
        }
    }
}