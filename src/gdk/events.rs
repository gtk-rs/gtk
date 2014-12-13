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

use gdk;
use libc::c_void;
use std::mem;

pub use self::event_type::EventType;
pub use self::owner_change::OwnerChange;
pub use self::setting_action::SettingAction;
pub use self::property_state::PropertyState;
pub use self::crossing_mode::CrossingMode;
pub use self::notify_type::NotifyType;
pub use self::scroll_direction::ScrollDirection;
pub use self::visibility_state::VisibilityState;

pub mod event_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum EventType {
        Nothing           = -1,
        Delete            = 0,
        Destroy           = 1,
        Expose            = 2,
        MotionNotify      = 3,
        ButtonPress       = 4,
        DoubleButtonPress = 5,
        TripleButtonPress = 6,
        ButtonRelease     = 7,
        KeyPress          = 8,
        KeyRelease        = 9,
        EnterNotify       = 10,
        LeaveNotify       = 11,
        FocusChange       = 12,
        Configure         = 13,
        Map               = 14,
        Unmap             = 15,
        PropertyNotify    = 16,
        SelectionClear    = 17,
        SelectionRequest  = 18,
        SelectionNotify   = 19,
        ProximityIn       = 20,
        ProximityOut      = 21,
        DragEnter         = 22,
        DragLeave         = 23,
        DragMotion        = 24,
        DragStatus        = 25,
        DropStart         = 26,
        DropFinished      = 27,
        ClientEvent       = 28,
        VisibilityNotify  = 29,
        Scroll            = 31,
        WindowState       = 32,
        Setting           = 33,
        OwnerChange       = 34,
        GrabBroken        = 35,
        Damage            = 36,
        TouchBegin        = 37,
        TouchUpdate       = 38,
        TouchEnd          = 39,
        TouchCancel       = 40
    }
}

pub trait Event {
    fn get_send_event(&self) -> bool {
        unsafe {
            let event_any : &EventAny = mem::transmute(self);
            event_any.send_event == 0
        }
    }
}

#[repr(C)]
#[deriving(Copy)]
pub struct EventAny {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,
}

impl Event for EventAny {}

#[deriving(Copy)]
pub struct EventExpose {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub area : gdk::Rectangle,
    pub region : *mut c_void, //TODO cairo_region_t
    pub count : i8 /* If non-zero, how many more events follow. */
}

impl Event for EventExpose {}

#[deriving(Copy)]
pub struct EventVisibility{
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    state : gdk::VisibilityState
}

impl Event for EventVisibility {}

#[deriving(Copy)]
pub struct EventMotion {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    time : u32,
    x : f64,
    y : f64,
    axes : *mut f64,
    state : gdk::enums::modifier_type::ModifierType,
    is_hint : i16,
    device : *mut gdk::Device,
    x_root : f64,
    y_root : f64
}

impl Event for EventMotion {}

#[deriving(Copy)]
pub struct EventButton {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    time : u32,
    x : f64,
    y : f64,
    axes : *mut f64,
    state : gdk::enums::modifier_type::ModifierType,
    button : u32,
    device : *mut gdk::Device,
    x_root : f64,
    y_root : f64
}

impl Event for EventButton {}

#[deriving(Copy)]
pub struct EventTouch {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub axes : *mut f64,
    pub state : gdk::enums::modifier_type::ModifierType,
    pub sequence : *mut c_void, //gdk::EventSequence
    pub emulating_pointer : i32, // boolean
    pub device : *mut gdk::Device,
    pub x_root : f64,
    pub y_root : f64
}

impl Event for EventTouch {}

#[deriving(Copy)]
pub struct EventScroll {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub state : gdk::enums::modifier_type::ModifierType,
    pub direction : gdk::ScrollDirection,
    pub device : *mut gdk::Device,
    pub x_root : f64,
    pub y_root : f64,
    pub delta_x : f64,
    pub delta_y : f64
}

impl Event for EventScroll {}

