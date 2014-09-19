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

#![macro_escape]

macro_rules! check_pointer(
    ($tmp_pointer:ident, $gtk_struct:ident) => (
        if $tmp_pointer.is_null() {
            None
        } else {
            unsafe{
                ::glib::ffi::g_object_ref($tmp_pointer as *mut ::glib::ffi::C_GObject);
            }

            Some($gtk_struct {
                pointer: $tmp_pointer
            })
        }
    );
)

macro_rules! struct_Widget(
    ($gtk_struct:ident) => (
        pub struct $gtk_struct {
            pointer: *mut ffi::C_GtkWidget
        }
    );
)

macro_rules! impl_TraitWidget(
    ($gtk_struct:ident) => (
        impl ::gtk::ffi::FFIWidget for $gtk_struct {
            fn get_widget(&self) -> *mut ffi::C_GtkWidget {
                self.pointer
            }

            fn wrap(widget: *mut ffi::C_GtkWidget) -> $gtk_struct {
                unsafe{
                    ::glib::ffi::g_object_ref(widget as *mut ::glib::ffi::C_GObject);
                }

                $gtk_struct {
                    pointer: widget
                }
            }
        }

        impl ::gtk::traits::Widget for $gtk_struct {}
    );
)

macro_rules! impl_drop(
    ($gtk_struct:ident) => (
        impl Drop for $gtk_struct {
            fn drop(&mut self) {
                unsafe {
                    ::glib::ffi::g_object_unref(self.pointer as *mut ::glib::ffi::C_GObject);
                }
            }
        }

        impl Clone for $gtk_struct {
            fn clone(&self) -> $gtk_struct {
                let pointer = unsafe {
                    ::glib::ffi::g_object_ref(self.pointer as *mut ::glib::ffi::C_GObject)
                };

                $gtk_struct {
                    pointer: pointer as *mut ffi::C_GtkWidget
                }
            }
        }
    );
)

// Useful for function wich take a valid widget or NULL for a default widget
// takes an option<&trait::Widget> and return the c widget pointer or ptr::null()
macro_rules! get_widget(
    ($w:ident) => (
        match $w {
            Some(ref _w) => _w.get_widget(),
            None => ::std::ptr::null_mut()
        };
    );
)
