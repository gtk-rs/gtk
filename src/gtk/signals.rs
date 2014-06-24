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

//#![feature(macro_rules)]
//#![macro_escape]

#![allow(unused_imports)]
#![allow(unused_variable)]

use std::mem::transmute;
use libc::c_void;
use gtk::ffi;
use gdk;
use glib;
use gtk;

pub trait Signal<'a>{
    fn get_signal_name(&self) -> &str;

    fn get_trampoline(&self) -> extern "C" fn();

    fn fetch_cb(&self) -> *||;
}

// The defintion of the signal macro is split in a argumentless and
// one-with-arguments case because of a limitation in the rust macro
// system.
macro_rules! signal(
    //Signals without arguments
    ($signal:ident, $class:ident () -> $ret_type:ty) => (

        //General case (see below)
        signal!($signal, $class [] -> $ret_type)

        mod $signal{
            use std::mem::transmute;
            use libc::c_void;
            use gtk::ffi;
            use gdk;
            use glib;
            use gtk;

            pub extern fn trampoline(widget : *ffi::C_GtkWidget, user_data : *Box<super::Signal>) -> $ret_type{
                unsafe{
                    let ref signal = *user_data;
                    let cb : *|| -> $ret_type = transmute(signal.fetch_cb());
                    (*cb)()
                }
            }
        }
    );

    //Signals with arguments
    ($signal:ident, $class:ident ( $($arg_name:ident : $arg_type:ty),* ) -> $ret_type:ty) => (
        //General case (see below)
        signal!($signal, $class [$(($arg_name : $arg_type)),*] -> $ret_type)

        mod $signal{
            use std::mem::transmute;
            use libc::c_void;
            use gtk::ffi;
            use gdk;
            use glib;
            use gtk;

            pub extern fn trampoline(widget : *ffi::C_GtkWidget, $($arg_name : $arg_type),* , user_data : *Box<super::Signal>) -> $ret_type{
                unsafe{
                    let ref signal = *user_data;
                    let cb_raw = signal.fetch_cb();
                    let cb : *|$($arg_type),*| -> $ret_type = transmute(cb_raw);
                    (*cb)($($arg_name),*)
                }
            }
        }
    );

    //TODO custom trampoline
    ($signal:ident, $class:ident ( $($arg_name:ident : $arg_type:ty),* ) -> $ret_type:ty,
                    trampoline   ( $($t_arg_nm:ident : $t_arg_ty:ty),* ) -> $t_ret_ty:ty $t_blck:block) => (

        //General case (see below)
        signal!($signal, $class [$(($arg_name : $arg_type)),*] -> $ret_type)

        mod $signal{
            #[allow(unused_imports)]
            #[allow(unused_variable)]

            use std::mem::transmute;
            use libc::c_void;
            use gtk::ffi;
            use gdk;
            use glib;
            use gtk;

            pub extern fn trampoline(widget : *ffi::C_GtkWidget, $($t_arg_nm : $t_arg_ty),* , user_data : *Box<super::Signal>) -> $t_ret_ty{
                unsafe{
                    let ref signal = *user_data;
                    let cb : *|$($arg_type),*| -> $ret_type = transmute(signal.fetch_cb());
                    let out = $blck
                    out
                }
            }
        }
    );

    //General case
    ($signal:ident, $class:ident [ $(($arg_name:ident : $arg_type:ty)),* ] -> $ret_type:ty) => (
        pub struct $class<'a>{
            pub cb: |$($arg_type),*|:'a -> $ret_type
        }

        impl<'a> $class<'a>{
            pub fn new (cb : |$($arg_type),*|:'a -> $ret_type) -> Box<Signal<'a>> {
                box $class{
                    cb: cb
                } as Box<Signal<'a>>
            }
        }

        impl<'a> Signal<'a> for $class<'a>{
            fn get_signal_name(&self) -> &str {
                stringify!($signal)
            }

            fn get_trampoline(&self) -> extern "C" fn(){
                unsafe{
                    transmute(self::$signal::trampoline)
                }
            }

            fn fetch_cb(&self) -> *||{
                unsafe{
                    transmute(&self.cb)
                }
            }
        }
    );
)