#[deriving(Copy)]
pub struct EventKey {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub time : u32,
    pub state : gdk::enums::modifier_type::ModifierType,
    pub keyval : u32,
    pub length : i32,
    pub string : *mut char,
    pub hardware_keycode : u16,
    pub group : u8,
    pub is_modifier: u32
}

impl Event for EventKey {}

#[deriving(Copy)]
pub struct EventCrossing {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub subwindow : gdk::Window,
    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub x_root : f64,
    pub y_root : f64,
    pub mode : gdk::CrossingMode,
    pub detail : gdk::NotifyType,
    pub focus : i32, // boolean
    pub state : gdk::enums::modifier_type::ModifierType
}

impl Event for EventCrossing {}

#[deriving(Copy)]
pub struct EventFocus {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    _in : i16
}

impl Event for EventFocus {}

#[deriving(Copy)]
pub struct EventConfigure {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32
}

impl Event for EventConfigure {}

#[deriving(Copy)]
pub struct EventProperty {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    atom : gdk::Atom,
    time : u32,
    state : u32 //FIXME
}

impl Event for EventProperty {}

#[deriving(Copy)]
pub struct EventSelection {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub selection : gdk::Atom,
    pub target : gdk::Atom,
    pub property : gdk::Atom,
    pub time : u32,
    pub requestor : *mut gdk::Window
}

impl Event for EventSelection {}

#[deriving(Copy)]
pub struct EventOwnerChange {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub owner : *mut gdk::Window,
    pub reason : gdk::OwnerChange,
    pub selection : gdk::Atom,
    pub time : u32,
    pub selection_time : u32
}

impl Event for EventOwnerChange {}

#[deriving(Copy)]
pub struct EventProximity {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub time : u32,
    pub device : *mut gdk::Device
}

impl Event for EventProximity {}

#[deriving(Copy)]
pub struct EventSetting {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub action : gdk::SettingAction,
    pub name : *mut char
}

impl Event for EventSetting {}

#[deriving(Copy)]
pub struct EventWindowState {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub changed_mask : gdk::WindowState,
    pub new_window_state : gdk::WindowState
}

impl Event for EventWindowState {}

#[deriving(Copy)]
pub struct EventGrabBroken {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub keyboard : i32, // boolean
    pub implicit : i32, // boolean
    pub grab_window : *mut gdk::Window
}

impl Event for EventGrabBroken  {}

#[deriving(Copy)]
pub struct EventDND {
    pub _type : gdk::EventType,
    pub window : *mut gdk::Window,
    send_event : i8,

    pub context : *mut c_void, //gdk::DragContext
    pub time : u32,
    pub x_root : i16, //short
    pub y_root : i16  //short
}

impl Event for EventDND  {}


//Supporting types

pub mod visibility_state {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum VisibilityState{
        VisibilityUnobscured,
        VisibilityPartial,
        VisibilityFullyObscured
    }
}

pub mod scroll_direction {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum ScrollDirection{
        ScrollUp,
        ScrollDown,
        ScrollLeft,
        ScrollRight,
        ScrollSmooth
    }
}

pub mod notify_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum NotifyType{
        NotifyAncestor   = 0,
        NotifyVirtual    = 1,
        NotifyInferior   = 2,
        NotifyNonlinear  = 3,
        NotifyNonlinearVirtual  = 4,
        NotifyUnknown    = 5
    }
}

pub mod crossing_mode {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum CrossingMode{
        CrossingNormal,
        CrossingGrab,
        CrossingUngrab,
        CrossingGtkGrab,
        CrossingGtkUngrab,
        CrossingStateChanged,
        CrossingTouchBegin,
        CrossingTouchEnd,
        CrossingDeviceSwitch
    }
}

pub mod property_state {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum PropertyState{
        PropertyNewValue,
        PropertyDelete
    }
}

pub mod setting_action {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum SettingAction{
        SettingActionNew,
        SettingActionChanged,
        SettingActionDeleted
    }
}

pub mod owner_change {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
    pub enum OwnerChange{
        OwnerChangeNewOwner,
        OwnerChangeDestroy,
        OwnerChangeClose
    }
}
