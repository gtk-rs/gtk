// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_SCROLLABLE;
use ffi;

pub trait ScrollableTrait: ::WidgetTrait {
    fn get_hadjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_scrollable_get_hadjustment(GTK_SCROLLABLE(self.unwrap_widget())))
        }
    }

    fn set_hadjustment(&self, hadjustment: ::Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(GTK_SCROLLABLE(self.unwrap_widget()),
                                                hadjustment.unwrap_pointer())
        }
    }

    fn get_vadjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_scrollable_get_vadjustment(GTK_SCROLLABLE(self.unwrap_widget())))
        }
    }

    fn set_vadjustment(&self, vadjustment: ::Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(GTK_SCROLLABLE(self.unwrap_widget()),
                                                vadjustment.unwrap_pointer())
        }
    }

    fn get_hscroll_policy(&self) -> ::ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_hscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()))
        }
    }

    fn set_hscroll_policy(&self, policy: ::ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()),
                                                   policy)
        }
    }

    fn get_vscroll_policy(&self) -> ::ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_vscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()))
        }
    }

    fn set_vscroll_policy(&self, policy: ::ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()),
                                                   policy)
        }
    }
}
