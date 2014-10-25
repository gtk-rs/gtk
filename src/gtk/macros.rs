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
    ($tmp_pointer:ident, $gtk_struct:ident) => ( check_pointer!($tmp_pointer, $gtk_struct, G_OBJECT) );
    ($tmp_pointer:ident, $gtk_struct:ident, $cast_fn:ident) => (
        if $tmp_pointer.is_null() {
            None
        } else {
            unsafe{
                ::glib::ffi::g_object_ref(::gtk::cast::$cast_fn($tmp_pointer));
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
                    ::glib::ffi::g_object_ref(::gtk::ffi::cast_GtkObject(widget));
                }

                $gtk_struct {
                    pointer: widget
                }
            }
        }

        impl ::gtk::traits::Widget for $gtk_struct {}

        impl ::glib::traits::FFIGObject for $gtk_struct {
            fn get_gobject(&self) -> *mut ::glib::ffi::C_GObject {
                use gtk::ffi::FFIWidget;
                ::gtk::cast::G_OBJECT(self.get_widget())
            }
        }

        // impl ::glib::traits::Connect for $gtk_struct {}

    );
)

// macro_rules! impl_connect(
//     ($gtk_struct:ident -> $($signal_name:ident),*) => (
//         $(impl<'a> ::glib::traits::Connect<::gtk::signals::$signal_name<'a>> for $gtk_struct {})*

//     )
// )

macro_rules! impl_connect(
    ($gtk_struct:ident -> $($signal_name:ident),*) => (
        $(impl<'a> ::glib::traits::Connect<'a, ::gtk::signals::$signal_name<'a>> for $gtk_struct {})*
    )
)

macro_rules! impl_GObjectFunctions(
    ($gtk_struct:ident, $ffi_type:ident) => (
        fn get_pointer(&self) -> *mut ffi::$ffi_type {
            self.pointer
        }

        fn wrap_pointer(pointer: *mut ffi::$ffi_type) -> $gtk_struct {
            $gtk_struct {
                pointer: pointer
            }
        }
    )
)

macro_rules! impl_drop(
    ($gtk_struct:ident) => ( impl_drop!($gtk_struct, GTK_WIDGET) );
    ($gtk_struct:ident, $cast_func:ident) => (
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
                    pointer: ::gtk::cast::$cast_func(pointer)
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

macro_rules! impl_widget_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> ButtonPressEvent, ButtonReleaseEvent, CanActivateAccel,
                                     ChildNotify, CompositedChanged, ConfigureEvent, DamageEvent,
                                     DeleteEvent, DestroyEvent, DirectionChanged, Draw, EnterNotifyEvent,
                                     Event, EventAfter, Focus, FocusInEvent, FocusOutEvent, GrabBrokenEvent,
                                     GrabFocus, GrabNotify, Hide, KeyPressEvent,
                                     KeyReleaseEvent, KeynavFailed, LeaveNotifyEvent, Map, MapEvent,
                                     MnemonicActivate, MotionNotifyEvent, MoveFocus,
                                     PropertyNotifyEvent, ProximityInEvent, ProximityOutEvent,
                                     QueryTooltip, Realize, ScreenChanged, ScrollEvent,
                                     Show, SizeAllocate,
                                     StateFlagsChanged, StyleUpdated,
                                     TouchEvent, WindowStateEvent)


        // not implemented:
        // DragBegin, DragDataDelete, DragDataGet, DragDataReceived, DragDrop
        // DragEnd, DragFailed, DragLeave, DragMotion, HierarchyChanged, ParentSet, PopuMenu,
        // UnRealize, VisibilityNotifyEvent, UnMap, UnMapEvent, StyleSet, StateChanged, SelectionRequestEvent,
        // SelectionReceived, SelectionClearEvent, SelectionGet, SelectionNotifyEvent,
    )
)

macro_rules! impl_button_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> Activate, Clicked, Enter, Leave, Pressed, Released)
    )
)

macro_rules! impl_range_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> AdjustBounds, MoveSlider, ValueChanged)

    // ChangeValue
    )
)
