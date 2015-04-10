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