//GObject
//https://developer.gnome.org/gobject/unstable/gobject-The-Base-Object-Type.html#gobject-The-Base-Object-Type.signals
signal!(notify,	Notify(g_param_spec:c_void) -> ())

//GtkWidget
//https://developer.gnome.org/gtk3/stable/GtkWidget.html#GtkWidget.signals

signal!(accel_closures_changed, AccelClosuresChanged(spec : glib::ParamSpec) -> ())
signal!(can_activate_accel,     CanActivateAccel(signal_id:uint) -> bool)
signal!(child_notify,           ChildNotify(spec : glib::ParamSpec) -> ())
signal!(composited_changed,     CompositedChanged() -> ())
signal!(destroy,                Destroy() -> ())
signal!(direction_changed,      DirectionChanged(previous_direction: gtk::TextDirection) -> ())
signal!(draw,                   Draw(cairo_context : *c_void) -> ()) //TODO
signal!(focus,                  Focus(direction : gtk::DirectionType) -> bool)
signal!(grab_focus,             GrabFocus() -> ())
signal!(grab_notify,            GrabNotify(was_grabbed : bool) -> ())
signal!(hide,                   Hide() -> ())
signal!(keynav_failed,          KeynavFailed(direction : gtk::DirectionType) -> bool)
signal!(map,                    Map() -> ())
signal!(mnemonic_activate,      MnemonicActivate(arg : bool) -> bool)
signal!(move_focus,             MoveFocus(direction : gtk::DirectionType) -> ())
signal!(popup_menu,             PopupMenu() -> bool)
signal!(query_tooltip,          QueryTooltip(x:int, y:int, keyboard_mode:bool, tooltip : *gtk::Tooltip) -> bool)
signal!(realize,                Realize() -> ())
signal!(screen_changed,         ScreenChanged(previous_screen : *gdk::Screen) -> ())
signal!(show,                   Show() -> ())
signal!(show_help,              ShowHelp(help_type : gtk::WidgetHelpType) -> bool)
signal!(size_allocate,          SizeAllocate(allocation : *gdk::Rectangle) -> ())
signal!(state_chagned,          StateChagned(state : gtk::StateType) -> ())
signal!(state_flags_changed,    StateFlagsChanged(flags : gtk::StateFlags) -> ())
signal!(style_updated,          StyleUpdated() -> ())
signal!(unmap,                  Unmap() -> ())
signal!(unrealize,              Unrealize() -> ())

/*
signal!(hierarchy_changed, HierarchyChanged(previous_toplevel : *gtk::Widget) -> (),
    trampoline (previous_toplevel : *ffi::C_GtkWidget) -> Box<gtk::Widget> {
        cb(previous_toplevel)
    }
)
signal!(parent_set,    ParentSet(old_parent : *gtk::Widget) -> ())
*/

