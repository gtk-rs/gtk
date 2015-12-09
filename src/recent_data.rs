// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_char;
use glib::translate::*;
use ffi;

pub struct RecentData {
    display_name: String,
    description: String,
    mime_type: String,
    app_name: String,
    app_exec: String,
    groups: Vec<String>,
    is_private: bool,
}

impl <'a> ToGlibPtr<'a, *mut ffi::GtkRecentData> for RecentData {
    type Storage = (Box<ffi::GtkRecentData>,
                    [Stash<'a, *mut c_char, String>; 5],
                    Stash<'a, *mut *mut c_char, [String]>);

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GtkRecentData, Self> {
        let display_name = self.display_name.to_glib_none();
        let description = self.description.to_glib_none();
        let mime_type = self.mime_type.to_glib_none();
        let app_name = self.app_name.to_glib_none();
        let app_exec = self.app_exec.to_glib_none();
        let groups = self.groups.to_glib_none();

        let mut data = Box::new(ffi::GtkRecentData {
            display_name: display_name.0,
            description: description.0,
            mime_type: mime_type.0,
            app_name: app_name.0,
            app_exec: app_exec.0,
            groups: groups.0,
            is_private: self.is_private.to_glib(),
        });

        Stash(&mut *data, (data, [display_name, description, mime_type,
                                  app_name, app_exec], groups))
    }
}
