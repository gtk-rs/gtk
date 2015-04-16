// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use libc::c_char;
use glib::translate::{ToGlib, ToGlibPtr, Stash};

pub struct RecentData {
    display_name: String,
    description: String,
    mime_type: String,
    app_name: String,
    app_exec: String,
    groups: Vec<String>,
    is_private: bool
}

impl <'a> ToGlibPtr<'a, *mut ffi::C_GtkRecentData> for RecentData {
    type Storage = (Box<ffi::C_GtkRecentData>,
                    [Stash<'a, *const c_char, String>; 5],
                    Stash<'a, *mut *const c_char, Vec<String>>);

    fn borrow_to_glib(&'a self)
        -> Stash<*mut ffi::C_GtkRecentData, RecentData> {
        let display_name = self.display_name.borrow_to_glib();
        let description = self.description.borrow_to_glib();
        let mime_type = self.mime_type.borrow_to_glib();
        let app_name = self.app_name.borrow_to_glib();
        let app_exec = self.app_exec.borrow_to_glib();
        let groups = self.groups.borrow_to_glib();

        let mut data = Box::new(ffi::C_GtkRecentData {
            display_name: display_name.0 as *mut c_char,
            description: description.0 as *mut c_char,
            mime_type: mime_type.0 as *mut c_char,
            app_name: app_name.0 as *mut c_char,
            app_exec: app_exec.0 as *mut c_char,
            groups: groups.0 as *mut *mut c_char,
            is_private: self.is_private.to_glib(),
        });

        Stash(&mut *data, (data, [display_name, description, mime_type,
                                            app_name, app_exec], groups))
    }
}
