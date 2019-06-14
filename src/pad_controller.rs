// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::IsA;
use gtk_sys;
use PadActionEntry;
use PadController;

pub trait PadControllerExtManual: 'static {
    fn set_action_entries(&self, entries: &[PadActionEntry]);
}

impl<O: IsA<PadController>> PadControllerExtManual for O {
    fn set_action_entries(&self, entries: &[PadActionEntry]) {
        let n_entries = entries.len() as i32;
        let entry_strings = entries
            .iter()
            .map(|e| (e.label.to_glib_none(), e.action_name.to_glib_none()))
            .collect::<Vec<(Stash<_, _>, Stash<_, _>)>>();
        let entries = entries
            .iter()
            .zip(entry_strings.iter())
            .map(|(e, (label, action_name))| gtk_sys::GtkPadActionEntry {
                type_: e.type_.to_glib(),
                index: e.index,
                mode: e.mode,
                label: label.0,
                action_name: action_name.0,
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
