// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A single line text entry field

use ffi;

/// Entry — A single line text entry field
/*
* # Availables signals :
* * `activate` : Action
* * `backspace` : Action
* * `copy-clipboard` : Action
* * `cut-clipboard` : Action
* * `delete-from-cursor` : Action
* * `icon-press` : Run Last
* * `icon-release` : Run Last
* * `insert-at-cursor` : Action
* * `move-cursor` : Action
* * `paste-clipboard` : Action
* * `populate-popup` : Run Last
* * `preedit-changed` : Action
* * `toggle-overwrite` : Action
*/
struct_Widget!(Entry);

impl Entry {
    pub fn new() -> Option<Entry> {
        let tmp_pointer = unsafe { ffi::gtk_entry_new() };
        check_pointer!(tmp_pointer, Entry)
    }

    pub fn new_with_buffer(buffer: &::EntryBuffer) -> Option<Entry> {
        let tmp_pointer = unsafe { ffi::gtk_entry_new_with_buffer(buffer.unwrap_pointer()) };
        check_pointer!(tmp_pointer, Entry)
    }
}

impl_drop!(Entry);
impl_TraitWidget!(Entry);

impl ::EntryTrait for Entry {}
impl ::EditableTrait for Entry {}
