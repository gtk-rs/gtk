// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! An interface for scrollable widgets

use glib::translate::*;
use glib::types;
use ffi;

use adjustment::Adjustment;
use object::{Object, Upcast};

use ScrollablePolicy;

pub type Scrollable = Object<ffi::GtkScrollable>;

impl types::StaticType for Scrollable {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_scrollable_get_type()) }
    }
}

pub trait ScrollableExt {
    fn get_hadjustment(&self) -> Adjustment;
    fn set_hadjustment(&self, hadjustment: &Adjustment);
    fn get_vadjustment(&self) -> Adjustment;
    fn set_vadjustment(&self, vadjustment: &Adjustment);
    fn get_hscroll_policy(&self) -> ScrollablePolicy;
    fn set_hscroll_policy(&self, policy: ScrollablePolicy);
    fn get_vscroll_policy(&self) -> ScrollablePolicy;
    fn set_vscroll_policy(&self, policy: ScrollablePolicy);
}

impl<O: Upcast<Scrollable>> ScrollableExt for O {
    fn get_hadjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_hadjustment(self.upcast().to_glib_none().0))
        }
    }

    fn set_hadjustment(&self, hadjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(self.upcast().to_glib_none().0,
                hadjustment.to_glib_none().0)
        }
    }

    fn get_vadjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_vadjustment(self.upcast().to_glib_none().0))
        }
    }

    fn set_vadjustment(&self, vadjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(self.upcast().to_glib_none().0,
                vadjustment.to_glib_none().0)
        }
    }

    fn get_hscroll_policy(&self) -> ScrollablePolicy {
        unsafe { ffi::gtk_scrollable_get_hscroll_policy(self.upcast().to_glib_none().0) }
    }

    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(self.upcast().to_glib_none().0, policy)
        }
    }

    fn get_vscroll_policy(&self) -> ScrollablePolicy {
        unsafe { ffi::gtk_scrollable_get_vscroll_policy(self.upcast().to_glib_none().0) }
    }

    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe { ffi::gtk_scrollable_set_vscroll_policy(self.upcast().to_glib_none().0, policy) }
    }
}
