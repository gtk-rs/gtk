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

//! An interface for scrollable widgets

use gtk::cast::GTK_SCROLLABLE;
use gtk::{self, ffi};

/// GtkScrollable — An interface for scrollable widgets
pub trait ScrollableTrait: gtk::WidgetTrait {
    fn get_hadjustment(&self) -> gtk::Adjustment {
        unsafe {
            gtk::Adjustment::wrap_pointer(ffi::gtk_scrollable_get_hadjustment(GTK_SCROLLABLE(self.get_widget())))
        }
    }

    fn set_hadjustment(&mut self, hadjustment: gtk::Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(GTK_SCROLLABLE(self.get_widget()),
                                                hadjustment.get_pointer())
        }
    }

    fn get_vadjustment(&self) -> gtk::Adjustment {
        unsafe {
            gtk::Adjustment::wrap_pointer(ffi::gtk_scrollable_get_vadjustment(GTK_SCROLLABLE(self.get_widget())))
        }
    }

    fn set_vadjustment(&mut self, vadjustment: gtk::Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(GTK_SCROLLABLE(self.get_widget()),
                                                vadjustment.get_pointer())
        }
    }

    fn get_hscroll_policy(&self) -> gtk::ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_hscroll_policy(GTK_SCROLLABLE(self.get_widget()))
        }
    }

    fn set_hscroll_policy(&mut self, policy: gtk::ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(GTK_SCROLLABLE(self.get_widget()),
                                                   policy)
        }
    }

    fn get_vscroll_policy(&self) -> gtk::ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_vscroll_policy(GTK_SCROLLABLE(self.get_widget()))
        }
    }

    fn set_vscroll_policy(&mut self, policy: gtk::ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(GTK_SCROLLABLE(self.get_widget()),
                                                   policy)
        }
    }
}
