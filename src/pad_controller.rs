// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::IsA;
use gtk_sys;
use PadActionEntry;
use PadController;

pub trait PadControllerExtManual: 'static {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action_entries(&self, entries: &[PadActionEntry]);
}

impl<O: IsA<PadController>> PadControllerExtManual for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action_entries(&self, entries: &[PadActionEntry]) {
        let n_entries = entries.len() as i32;
        let entry_strings = entries
            .iter()
            .map(|e| (CString::new(&e.label), CString::new(&e.action_name)))
            .collect::<Vec<_>>();
        let entries = entries
            .iter()
            .zip(entry_strings.iter())
            .map(|(e, (label, action_name))| gtk_sys::GtkPadActionEntry {
                type_: e.type_.to_glib(),
                index: e.index,
                mode: e.mode,
                label: label.as_ptr() as *mut _,
                action_name: action_name.as_ptr() as *mut _,
            })
            .collect::<Vec<_>>();
        unsafe {
            gtk_sys::gtk_pad_controller_set_action_entries(
                self.as_ref().to_glib_none().0,
                entries.as_ptr(),
                n_entries,
            );
        }
    }
}
