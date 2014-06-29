
use gdk;
use libc::c_void;
use std::mem;

pub enum EventType{
  Nothing          = -1,
  Delete           = 0,
  Destroy          = 1,
  Expose           = 2,
  MotionNotify     = 3,
  ButtonPress      = 4,
  DoubleButtonPress = 5,
  TripleButtonPress = 6,
  ButtonRelease    = 7,
  KeyPress         = 8,
  KeyRelease       = 9,
  EnterNotify      = 10,
  LeaveNotify      = 11,
  FocusChange      = 12,
  Configure        = 13,
  Map              = 14,
  Unmap            = 15,
  PropertyNotify   = 16,
  SelectionClear   = 17,
  SelectionRequest = 18,
  SelectionNotify  = 19,
  ProximityIn      = 20,
  ProximityOut     = 21,
  DragEnter        = 22,
  DragLeave        = 23,
  DragMotion       = 24,
  DragStatus       = 25,
  DropStart        = 26,
  DropFinished     = 27,
  ClientEvent      = 28,
  VisibilityNotify = 29,
  Scroll           = 31,
  WindowState      = 32,
  Setting          = 33,
  OwnerChange      = 34,
  GrabBroken       = 35,
  Damage           = 36,
  TouchBegin       = 37,
  TouchUpdate      = 38,
  TouchEnd         = 39,
  TouchCancel      = 40
}

pub trait Event {
  fn get_send_event(&self) -> bool{
    unsafe{
      let event_any : &EventAny = mem::transmute(self);
      event_any.send_event == 0
    }
  }
}

pub struct EventAny{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

}

impl Event for EventAny {}

pub struct EventExpose{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub area : gdk::Rectangle,
  pub region : *mut c_void, //TODO cairo_region_t
  pub count : int, /* If non-zero, how many more events follow. */
}

impl Event for EventExpose {}

pub struct EventVisibility{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  state : gdk::VisibilityState,
}

impl Event for EventVisibility {}

pub struct EventMotion{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  time : u32,
  x : f64,
  y : f64,
  axes : *mut f64,
  state : uint,
  is_hint : i16,
  device : *mut gdk::Device,
  x_root : f64,
  y_root : f64,
}

impl Event for EventMotion {}

pub struct EventButton{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  time : u32,
  x : f64,
  y : f64,
  axes : *mut f64,
  state : uint,
  button : uint,
  device : *mut gdk::Device,
  x_root : f64,
  y_root : f64,
}

impl Event for EventButton {}

pub struct EventTouch{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub time : u32,
  pub x : f64,
  pub y : f64,
  pub axes : *mut f64,
  pub state : uint,
  pub sequence : *mut c_void, //gdk::EventSequence
  pub emulating_pointer : bool,
  pub device : *mut gdk::Device,
  pub x_root : f64,
  pub y_root : f64,
}

impl Event for EventTouch {}

pub struct EventScroll{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub time : u32,
  pub x : f64,
  pub y : f64,
  pub state : uint,
  pub direction : gdk::ScrollDirection,
  pub device : *mut gdk::Device,
  pub x_root : f64,
  pub y_root : f64,
  pub delta_x : f64,
  pub delta_y : f64,
}

impl Event for EventScroll {}

pub struct EventKey{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub time : u32,
  pub state : uint,
  pub keyval : uint,
  pub length : int,
  pub string : *mut char,
  pub hardware_keycode : u16,
  pub group : u8,
  pub is_modified: uint
}

impl Event for EventKey {}

pub struct EventCrossing{
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
  pub focus : bool,
  pub state : uint, //FIXME
}

impl Event for EventCrossing {}

pub struct EventFocus{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  _in : i16,
}

impl Event for EventFocus {}

pub struct EventConfigure{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub x : int,
  pub y : int,
  pub width : int,
  pub height : int,
}

impl Event for EventConfigure {}

pub struct EventProperty{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  atom : gdk::Atom,
  time : u32,
  state : uint, //FIXME
}

impl Event for EventProperty {}

pub struct EventSelection{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub selection : gdk::Atom,
  pub target : gdk::Atom,
  pub property : gdk::Atom,
  pub time : u32,
  pub requestor : *mut gdk::Window,
}

impl Event for EventSelection {}

pub struct EventOwnerChange{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub owner : *mut gdk::Window,
  pub reason : gdk::OwnerChange,
  pub selection : gdk::Atom,
  pub time : u32,
  pub selection_time : u32,
}

impl Event for EventOwnerChange {}

pub struct EventProximity{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub time : u32,
  pub device : *mut gdk::Device,
}

impl Event for EventProximity {}

pub struct EventSetting{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub action : gdk::SettingAction,
  pub name : *mut char,
}

impl Event for EventSetting {}

pub struct EventWindowState{
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub changed_mask : gdk::WindowState,
  pub new_window_state : gdk::WindowState,
}

impl Event for EventWindowState {}

pub struct EventGrabBroken {
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub keyboard : bool,
  pub implicit : bool,
  pub grab_window : *mut gdk::Window,
}

impl Event for EventGrabBroken  {}

pub struct EventDND {
  pub _type : gdk::EventType,
  pub window : *mut gdk::Window,
  send_event : i8,

  pub context : *mut c_void, //gdk::DragContext
  pub time : u32,
  pub x_root : i16, //short
  pub y_root : i16, //short
}

impl Event for EventDND  {}


//Supporting types

pub enum VisibilityState{
  VisibilityUnobscured,
  VisibilityPartial,
  VisibilityFullyObscured
}

pub enum ScrollDirection{
  ScrollUp,
  ScrollDown,
  ScrollLeft,
  ScrollRight,
  ScrollSmooth
}

pub enum NotifyType{
  NotifyAncestor   = 0,
  NotifyVirtual    = 1,
  NotifyInferior   = 2,
  NotifyNonlinear  = 3,
  NotifyNonlinearVirtual  = 4,
  NotifyUnknown    = 5
}

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

pub enum PropertyState{
  PropertyNewValue,
  PropertyDelete
}

pub enum WindowState{
  WindowStateWithdrawn  = 1 << 0,
  WindowStateIconified  = 1 << 1,
  WindowStateMaximized  = 1 << 2,
  WindowStateSticky     = 1 << 3,
  WindowStateFullscreen = 1 << 4,
  WindowStateAbove      = 1 << 5,
  WindowStateBelow      = 1 << 6,
  WindowStateFocused    = 1 << 7,
  WindowStateTiled      = 1 << 8
}

pub enum SettingAction{
  SettingActionNew,
  SettingActionChanged,
  SettingActionDeleted
}

pub enum OwnerChange{
  OwnerChangeNewOwner,
  OwnerChangeDestroy,
  OwnerChangeClose
}