//GtkWidget: GDK events
signal!(button_press_event,     ButtonPressEvent(event : *gdk::EventButton) -> bool)
signal!(button_release_event,   ButtonReleaseEvent(event : *gdk::EventButton) -> bool)
signal!(configure_event,        ConfigureEvent(event : *gdk::EventConfigure) -> bool)
signal!(damage_event,           DamageEvent(event : *gdk::EventExpose) -> bool)
signal!(delete_event,           DeleteEvent(event : *gdk::EventAny) -> bool)
signal!(destroy_event,          DestroyEvent(event : *gdk::EventAny) -> bool)
signal!(enter_notify_event,     EnterNotifyEvent(event : *gdk::EventCrossing) -> bool)
signal!(leave_notify_event,     LeaveNotifyEvent(event : *gdk::EventCrossing) -> bool)
signal!(event,                  Event(event : *gdk::EventAny) -> bool)
signal!(event_after,            EventAfter(event : *gdk::EventAny) -> bool)
signal!(focus_in_event,         FocusInEvent(event : *gdk::EventFocus) -> bool)
signal!(focus_out_event,        FocusOutEvent(event : *gdk::EventFocus) -> bool)
signal!(grab_broken_event,      GrabBrokenEvent(event : *gdk::EventGrabBroken) -> bool)
signal!(key_press_event,        KeyPressEvent(event : *gdk::EventKey) -> bool)
signal!(key_release_event,      KeyReleaseEvent(event : *gdk::EventKey) -> bool)
signal!(map_event,              MapEvent(event : *gdk::EventAny) -> bool)
signal!(motion_notify_event,    MotionNotifyEvent(event : *gdk::EventMotion) -> bool)
signal!(property_notify_event,  PropertyNotifyEvent(event : *gdk::EventProperty) -> bool)
signal!(proximity_in_event,     ProximityInEvent(event : *gdk::EventProximity) -> bool)
signal!(proximity_out_event,    ProximityOutEvent(event : *gdk::EventProximity) -> bool)
signal!(scroll_event,           ScrollEvent(event : *gdk::EventScroll) -> bool)
signal!(touch_event,            TouchEvent(event : *gdk::EventTouch) -> bool)
signal!(unmap_event,            UnmapEvent(event : *gdk::EventAny) -> bool)
signal!(window_state_event,     WindowStateEvent(event : *gdk::EventWindowState) -> bool)

//GtkWidget: Drag-drop
/*
signal!(drag_begin,         DragBegin(context : *gdk::DragContext) -> ())
signal!(drag_data_delete,   DragDataDelete(context : *gdk::DragContext) -> ())
signal!(drag_data_get,      DragDataGet(context : *gdk::DragContext,
                                        data : *gtk::SelectionData,
                                        info: uint,
                                        time: uint) -> ())
signal!(drag_data_received, DragDataReceived(context : *gdk::DragContext,
                                             x: int,
                                             y: int,
                                             data: *gtk::SelectionData,
                                             info: uint,
                                             time: uint) -> ())
signal!(drag_drop,          DragDrop(context : *gdk::DragContext,
                                     x: int,
                                     y: int,
                                     time: uint) -> ())
signal!(drag_end,           DragEnd(context : *gdk::DragContext) -> ())
signal!(drag_failed,        DragFailed(context : *gdk::DragContext, result: gtk::DragResult) -> ())
signal!(drag_leave,         DragLeave(context : *gdk::DragContext, time : uint) -> ())
signal!(drag_motion,        DragMotion(context : *gdk::DragContext,
                                       x : int,
                                       y : int,
                                       time : uint) -> bool)

//GtkWidget: Selection
signal!(selection_get,          SelectionGet(data : *gtk::SelectionData, info: uint, time: uint) -> ())
signal!(selection_received,     SelectionReceived(data : *gtk::SelectionData, time: uint) -> ())

signal!(selection_clear_event,  SelectionClearEvent(event : *gdk::EventSelection) -> bool)
signal!(selection_request_event,SelectionRequestEvent(event : *gdk::EventSelection) -> bool)
signal!(selection_notify_event, SelectionNotifyEvent(event : *gdk::EventSelection) -> bool)
*/

//GtkContainer
//signal!(add,                Add(widget: *gtk::Widget) -> ())
signal!(check_resize,       CheckResize() -> ())
//signal!(remove,             Remove(widget: *gtk::Widget) -> ())
//signal!(set_focus_child,    SetFocusChild(widget: *gtk::Widget) -> ())

//GtkButton
signal!(activate,           Activate() -> ())
signal!(clicked,            Clicked() -> ())

//GtkDialog
signal!(response,           Response(response_id : int) -> ())