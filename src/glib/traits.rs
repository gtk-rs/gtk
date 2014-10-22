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

use glib::ffi;
use gtk::signals::Signal;

pub trait FFIGObject {
    fn get_gobject(&self) -> *mut ffi::C_GObject;
}

// pub trait Connect<T>: FFIGObject {
//     fn connect<'a>(&self, signal: Box<Signal<'a>>) -> () {
//         use std::mem::transmute;

//         unsafe {
//             let signal_name     = signal.get_signal_name().to_string();
//             let trampoline      = signal.get_trampoline();

//             let user_data_ptr   = transmute(box signal);

//             signal_name.replace("_", "-").with_c_str(|signal_name| {
//                 ffi::glue_signal_connect(
//                     self.get_gobject(),
//                     signal_name,
//                     Some(trampoline),
//                     user_data_ptr
//                 )
//             });
//         }
//     }
// }

pub trait Connect<'a, T: Signal<'a>>: FFIGObject {
    fn connect<'a>(&self, signal: Box<T>) -> () {
        use std::mem::transmute;

        let signal = signal as Box<Signal<'a>>;

        unsafe {
            let signal_name     = signal.get_signal_name().to_string();
            let trampoline      = signal.get_trampoline();

            let user_data_ptr   = transmute(box signal);

            signal_name.replace("_", "-").with_c_str(|signal_name| {
                ffi::glue_signal_connect(
                    self.get_gobject(),
                    signal_name,
                    Some(trampoline),
                    user_data_ptr
                )
            });
        }
    }
}
